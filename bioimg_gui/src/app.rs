use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::thread::JoinHandle;

use bioimg_zoo::collection::ZooNickname;
use indoc::indoc;

use bioimg_runtime as rt;
use bioimg_runtime::zoo_model::{ModelPackingError, ZooModel};
use bioimg_spec::rdf::{self, ResourceId, ResourceName};
use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::non_empty_list::NonEmptyList;

use crate::project_data::{AppStateRawData, ProjectLoadError};
use crate::result::{GuiError, Result, VecResultExt};
use crate::widgets::attachments_widget::AttachmentsWidget;

use crate::widgets::code_editor_widget::MarkdwownLang;
use crate::widgets::collapsible_widget::CollapsibleWidget;
use crate::widgets::cover_image_widget::CoverImageItemConf;
// use crate::widgets::cover_image_widget::CoverImageWidget;
use crate::widgets::icon_widget::IconWidgetValue;
use crate::widgets::image_widget_2::SpecialImageWidget;
use crate::widgets::json_editor_widget::JsonObjectEditorWidget;
use crate::widgets::model_interface_widget::ModelInterfaceWidget;
use crate::widgets::model_links_widget::ModelLinksWidget;
use crate::widgets::notice_widget::NotificationsWidget;
use crate::widgets::search_and_pick_widget::SearchAndPickWidget;
use crate::widgets::staging_opt::StagingOpt;
use crate::widgets::staging_string::{InputLines, StagingString};
use crate::widgets::staging_vec::StagingVec;
use crate::widgets::util::TaskChannel;
use crate::widgets::version_widget::VersionWidget;
use crate::widgets::weights_widget::WeightsWidget;
use crate::widgets::zoo_widget::{upload_model, ZooLoginWidget};
use crate::widgets::ValueWidget;
use crate::widgets::Restore;
use crate::widgets::{
    author_widget::AuthorWidget, cite_widget::CiteEntryWidget, code_editor_widget::CodeEditorWidget,
    icon_widget::IconWidget, maintainer_widget::MaintainerWidget, url_widget::StagingUrl,
    util::group_frame, StatefulWidget,
};

#[derive(Default)]
enum PackingStatus {
    #[default]
    Done,
    Packing {
        path: PathBuf,
        task: poll_promise::Promise<Result<(), ModelPackingError>>,
    },
}

pub enum TaskResult{
    Notification(Result<String, String>),
    ModelImport(Box<rt::zoo_model::ZooModel>),
}

#[derive(Restore)]
pub struct AppState1 {
    pub staging_name: StagingString<ResourceName>,
    pub staging_description: StagingString<BoundedString<0, 1024>>,
    pub cover_images: StagingVec<SpecialImageWidget<rt::CoverImage>, CoverImageItemConf>,
    pub model_id_widget: StagingOpt<StagingString<ResourceId>>,
    pub staging_authors: StagingVec<CollapsibleWidget<AuthorWidget>>,
    pub attachments_widget: StagingVec<CollapsibleWidget<AttachmentsWidget>>,
    pub staging_citations: StagingVec<CollapsibleWidget<CiteEntryWidget>>,
    pub custom_config_widget: StagingOpt<JsonObjectEditorWidget>, //FIXME
    pub staging_git_repo: StagingOpt<StagingUrl>,
    pub icon_widget: StagingOpt<IconWidget>,
    pub links_widget: ModelLinksWidget,
    pub staging_maintainers: StagingVec<CollapsibleWidget<MaintainerWidget>>,
    pub staging_tags: StagingVec<StagingString<rdf::Tag>>,
    pub staging_version: StagingOpt<VersionWidget>,

    pub staging_documentation: CodeEditorWidget<MarkdwownLang>,
    pub staging_license: SearchAndPickWidget<rdf::LicenseId>,
    //badges
    pub model_interface_widget: ModelInterfaceWidget,
    ////
    pub weights_widget: WeightsWidget,

    #[restore_default]
    pub zoo_login_widget: ZooLoginWidget,
    #[restore_default]
    pub zoo_model_creation_task: Option<JoinHandle<Result<ZooNickname>>>,

    #[restore_default]
    pub notifications_widget: NotificationsWidget,
    #[restore_default]
    pub notifications_channel: TaskChannel<TaskResult>,
    #[restore_default]
    model_packing_status: PackingStatus,
    #[restore_default]
    close_confirmed: bool,
    #[restore_default]
    show_confirmation_dialog: bool,
}

impl ValueWidget for AppState1{
    type Value<'v> = rt::zoo_model::ZooModel;

    fn set_value<'v>(&mut self, zoo_model: Self::Value<'v>) {
        self.staging_name.set_value(zoo_model.name);
        self.staging_description.set_value(zoo_model.description);
        self.cover_images.set_value(
            zoo_model.covers.into_iter()
                .map(|cover| (None, Some(cover)))
                .collect()
        );
        self.model_id_widget.set_value(zoo_model.id);
        self.staging_authors.set_value(zoo_model.authors.into_inner());
        self.attachments_widget.set_value(zoo_model.attachments);
        self.staging_citations.set_value(zoo_model.cite.into_inner());
        self.custom_config_widget.set_value(
            if zoo_model.config.is_empty(){
                None
            } else {
                Some(zoo_model.config)
            }
        );
        self.staging_git_repo.set_value(zoo_model.git_repo.map(|val| Arc::new(val)));
        self.icon_widget.set_value(zoo_model.icon.map(IconWidgetValue::from));
        self.links_widget.set_value(zoo_model.links);
        self.staging_maintainers.set_value(zoo_model.maintainers);
        self.staging_tags.set_value(zoo_model.tags);
        self.staging_version.set_value(zoo_model.version);
        self.staging_documentation.set_value(&zoo_model.documentation);
        self.staging_license.set_value(zoo_model.license);

        self.model_interface_widget.set_value(zoo_model.interface);

        self.weights_widget.set_value(zoo_model.weights);

        self.model_packing_status = PackingStatus::default();
    }
}

impl Default for AppState1 {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            staging_description: StagingString::new(InputLines::Multiline),
            cover_images: StagingVec::default(),
            model_id_widget: Default::default(),
            staging_authors: StagingVec::default(),
            attachments_widget: Default::default(),
            staging_citations: StagingVec::default(),
            custom_config_widget: Default::default(),
            staging_git_repo: Default::default(),
            icon_widget: Default::default(),
            links_widget: Default::default(),
            staging_maintainers: StagingVec::default(),
            staging_tags: StagingVec::default(),
            staging_version: Default::default(),
            staging_documentation: Default::default(),
            staging_license: SearchAndPickWidget::from_enum(Default::default()),

            model_interface_widget: Default::default(),

            model_packing_status: PackingStatus::default(),
            weights_widget: Default::default(),
            notifications_widget: NotificationsWidget::new(),
            notifications_channel: Default::default(),
            zoo_login_widget: Default::default(),
            zoo_model_creation_task: Default::default(),

            close_confirmed: false,
            show_confirmation_dialog: false,
        }
    }
}

impl AppState1{
    pub fn create_model(&self) -> Result<ZooModel>{
        let name = self.staging_name.state()
            .cloned()
            .map_err(|e| GuiError::new_with_rect("Check resoure name for errors", e.failed_widget_rect))?;
        let description = self.staging_description.state()
            .cloned()
            .map_err(|e| GuiError::new_with_rect("Check resource text description for errors", e.failed_widget_rect))?;
        let covers: Vec<_> = self.cover_images.state().into_iter()
            .map(|cover_img_res|{
                cover_img_res
                    .map(|val| val.clone())
                    .map_err(|e| GuiError::new_with_rect("Check cover images for errors", e.failed_widget_rect))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let model_id = self.model_id_widget.state().transpose()
            .map_err(|e| GuiError::new_with_rect("Check model id for errors", e.failed_widget_rect))?
            .cloned();
        let authors = NonEmptyList::
            try_from(
                self.staging_authors.state()
                    .collect_result()
                    .map_err(|e| GuiError::new_with_rect("Check authors for errors", e.failed_widget_rect))?
            )
            .map_err(|_| GuiError::new("Empty authors"))?;
        let attachments = self.attachments_widget.state()
            .collect_result()
            .map_err(|e| GuiError::new_with_rect("Check model attachments for errors", e.failed_widget_rect))?;
        let cite = self.staging_citations.state()
            .collect_result()
            .map_err(|e| GuiError::new_with_rect("Check cites for errors", e.failed_widget_rect))?;
        let non_empty_cites = NonEmptyList::try_from(cite)
            .map_err(|_| GuiError::new("Cites are empty"))?;
        let config = self.custom_config_widget.state().cloned()
            .transpose()
            .map_err(|e| GuiError::new_with_rect("Check custom configs for errors", e.failed_widget_rect))?
            .unwrap_or(serde_json::Map::default());
        let git_repo = self.staging_git_repo.state()
            .transpose()
            .map_err(|e| GuiError::new_with_rect("Check git repo field for errors", e.failed_widget_rect))?
            .map(|val| val.as_ref().clone());
        let icon = self.icon_widget.state().transpose().map_err(|_| GuiError::new("Check icons field for errors"))?;
        let links = self.links_widget.state()
            .collect_result()
            .map_err(|e| GuiError::new_with_rect("Check links for errors", e.failed_widget_rect))?
            .into_iter()
            .map(|s| s.clone())
            .collect();
        let maintainers = self.staging_maintainers.state().collect_result()
            .map_err(|e| GuiError::new_with_rect("Check maintainers field for errors", e.failed_widget_rect))?;
        let tags: Vec<rdf::Tag> = self.staging_tags.state()
            .into_iter()
            .map(|res_ref| res_ref.cloned())
            .collect::<Result<Vec<_>>>()
            .map_err(|e| GuiError::new_with_rect("Check tags for errors", e.failed_widget_rect))?;
        let version = self.staging_version.state()
            .transpose()
            .map_err(|e| GuiError::new_with_rect("Review resource version field", e.failed_widget_rect))?
            .cloned();
        let documentation = self.staging_documentation.state().to_owned();
        let license = self.staging_license.state();
        let model_interface = self.model_interface_widget.state()
            .as_ref()
            .map(|interf| interf.clone())
            .map_err(|_| GuiError::new("Check model interface for errors"))?;
        let weights = self.weights_widget.state()
            .map_err(|e| GuiError::new_with_rect("Check model weights for errors", e.failed_widget_rect))?
            .as_ref().clone();

        Ok(ZooModel {
            name,
            description,
            covers,
            attachments,
            cite: non_empty_cites,
            config,
            git_repo,
            icon,
            links,
            maintainers,
            tags,
            version,
            authors,
            documentation,
            license,
            id: model_id,
            weights,
            interface: model_interface,
        })
    }

    fn save_project(&self, project_file: &Path) -> Result<String, String>{
        let writer = std::fs::File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(project_file).map_err(|err| format!("Could not open project file for writing: {err}"))?;
        AppStateRawData::Version1(self.dump()).save(writer)
            .map_err(|err| format!("Could not serialize project to bytes: {err}"))
            .map(|_| format!("Saved project to {}", project_file.to_string_lossy()))
    }

    fn load_project(&mut self, project_file: &Path) -> Result<(), String>{
        let reader = std::fs::File::open(&project_file).map_err(|err| format!("Could not open project file: {err}"))?;
        let proj_data = match AppStateRawData::load(reader){
            Err(ProjectLoadError::FutureVersion{found_version}) => return Err(format!(
                "Found project data version {found_version}, but this program only supports project data up to {}\n\
                You can try downloading the newest version at https://github.com/kreshuklab/bioimg_rs/releases",
                AppStateRawData::highest_supported_version(),
            )),
            Err(err) => return Err(format!("Could not load project file at {}: {err}", project_file.to_string_lossy())),
            Ok(proj_data) => proj_data,
        };
        match proj_data{
            AppStateRawData::Version1(ver1) => self.restore(ver1),
        }
        Ok(())
    }
}


impl eframe::App for AppState1 {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("Zoo", |ui|{
                    self.zoo_login_widget.draw_and_parse(ui, egui::Id::from("zoo login"));

                    let upload_button = egui::Button::new("⬆ Upload Model");
                    let Ok(user_token) = self.zoo_login_widget.state() else {
                        ui.add_enabled_ui(false, |ui|{
                            ui.add(upload_button).on_disabled_hover_text("Please login first");
                        });
                        return;
                    };
                    let Some(packing_task) = self.zoo_model_creation_task.take() else {
                        if !ui.add(upload_button).clicked(){
                            return;
                        }
                        let model = match self.create_model(){
                            Ok(model) => model,
                            Err(err) => {
                                self.notifications_widget.push_message(Err(err.to_string()));
                                return;
                            }
                        };
                        let user_token = user_token.as_ref().clone();
                        let sender = self.notifications_channel.sender().clone();
                        let on_progress = move |msg: String|{
                            sender.send(TaskResult::Notification(Ok(msg))).unwrap(); //FIXME: is there anything sensible to do if this fails?
                        };
                        self.zoo_model_creation_task = Some(
                            std::thread::spawn(|| upload_model(user_token, model, on_progress))
                        );
                        return
                    };
                    if !packing_task.is_finished() {
                        ui.add_enabled_ui(false, |ui|{
                            ui.add(upload_button).on_disabled_hover_text("Uploading model...");
                        });
                        self.zoo_model_creation_task = Some(packing_task);
                        return;
                    }
                    match packing_task.join().unwrap(){
                        Ok(nickname) => self.notifications_widget.push_message(
                            Ok(format!("Model successfully uploaded: {nickname}"))
                        ),
                        Err(upload_err) => self.notifications_widget.push_message(
                            Err(format!("Could not upload model: {upload_err}"))
                        ),
                    };
                });
                ui.menu_button("File", |ui| {
                    if ui.button("Import Model").clicked() {
                        ui.close_menu();
                        let sender = self.notifications_channel.sender().clone();

                        #[cfg(target_arch="wasm32")]
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(file) = rfd::AsyncFileDialog::new().add_filter("bioimage model", &["zip"],).pick_file().await {
                                let contents = file.read().await;
                                let reader: Box<dyn SeekReadSend + 'static> = Box::new(std::io::Cursor::new(contents));
                                let archive = ZipArchive::new(reader).unwrap();
                                let shared_archive = SharedZipArchive::new(
                                    ZipArchiveIdentifier::Name(file.file_name()),
                                    archive
                                );
                                let message = match rt::zoo_model::ZooModel::try_load_archive(shared_archive){
                                    Err(err) => TaskResult::Notification(Err(format!("Could not import model: {err}"))),
                                    Ok(zoo_model) => TaskResult::ModelImport(Box::new(zoo_model)),
                                };
                                sender.send(message).unwrap();
                            }
                        });

                        #[cfg(not(target_arch="wasm32"))]
                        if let Some(model_path) = rfd::FileDialog::new().add_filter("bioimage model", &["zip"],).pick_file() {
                            let model_path_str = model_path.to_string_lossy();
                            let message = match rt::zoo_model::ZooModel::try_load(&model_path){
                                Err(err) => TaskResult::Notification(Err(format!("Could not import model {model_path_str}: {err}"))),
                                Ok(zoo_model) => TaskResult::ModelImport(Box::new(zoo_model)),
                            };
                            sender.send(message).unwrap();
                        }
                    }
                    #[cfg(not(target_arch="wasm32"))]
                    if ui.button("Save Project").clicked() { 'save_project: {
                        ui.close_menu();
                        let Some(path) = rfd::FileDialog::new().set_file_name("MyProject.bmb").save_file() else {
                            break 'save_project;
                        };
                        let result = self.save_project(&path);
                        self.notifications_widget.push_message(result);
                    }}
                    #[cfg(not(target_arch="wasm32"))]
                    if ui.button("Load Project").clicked() { 'load_project: {
                        ui.close_menu();
                        let Some(path) = rfd::FileDialog::new().add_filter("bioimage model builder", &["bmb"]).pick_file() else {
                            break 'load_project;
                        };
                        if let Err(err) = self.load_project(&path){
                            self.notifications_widget.push_message(Err(err));
                        }
                    }}
                });
                ui.add_space(16.0);
                egui::widgets::global_theme_preference_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 { x: 10.0, y: 10.0 };
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Model Properties");

                ui.horizontal_top(|ui| {
                    ui.strong("Name: ").on_hover_text(
                        "A human-friendly name of the resource description. \
                        May only contains letters, digits, underscore, minus, parentheses and spaces."
                    );
                    self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                    let _name_result = self.staging_name.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Description: ").on_hover_text("A brief description of the model.");
                    self.staging_description.draw_and_parse(ui, egui::Id::from("Name"));
                    let _description_result = self.staging_description.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Cover Images: ").on_hover_text(
                        "Images to be shown to users on the model zoo, preferrably showing what the input \
                        and output look like."
                    );
                    self.cover_images.draw_and_parse(ui, egui::Id::from("Cover Images"));
                    // let cover_img_results = self.cover_images.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Model Id: ").on_hover_text(
                        "A model zoo id of the form <adjective>-<animal>, like 'affable-shark'.\
                        If you're creating a model from scratch, leave this empty and an id will be generated \
                        for you when you upload your model to the zoo."
                    );
                    self.model_id_widget.draw_and_parse(ui, egui::Id::from("Model Id"));
                    // let cover_img_results = self.cover_images.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Authors: ").on_hover_text(
                        "The authors are the creators of this resource description and the primary points of contact."
                    );
                    self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                    // let author_results = self.staging_authors.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Attachments: ").on_hover_text(
                        "Any other files that are relevant to your model can be listed as 'attachments'"
                    );
                    self.attachments_widget.draw_and_parse(ui, egui::Id::from("Attachments"));
                    // let author_results = self.staging_authors.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Cite: ").on_hover_text("How this model should be cited in other publications.");
                    self.staging_citations.draw_and_parse(ui, egui::Id::from("Cite"));
                    // let citation_results = self.staging_citations.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.weak("Custom configs: ").on_hover_text(
                        "A JSON value representing any extra, 'proprietary' parameters your model might need during runtime. \
                        This field is still available for legacy reasons and its use is strongly discouraged"
                    );
                    self.custom_config_widget.draw_and_parse(ui, egui::Id::from("Custom configs"));
                    // let citation_results = self.staging_citations.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Git Repo: ").on_hover_text(
                        "A URL to the git repository with the source code that produced this model"
                    );
                    self.staging_git_repo.draw_and_parse(ui, egui::Id::from("Git Repo"));
                    // let git_repo_result = self.staging_git_repo.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Icon: ").on_hover_text(indoc!("
                        An icon for quick identification on bioimage.io.
                        This can either be an emoji or a small square image."
                    ));
                    group_frame(ui, |ui| {
                        self.icon_widget.draw_and_parse(ui, egui::Id::from("Icon"));
                    });
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Model Zoo Links: ").on_hover_text("IDs of other bioimage.io resources");
                    group_frame(ui, |ui| {
                        self.links_widget.draw_and_parse(ui, egui::Id::from("Model Zoo Links"));
                    });
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Maintainers: ").on_hover_text(
                        "Maintainers of this resource. If not specified, 'authors' are considered maintainers \
                        and at least one of them must specify their `github_user` name."
                    );
                    self.staging_maintainers.draw_and_parse(ui, egui::Id::from("Maintainers"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Tags: ").on_hover_text("Tags to help search and classifying your model in the model zoo");
                    self.staging_tags.draw_and_parse(ui, egui::Id::from("Tags"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Resource Version: ").on_hover_ui(|ui|{
                        ui.horizontal(|ui|{
                            ui.label("The version of this model, following");
                            ui.hyperlink_to("SermVer 2.0", "https://semver.org/#semantic-versioning-200");
                        });

                        ui.label(indoc!("
                            If you upload an updated version of this model to the zoo, you should bump this version \
                            to differentiate it from the previous uploads"
                        ));
                    });
                    self.staging_version.draw_and_parse(ui, egui::Id::from("Version"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Documentation (markdown): ").on_hover_text(
                        "All model documentation should be written here. This field accepts Markdown syntax"
                    );
                    self.staging_documentation.draw_and_parse(ui, egui::Id::from("Documentation"));
                });

                ui.horizontal(|ui| {
                    ui.strong("License: ").on_hover_text("A standard software licence, specifying how this model can be used and for what purposes.");
                    self.staging_license.draw_and_parse(ui, egui::Id::from("License"));
                });

                ui.horizontal(|ui| {
                    ui.strong("Model Interface: ").on_hover_text(indoc!("
                        During runtime, the model weights (specified further down) will be fed with input data. This input data must be \
                        in a particular shape, order, and of a particular type to be accepted by the overall Zoo Model.

                        This data is preprocessed in a pipeline described in the 'preprocessing' fields, and then fed into the model weights.

                        The data comming out of the model weights is then further postprocessed (as specified in the 'postprocessing' \
                        field inside the 'outputs' field), and ultimately output in the shape, order and type specified in the 'outputs' fields."
                    ));
                    group_frame(ui, |ui| {
                        self.model_interface_widget.draw_and_parse(ui, egui::Id::from("Interface"));
                    });
                });

                ui.horizontal(|ui| {
                    ui.strong("Model Weights: ").on_hover_text(indoc!("
                        The serialized weights and biases underlying this model.

                        Model authors are strongly encouraged to use a format other than pytorch satedicts to maximize \
                        intercompatibility between tools. Pytorch statedicts contain arbitrary python code and, crucially, \
                        arbitrary dependencies that are very likely to clash with the dependencies of consumer applications. \
                        Further, pytorch state dicts essentially require client applications to either be written in Python or \
                        to ship the Python interpreter embedded into them.

                        You can include mutiple flavors of your model weights, but they all MUST produce the same results"
                    ));
                    group_frame(ui, |ui| {
                        self.weights_widget.draw_and_parse(ui, egui::Id::from("Weights"));
                    });

                });

                ui.horizontal(|ui| {
                    let save_button_clicked = ui.button("Save Model")
                        .on_hover_text("Save this model to a .zip file, ready to be used or uploaded to the Model Zoo")
                        .clicked();

                    while let Ok(msg) = self.notifications_channel.receiver().try_recv(){
                        match msg{
                            TaskResult::Notification(msg) => self.notifications_widget.push_message(msg),
                            TaskResult::ModelImport(model) => self.set_value(*model),
                        }
                    }
                    if let Some(error_rect) = self.notifications_widget.draw(ui, egui::Id::from("messages_widget")){
                        ui.scroll_to_rect(error_rect, None);
                    }

                    self.model_packing_status = match std::mem::take(&mut self.model_packing_status) {
                        PackingStatus::Done => 'done: {
                            if !save_button_clicked {
                                break 'done PackingStatus::Done;
                            }
                            let zoo_model = match self.create_model(){
                                Ok(zoo_model) => zoo_model,
                                Err(err) => {
                                    self.notifications_widget.push_gui_error(err);
                                    break 'done PackingStatus::Done;
                                }
                            };
                            ui.ctx().request_repaint();
                            let Some(mut path) = rfd::FileDialog::new().save_file() else {
                                break 'done PackingStatus::Done;
                            };
                            if let Some(ext) = path.extension().map(|ex| ex.to_string_lossy()){
                                if ext != "zip"{
                                    self.notifications_widget.push_message(Err(format!("Model extension must be '.zip'. Provided '.{ext}'")));
                                    break 'done PackingStatus::Done;
                                }
                            }
                            path.set_extension("zip");
                            let notification_message = format!("Packing into {}...", path.to_string_lossy());
                            self.notifications_widget.push_message(Ok(notification_message));
                            PackingStatus::Packing {
                                path: path.clone(),
                                task: poll_promise::Promise::spawn_thread("dumping_to_zip", move || {
                                    let file = std::fs::File::create(path)?;
                                    zoo_model.pack_into(file)
                                }),
                            }
                        }
                        PackingStatus::Packing { path, task } => match task.try_take() {
                            Ok(value) => {
                                self.notifications_widget.push_message(match &value{
                                    Ok(_) => Ok(format!("Model saved to {}", path.to_string_lossy())),
                                    Err(err) => Err(format!("Error saving model: {err}")),
                                });
                                PackingStatus::Done
                            },
                            Err(task) => {
                                ui.ctx().request_repaint();
                                PackingStatus::Packing { path, task }
                            }
                        },
                    }
                })
            });
        });

        if ctx.input(|i| i.viewport().close_requested()) {
            if self.close_confirmed {
                // do nothing - we will close
            } else {
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            egui::Modal::new(egui::Id::from("confirmation dialog"))
                .show(ctx, |ui| {
                    ui.label("Are you sure you want to quit?");
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() || ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                            self.show_confirmation_dialog = false;
                            self.close_confirmed = false;
                        }
                        if ui.button("Yes").clicked() || ui.input(|i| i.key_pressed(egui::Key::Enter)){
                            self.show_confirmation_dialog = false;
                            self.close_confirmed = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}
