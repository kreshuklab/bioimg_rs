use std::path::PathBuf;
use std::sync::Arc;

use bioimg_runtime as rt;
use bioimg_runtime::zoo_model::{ModelPackingError, ZooModel};
use bioimg_spec::rdf::{self, ResourceId, ResourceName};
use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::non_empty_list::NonEmptyList;

use crate::project_data::{AppStateRawData, ProjectLoadError};
use crate::result::{GuiError, Result, VecResultExt};
use crate::widgets::attachments_widget::AttachmentsWidget;

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
use crate::widgets::version_widget::VersionWidget;
use crate::widgets::weights_widget::WeightsWidget;
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

#[derive(Restore)]
pub struct AppState1 {
    pub staging_name: StagingString<ResourceName>,
    pub staging_description: StagingString<BoundedString<1, 1024>>,
    pub cover_images: StagingVec<SpecialImageWidget<rt::CoverImage>, CoverImageItemConf>,
    pub model_id_widget: StagingOpt<StagingString<ResourceId>>,
    pub staging_authors: StagingVec<CollapsibleWidget<AuthorWidget>>,
    pub attachments_widget: StagingVec<AttachmentsWidget>,
    pub staging_citations: StagingVec<CollapsibleWidget<CiteEntryWidget>>,
    pub custom_config_widget: StagingOpt<JsonObjectEditorWidget>, //FIXME
    pub staging_git_repo: StagingOpt<StagingUrl>,
    pub icon_widget: StagingOpt<IconWidget>,
    pub links_widget: ModelLinksWidget,
    pub staging_maintainers: StagingVec<CollapsibleWidget<MaintainerWidget>>,
    pub staging_tags: StagingVec<StagingString<rdf::Tag>>,
    pub staging_version: StagingOpt<VersionWidget>,

    pub staging_documentation: CodeEditorWidget,
    pub staging_license: SearchAndPickWidget<rdf::LicenseId>,
    //badges
    pub model_interface_widget: ModelInterfaceWidget,
    ////
    pub weights_widget: WeightsWidget,

    #[restore_default]
    pub notifications_widget: NotificationsWidget,
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
            attachments_widget: StagingVec::default(),
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

            close_confirmed: false,
            show_confirmation_dialog: false,
        }
    }
}


impl eframe::App for AppState1 {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import Model").clicked() {
                        ui.close_menu();
                        if let Some(model_path) = rfd::FileDialog::new().add_filter("bioimage model", &["zip"],).pick_file() {
                            match rt::zoo_model::ZooModel::try_load(&model_path){
                                Err(err) => self.notifications_widget.push_message(
                                    Err(format!("Could not import model {}: {err}", model_path.to_string_lossy()))
                                ),
                                Ok(zoo_model) => self.set_value(zoo_model)
                            }
                        }
                    }
                    if ui.button("Save Project").clicked() { 'save_project: {
                        ui.close_menu();
                        let Some(path) = rfd::FileDialog::new().set_file_name("MyProject.bmb").save_file() else {
                            break 'save_project;
                        };
                        let result = || -> Result<String, String>{
                            println!("Trying to open {path:?} for writing");
                            let writer = std::fs::File::options()
                                .write(true)
                                .create(true)
                                .truncate(true)
                                .open(&path).map_err(|err| format!("Could not open project file for writing: {err}"))?;
                            AppStateRawData::Version1(self.dump()).save(writer)
                                .map_err(|err| format!("Could not serialize project to bytes: {err}"))
                                .map(|_| format!("Saved project to {}", path.to_string_lossy()))
                        }();
                        self.notifications_widget.push_message(result);
                    }}
                    if ui.button("Load Project").clicked() { 'load_project: {
                        ui.close_menu();
                        let Some(path) = rfd::FileDialog::new().add_filter("bioimage model builder", &["bmb"]).pick_file() else {
                            break 'load_project;
                        };
                        let result = || -> Result<(), String>{
                            let reader = std::fs::File::open(&path).map_err(|err| format!("Could not open project file: {err}"))?;
                            let proj_data = match AppStateRawData::load(reader){
                                Err(ProjectLoadError::FutureVersion{found_version}) => return Err(format!(
                                    "Found project data version {found_version}, but this program only supports project data up to {}\n\
                                    You can try downloading the newest version at https://github.com/kreshuklab/bioimg_rs/releases",
                                    AppStateRawData::highest_supported_version(),
                                )),
                                Err(err) => return Err(format!("Could not load project file at {}: {err}", path.to_string_lossy())),
                                Ok(proj_data) => proj_data,
                            };
                            match proj_data{
                                AppStateRawData::Version1(ver1) => self.restore(ver1),
                            }
                            Ok(())
                        }();
                        if let Err(err) = result{
                            self.notifications_widget.push_message(Err(err));
                        }
                    }}
                });
                ui.add_space(16.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing = egui::Vec2 { x: 10.0, y: 10.0 };
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("Model Properties");

                ui.horizontal_top(|ui| {
                    ui.strong("Name: ");
                    self.staging_name.draw_and_parse(ui, egui::Id::from("Name"));
                    let _name_result = self.staging_name.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Description: ");
                    self.staging_description.draw_and_parse(ui, egui::Id::from("Name"));
                    let _description_result = self.staging_description.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Cover Images: ");
                    self.cover_images.draw_and_parse(ui, egui::Id::from("Cover Images"));
                    // let cover_img_results = self.cover_images.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Model Id: ");
                    self.model_id_widget.draw_and_parse(ui, egui::Id::from("Model Id"));
                    // let cover_img_results = self.cover_images.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Authors: ");
                    self.staging_authors.draw_and_parse(ui, egui::Id::from("Authors"));
                    // let author_results = self.staging_authors.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Attachments: ");
                    self.attachments_widget.draw_and_parse(ui, egui::Id::from("Attachments"));
                    // let author_results = self.staging_authors.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Cite: ");
                    self.staging_citations.draw_and_parse(ui, egui::Id::from("Cite"));
                    // let citation_results = self.staging_citations.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Custom configs: ");
                    self.custom_config_widget.draw_and_parse(ui, egui::Id::from("Custom configs"));
                    // let citation_results = self.staging_citations.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Git Repo: ");
                    self.staging_git_repo.draw_and_parse(ui, egui::Id::from("Git Repo"));
                    // let git_repo_result = self.staging_git_repo.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Icon: ");
                    group_frame(ui, |ui| {
                        self.icon_widget.draw_and_parse(ui, egui::Id::from("Icon"));
                    });
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Model Zoo Links: ");
                    group_frame(ui, |ui| {
                        self.links_widget.draw_and_parse(ui, egui::Id::from("Model Zoo Links"));
                    });
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Maintainers: ");
                    self.staging_maintainers.draw_and_parse(ui, egui::Id::from("Maintainers"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Tags: ");
                    self.staging_tags.draw_and_parse(ui, egui::Id::from("Tags"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Resource Version: ");
                    self.staging_version.draw_and_parse(ui, egui::Id::from("Version"));
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Documentation (markdown): ");
                    self.staging_documentation.draw_and_parse(ui, egui::Id::from("Documentation"));
                });

                ui.horizontal(|ui| {
                    ui.strong("License: ");
                    self.staging_license.draw_and_parse(ui, egui::Id::from("License"));
                });

                ui.horizontal(|ui| {
                    ui.strong("Model Interface: ");
                    group_frame(ui, |ui| {
                        self.model_interface_widget.draw_and_parse(ui, egui::Id::from("Interface"));
                    });
                });

                ui.horizontal(|ui| {
                    ui.strong("Model Weights: ");
                    group_frame(ui, |ui| {
                        self.weights_widget.draw_and_parse(ui, egui::Id::from("Weights"));
                    });

                });

                ui.horizontal(|ui| {
                    let save_button_clicked = ui.button("Save Model").clicked();
                    self.notifications_widget.draw(ui, egui::Id::from("messages_widget"));

                    self.model_packing_status = match std::mem::take(&mut self.model_packing_status) {
                        PackingStatus::Done => 'done: {
                            if !save_button_clicked {
                                break 'done PackingStatus::Done;
                            }
                            let zoo_model_res = (|| -> Result<ZooModel>{
                                let model_interface = self.model_interface_widget.state()
                                    .as_ref()
                                    .map(|interf| interf.clone())
                                    .map_err(|_| GuiError::new("Check model interface for errors".into()))?;

                                let covers: Vec<_> = self.cover_images.state().into_iter()
                                    .map(|cover_img_res|{
                                        cover_img_res
                                            .map(|val| val.clone())
                                            .map_err(|_| GuiError::new("Check cover images for errors".into()))
                                    })
                                    .collect::<Result<Vec<_>, _>>()?;

                                let attachments = self.attachments_widget.state()
                                    .collect_result()
                                    .map_err(|_| GuiError::new("Check model attachments for errors".into()))?;

                                let cite = self.staging_citations.state().collect_result().map_err(|_| GuiError::new("Check cites for errors".into()))?;
                                let non_empty_cites = NonEmptyList::try_from(cite)
                                    .map_err(|_| GuiError::new("Cites are empty".into()))?;

                                let tags: Vec<rdf::Tag> = self.staging_tags.state()
                                    .into_iter()
                                    .map(|res_ref| res_ref.cloned())
                                    .collect::<Result<Vec<_>>>()
                                    .map_err(|_| GuiError::new("Check tags for errors".into()))?;

                                let authors = NonEmptyList::try_from(
                                    self.staging_authors.state().collect_result().map_err(|_| GuiError::new("Check authors for errors".into()))?
                                ).map_err(|_| GuiError::new("Empty authors".into()))?;

                                Ok(ZooModel {
                                    description: self.staging_description.state()
                                        .cloned()
                                        .map_err(|_| GuiError::new("Check resource text description for errors".into()))?,
                                    covers,
                                    attachments,
                                    cite: non_empty_cites,
                                    config: self.custom_config_widget.state().cloned()
                                        .transpose()
                                        .map_err(|_| GuiError::new("Check custom configs for errors".into()))?
                                        .unwrap_or(serde_json::Map::default()),
                                    git_repo: self.staging_git_repo.state()
                                        .transpose()
                                        .map_err(|_| GuiError::new("Check git repo field for errors".into()))?
                                        .map(|val| val.as_ref().clone()),
                                    icon: self.icon_widget.state().transpose().map_err(|_| GuiError::new("Check icons field for errors".into()))?,
                                    links: Vec::<String>::new(),// FIXME: grab from widget,
                                    maintainers: self.staging_maintainers.state().collect_result().map_err(|_| GuiError::new("Check maintainers field for errors".into()))?,
                                    tags,
                                    version: self.staging_version.state()
                                        .transpose()
                                        .map_err(|_| GuiError::new("Review resource version field".into()))?
                                        .cloned(),
                                    authors,
                                    documentation: self.staging_documentation.state().to_owned(),
                                    license: self.staging_license.state(),
                                    name: self.staging_name.state()
                                        .cloned()
                                        .map_err(|_| GuiError::new("Check resoure name for errors".into()))?,
                                    id: self.model_id_widget.state().transpose().map_err(|_| GuiError::new("Check model id for errors".into()))?.cloned(),
                                    weights: self.weights_widget.state().map_err(|_| GuiError::new("Check model weights for errors".into()))?.as_ref().clone(),
                                    interface: model_interface,
                                })
                            })();

                            let zoo_model = match zoo_model_res{
                                Ok(zoo_model) => {
                                    self.notifications_widget.push_message(Ok(format!("Model saved successfully")));
                                    zoo_model
                                }
                                Err(err) => {
                                    self.notifications_widget.push_message(Err(err.to_string()));
                                    break 'done PackingStatus::Done;
                                }
                            };

                            ui.ctx().request_repaint();
                            let Some(path) = rfd::FileDialog::new().save_file() else {
                                break 'done PackingStatus::Done;
                            };
                            {
                                let notification_message = format!("Packing into {}...", path.to_string_lossy());
                                let next_state = PackingStatus::Packing {
                                    path: path.clone(),
                                    task: poll_promise::Promise::spawn_thread("dumping_to_zip", move || {
                                        let file = std::fs::File::create(path)?;
                                        zoo_model.pack_into(file)
                                    }),
                                };
                                self.notifications_widget.push_message(Ok(notification_message));
                                next_state
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
            egui::Window::new("Are you sure you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.close_confirmed = false;
                        }
                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.close_confirmed = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}
