// {"login_url":"https://ai.imjoy.io/public/apps/hypha-login/?key=mihDumpHGYxkPdSEKB7GgM","key":"mihDumpHGYxkPdSEKB7GgM","report_url":"https://ai.imjoy.io/public/services/hypha-login/report"}

use std::fmt::Display;

use bioimg_spec::rdf::HttpUrl;

#[derive(thiserror::Error, Debug)]
pub enum HyphaLoginError{
    #[error("{0}")]
    RequestFailed(ehttp::Error),
    #[error("{0}")]
    SerializationError(#[from] serde_json::Error),
    #[error(transparent)]
    LoginFailed(#[from] LoginCheckError)
}

impl From<ehttp::Error> for HyphaLoginError{
    fn from(value: ehttp::Error) -> Self {
        Self::RequestFailed(value)
    }
}

#[derive(thiserror::Error, serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginCheckError{
    detail: String
}

impl Display for LoginCheckError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Login to hypha failed: {}", self.detail)
    }
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Seconds(pub u32);

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserToken(String);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct LoginStartData{
    pub login_url: HttpUrl,
    pub key: String,
    pub report_url: HttpUrl,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct HyphaLoginCheckParams{
    pub key: String,
    pub timeout: Seconds,
}

pub struct HyphaClient{
}

impl HyphaClient{
    pub fn start_auth<F>(on_done: F)
    where
        F: FnOnce(Result<LoginStartData, HyphaLoginError>) + Send + 'static
    {
        let request = ehttp::Request{
            method: "POST".into(),
            url: "https://ai.imjoy.io/public/services/hypha-login/start".to_owned(),
            headers: ehttp::Headers {
                headers: vec![("Content-Type".to_owned(), "application/json".to_owned())]
            },
            body: "{}".as_bytes().into(),
        };
        ehttp::fetch(request, |result|{
            let final_result = || -> Result<LoginStartData, HyphaLoginError>{
                let ok_result = result?;
                let raw_text = ok_result.text().ok_or_else(|| "Got no response from server".to_owned())?;
                println!("Here's the resopnse text: {raw_text}");
                let login_data: LoginStartData = serde_json::from_str(raw_text)?;
                Ok(login_data)
            }();
            on_done(final_result)
        });
    }

    pub fn fetch_token<F>(start_data: &LoginStartData, on_done: F)
    where
        F: FnOnce(Result<UserToken, HyphaLoginError>) + Send + 'static
    {
        let body_payload = HyphaLoginCheckParams{
            key: start_data.key.clone(),
            timeout: Seconds(1)
        };

        let request = ehttp::Request {
            method: "POST".into(),
            url: "https://ai.imjoy.io/public/services/hypha-login/check".into(),
            headers: ehttp::Headers{ headers: vec![("Content-Type".into(), "application/json".into())] },
            body: serde_json::to_string(&body_payload).unwrap().into()
        };
        ehttp::fetch(request, |result|{
            let final_result = || -> Result<UserToken, HyphaLoginError>{
                let ok_result = result?;
                let raw_text = ok_result.text().ok_or_else(|| "Got no response from server".to_owned())?;
                let token_result: Result<UserToken, LoginCheckError> = serde_json::from_str(raw_text)?;
                Ok(token_result?)
            }();
            on_done(final_result)
        });
    }
}

#[test]
fn test_login(){
    use std::sync::{Arc, Mutex, Condvar};

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    HyphaClient::start_auth(move |result|{
        match result{
            Err(err) => {
                panic!("I guess request failed: {err:?}")
            },
            Ok(val) => println!("login here: {}", val.login_url)
        }


        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });

    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }
    println!("there??????????");
}