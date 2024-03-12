use std::path::PathBuf;

use bioimg_runtime as rt;
use bioimg_runtime::npy_array::ArcNpyArray;
use bioimg_runtime::zoo_model::{ModelPackingError, ZooModel};
use bioimg_spec::rdf;
use bioimg_spec::rdf::bounded_string::BoundedString;

use crate::result::Result;
use crate::widgets::enum_widget::EnumWidget;
use crate::widgets::file_widget::FileWidget;
use crate::widgets::model_interface_widget::ModelInterfaceWidget;
use crate::widgets::staging_opt::StagingOpt;
use crate::widgets::staging_string::{InputLines, StagingString};
use crate::widgets::staging_vec::StagingVec;
use crate::widgets::{
    author_widget::StagingAuthor2, cite_widget::StagingCiteEntry2, code_editor_widget::CodeEditorWidget,
    cover_image_widget::CoverImageWidget, icon_widget::StagingIcon, maintainer_widget::StagingMaintainer, url_widget::StagingUrl,
    util::group_frame, StatefulWidget,
};

struct ZooModelPackResult {
    model: ZooModel<ArcNpyArray>,
    path: PathBuf,
    save_result: Result<(), ModelPackingError>,
}

enum PackingStatus {
    Done(Option<ZooModelPackResult>),
    Packing {
        path: PathBuf,
        model: ZooModel<ArcNpyArray>,
        task: poll_promise::Promise<Result<(), ModelPackingError>>,
    },
}

impl Default for PackingStatus {
    fn default() -> Self {
        Self::Done(None)
    }
}

pub struct BioimgGui {
    staging_name: StagingString<BoundedString<1, 127>>,
    staging_description: StagingString<BoundedString<1, 1023>>,
    cover_images: StagingVec<CoverImageWidget>,
    // id?
    staging_authors: StagingVec<StagingAuthor2>,
    attachments_widget: StagingVec<FileWidget<Result<rt::LocalRdfAttachment>>>,
    staging_citations: StagingVec<StagingCiteEntry2>,
    //config
    staging_git_repo: StagingOpt<StagingUrl>,
    staging_icon: StagingIcon,
    //links
    staging_maintainers: StagingVec<StagingMaintainer>,
    staging_tags: StagingVec<StagingString<BoundedString<3, 1024>>>,
    staging_version: StagingString<rdf::Version>,

    staging_documentation: StagingOpt<CodeEditorWidget>,
    staging_license: EnumWidget<rdf::SpdxLicense>,
    //badges
    model_interface_widget: ModelInterfaceWidget,
    ////
    model_packing_status: PackingStatus,
}

impl Default for BioimgGui {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            staging_description: StagingString::new(InputLines::Multiline),
            cover_images: StagingVec::new("Cover Image"),
            staging_authors: StagingVec::new("Author"),
            attachments_widget: StagingVec::new("Attachment"),
            staging_citations: StagingVec::new("Cite"),
            staging_git_repo: Default::default(),
            staging_icon: Default::default(),
            staging_maintainers: StagingVec::new("Maintainer"),
            staging_tags: StagingVec::new("Tag"),
            staging_version: Default::default(),
            staging_documentation: Default::default(),
            staging_license: Default::default(),

            model_interface_widget: Default::default(),

            model_packing_status: PackingStatus::default(),
        }
    }
}

impl BioimgGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for BioimgGui {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
                    ui.strong("Git Repo: ");
                    self.staging_git_repo.draw_and_parse(ui, egui::Id::from("Git Repo"));
                    // let git_repo_result = self.staging_git_repo.state();
                });
                ui.add_space(10.0);

                ui.horizontal_top(|ui| {
                    ui.strong("Icon: ");
                    group_frame(ui, |ui| {
                        self.staging_icon.draw_and_parse(ui, egui::Id::from("Icon"));
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
                    self.model_interface_widget.draw_and_parse(ui, egui::Id::from("Interface"));
                });

                ui.horizontal(|ui| {
                    self.model_packing_status = match std::mem::take(&mut self.model_packing_status) {
                        PackingStatus::Done(payload) => 'done: {
                            let message = match &payload {
                                Some(ZooModelPackResult { path, save_result: Ok(_), .. }) => {
                                    format!("Saved model to {}", path.to_string_lossy())
                                }
                                Some(ZooModelPackResult { save_result: Err(err), .. }) => err.to_string(),
                                None => "".into(),
                            };
                            let save_button_clicked = ui
                                .add_enabled_ui(self.model_interface_widget.parsed.is_ok(), |ui| {
                                    ui.button("Save Model").clicked()
                                })
                                .inner;
                            ui.label(message);
                            if !save_button_clicked {
                                break 'done PackingStatus::Done(payload);
                            }
                            let Ok(model_interface) = &self.model_interface_widget.parsed else {
                                break 'done PackingStatus::Done(payload);
                            };
                            let Some(path) = rfd::FileDialog::new().pick_file() else {
                                break 'done PackingStatus::Done(payload);
                            };
                            let model = ZooModel { interface: model_interface.clone() };

                            let model_to_pack = model.clone();
                            PackingStatus::Packing {
                                path: path.clone(),
                                model: model.clone(),
                                task: poll_promise::Promise::spawn_thread("dumping_to_zip", move || {
                                    let file = std::fs::File::create(&path)?;
                                    model_to_pack.pack_into(file)
                                }),
                            }
                        }
                        PackingStatus::Packing { path, model, task } => match task.try_take() {
                            Ok(value) => PackingStatus::Done(Some(ZooModelPackResult { path, model, save_result: value })),
                            Err(task) => {
                                ui.add_enabled_ui(false, |ui| ui.button("Save Model"));
                                ui.label(format!("Packing into {}...", path.to_string_lossy()));
                                PackingStatus::Packing { path, model, task }
                            }
                        },
                    }
                })
            });
        });
    }
}
