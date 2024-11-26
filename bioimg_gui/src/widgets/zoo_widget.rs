use std::sync::Arc;

use bioimg_spec::rdf::HttpUrl;
use bioimg_zoo::auth::{AuthInProgress, AuthNeedsBrowserLogin, AuthStart, Seconds, UserToken};

use crate::result::GuiError;

use super::{error_display::show_error, StatefulWidget};

type BytesResponse = http::Response<Vec<u8>>;

type ReqResult = Result<BytesResponse, String>;

fn send_bytes<T: AsRef<[u8]>>(req: http::Request<T>) -> Result<http::Response<Vec<u8>>, String>{
    let (http_parts, body) = req.into_parts();
    let reader_req = http::Request::from_parts(http_parts, std::io::Cursor::new(body));
    send_reader(reader_req)
}

fn send_reader<R: std::io::Read>(req: http::Request<R>) -> Result<http::Response<Vec<u8>>, String>{
    eprintln!("Requesting {}", req.uri().to_string());
    let (http_parts, body) = req.into_parts();
    let request: ureq::Request = http_parts.into();
    let resp: http::Response<Vec<u8>> = request.send(body)
        .map_err(|err| {
            eprintln!("Ok, so something went wrong!!!!!!!!!!!!!!!!!!!!!!!");
            let ureq::Error::Status(status, resp) = err else {
                eprintln!(">>>>>>> iit was transport error");
                return "Some transport error".to_owned();
            };
            let mut reader = resp.into_reader();
            let mut buf = vec![];
            reader.read_to_end(&mut buf).unwrap();
            let s = String::from_utf8_lossy(&buf);
            eprintln!(">>>>>>> iit was something else {status}: {s}");
            s.to_string()
        })?
        .into();
    if !resp.status().is_success(){
        let payload = String::from_utf8_lossy(resp.body());
        println!("Error!!\n{}", payload);
        return Err(format!("Request failed with result {}\n{payload}", resp.status()))
    }
    Ok(resp)
}


enum ZooLoginState{
    Start(AuthStart),
    FetchingLoginUrl{state: AuthStart, request_task: std::thread::JoinHandle<ReqResult>},
    NeedsBrowserLogin(AuthNeedsBrowserLogin),
    AuthInProgress{login_url: HttpUrl, state: AuthInProgress, request_task: std::thread::JoinHandle<ReqResult>},
    Authenticated(Arc<UserToken>),

    Failed(GuiError)
}

impl Default for ZooLoginState{
    fn default() -> Self {
        Self::Start(Default::default())
    }
}

impl ZooLoginState{
    pub fn fetching_login_url() -> Self{
        let start = AuthStart::new();
        let req: http::Request<_>  = start.as_ref().clone();
        ZooLoginState::FetchingLoginUrl{
            request_task: std::thread::spawn(move || send_bytes(req)),
            state: start,
        }
    }
}

#[derive(Default)]
pub struct ZooLoginWidget{
    state: ZooLoginState,
}

impl ZooLoginWidget{
    pub fn update(&mut self){
        self.state = match std::mem::take(&mut self.state){
            ZooLoginState::FetchingLoginUrl{state, request_task} => 'advancing_start: {
                if !request_task.is_finished(){
                    break 'advancing_start ZooLoginState::FetchingLoginUrl{state, request_task}
                }
                let response = match request_task.join().unwrap(){ //FIXME: report failure to join?
                    Ok(resp) => resp,
                    Err(reason) => break 'advancing_start ZooLoginState::Failed(GuiError::new(reason.to_string()))
                };
                match state.try_advance(&response){
                    Ok(new_state) => ZooLoginState::NeedsBrowserLogin(new_state),
                    Err((_current_state, reason)) => ZooLoginState::Failed(GuiError::new(reason.to_string()))
                }
            },
            ZooLoginState::AuthInProgress { login_url, state, request_task } => 'fetching_token: {
                if ! request_task.is_finished(){
                    break 'fetching_token ZooLoginState::AuthInProgress { login_url, state, request_task }
                }
                let response = match request_task.join().unwrap(){ //FIXME: report failure to join?
                    Ok(resp) => resp,
                    Err(reason) => {
                        break 'fetching_token ZooLoginState::Failed(GuiError::new(reason.to_string()))
                    }
                };
                match state.try_advance(&response){
                    Ok(user_token) => ZooLoginState::Authenticated(Arc::new(user_token)),
                    Err((_current_state, reason)) => ZooLoginState::Failed(GuiError::new(reason.to_string())),
                }
            },
            state => state,
        }
    }
}

impl StatefulWidget for ZooLoginWidget{
    type Value<'p> = Result<Arc<UserToken>, GuiError>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        self.update();
        ui.vertical(|ui|{
            self.state = match std::mem::take(&mut self.state){
                ZooLoginState::Start(state) => {
                    if ui.button("start login").clicked(){
                        ZooLoginState::fetching_login_url()
                    }else{
                        ZooLoginState::Start(state)
                    }
                },
                ZooLoginState::Failed(state) => {
                    let clicked = ui.horizontal(|ui|{
                        let clicked = ui.button("start login").clicked();
                        ui.label(egui::RichText::new("login failed").color(egui::Color32::RED))
                            .on_hover_ui(|ui|{
                                ui.label(state.to_string());
                            });
                        clicked
                    }).inner;
                    if clicked{
                        ZooLoginState::fetching_login_url()
                    }else{
                        ZooLoginState::Failed(state)
                    }
                },
                ZooLoginState::FetchingLoginUrl{state, request_task} => {
                    ui.horizontal(|ui|{
                        ui.add_enabled_ui(false, |ui| ui.button("start login"));
                        ui.weak("Requesting login URL...");
                    });
                    ui.ctx().request_repaint();
                    ZooLoginState::FetchingLoginUrl{state, request_task}
                },
                ZooLoginState::NeedsBrowserLogin(state) => 'needs_browser_login: {
                    let link_clicked = ui.horizontal(|ui|{
                        ui.add_enabled_ui(false, |ui| ui.button("start login"));
                        ui.weak("Please login here: ");
                        eprintln!("{}", state.login_url());
                        ui.hyperlink(state.login_url()).clicked()
                    }).inner;
                    if !link_clicked{
                        break 'needs_browser_login ZooLoginState::NeedsBrowserLogin(state)
                    }
                    let (login_url, auth_in_progress) = state.advance(Seconds(3600));
                    let req: http::Request<_> = auth_in_progress.as_ref().clone();
                    ZooLoginState::AuthInProgress {
                        login_url,
                        state: auth_in_progress,
                        request_task: std::thread::spawn(move ||{
                            send_bytes(req)
                        })
                    }
                },
                ZooLoginState::AuthInProgress { login_url, state, request_task } => {
                    ui.horizontal(|ui|{
                        ui.add_enabled_ui(false, |ui| ui.button("start login"));
                        ui.weak("Waiting for login / fetching token...");
                    });
                    ui.ctx().request_repaint();
                    ZooLoginState::AuthInProgress { login_url, state, request_task }
                },
                ZooLoginState::Authenticated(user_token) => 'authenticated: {
                    let restart_login_clicked = ui.horizontal(|ui|{
                        let clicked = ui.button("restart login").clicked();
                        ui.weak("Login successful");
                        clicked
                    }).inner;
                    if restart_login_clicked{
                        ui.ctx().request_repaint();
                        break 'authenticated ZooLoginState::fetching_login_url();
                    }
                    ZooLoginState::Authenticated(user_token)
                },
            };
            ui.separator();
            match self.state(){
                Ok(user_token) => {
                    ui.add_enabled_ui(true, |ui|{
                        ui.button("⬆ Upload model to Zoo");
                    });
                },
                Err(_) => {
                    ui.add_enabled_ui(false, |ui|{
                        ui.button("⬆ Upload model to Zoo").on_hover_ui(|ui|{
                            ui.label("Please login first");
                        });
                    });
                },
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match &self.state{
            ZooLoginState::Authenticated(token) => Ok(Arc::clone(token)),
            ZooLoginState::Failed(reason) => Err(reason.clone()),
            _ => Err(GuiError::new("Auth not ready yet")), //FIXME:
        }
    }
}
