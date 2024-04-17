use std::num::NonZeroUsize;
use std::path::PathBuf;

use bioimg_runtime as rt;
use bioimg_runtime::zoo_model::{ModelPackingError, ZooModel};
use bioimg_spec::rdf::model::axes::output_axes::OutputSpacetimeSize;
use bioimg_spec::rdf::model::{self as modelrdf, AxisSizeReference, QualifiedAxisId, SpecialAxisId};
use bioimg_spec::rdf::{self, ResourceName};
use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::non_empty_list::NonEmptyList;

use crate::result::{GuiError, Result, VecResultExt};
use crate::widgets::attachments_widget::AttachmentsWidget;
use crate::widgets::inout_tensor_widget::{InputTensorWidget, OutputTensorWidget};
use crate::widgets::input_axis_widget::InputAxisWidget;
use crate::widgets::output_axis_widget::OutputAxisWidget;

// use crate::widgets::cover_image_widget::CoverImageWidget;
use crate::widgets::enum_widget::EnumWidget;
use crate::widgets::image_widget::ImageWidget;
use crate::widgets::model_interface_widget::ModelInterfaceWidget;
use crate::widgets::notice_widget::NoticeWidget;
use crate::widgets::staging_opt::StagingOpt;
use crate::widgets::staging_string::{InputLines, StagingString};
use crate::widgets::staging_vec::StagingVec;
use crate::widgets::weights_widget::{TorchscriptWeightsWidget, WeightsWidget};
use crate::widgets::ValueWidget;
use crate::widgets::{
    author_widget::StagingAuthor2, cite_widget::StagingCiteEntry2, code_editor_widget::CodeEditorWidget,
    icon_widget::IconWidget, maintainer_widget::StagingMaintainer, url_widget::StagingUrl,
    util::group_frame, StatefulWidget,
};

enum PackingStatus {
    Done,
    Packing {
        path: PathBuf,
        task: poll_promise::Promise<Result<(), ModelPackingError>>,
    },
}

impl Default for PackingStatus {
    fn default() -> Self {
        Self::Done
    }
}

pub struct BioimgGui {
    staging_name: StagingString<ResourceName>,
    staging_description: StagingString<BoundedString<1, 1023>>,
    cover_images: StagingVec<ImageWidget<rt::CoverImage>>,
    // id?
    staging_authors: StagingVec<StagingAuthor2>,
    attachments_widget: StagingVec<AttachmentsWidget>,
    staging_citations: StagingVec<StagingCiteEntry2>,
    //config
    staging_git_repo: StagingOpt<StagingUrl>,
    icon_widget: StagingOpt<IconWidget>,
    //links
    staging_maintainers: StagingVec<StagingMaintainer>,
    staging_tags: StagingVec<StagingString<BoundedString<3, 1024>>>,
    staging_version: StagingOpt<StagingString<rdf::Version>>,

    staging_documentation: CodeEditorWidget,
    staging_license: EnumWidget<rdf::LicenseId>,
    //badges
    model_interface_widget: ModelInterfaceWidget,
    ////
    model_packing_status: PackingStatus,
    weights_widget: WeightsWidget,

    packing_notice: NoticeWidget,
}

impl Default for BioimgGui {
    fn default() -> Self {
        Self {
            staging_name: StagingString::new(InputLines::SingleLine),
            staging_description: StagingString::new(InputLines::Multiline),
            cover_images: StagingVec::default(),
            staging_authors: StagingVec::default(),
            attachments_widget: StagingVec::default(),
            staging_citations: StagingVec::default(),
            staging_git_repo: Default::default(),
            icon_widget: Default::default(),
            staging_maintainers: StagingVec::default(),
            staging_tags: StagingVec::default(),
            staging_version: Default::default(),
            staging_documentation: Default::default(),
            staging_license: Default::default(),

            model_interface_widget: Default::default(),

            model_packing_status: PackingStatus::default(),
            weights_widget: Default::default(),
            packing_notice: NoticeWidget::new_hidden(),
        }
    }
}

impl BioimgGui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut out = Self::default();

        out.staging_name.set_value("UNet 2D Nuclei Broad".try_into().unwrap());
        out.staging_description.set_value("A 2d U-Net trained on the nuclei broad dataset.".try_into().unwrap());
        out.staging_citations.set_value(vec![
            rdf::CiteEntry2{
                text: "Ronneberger, Olaf et al. U-net: Convolutional networks for biomedical image segmentation. MICCAI 2015."
                    .try_into().unwrap(),
                doi: Some("10.1007/978-3-319-24574-4_28".try_into().unwrap()),
                url: None,
            },
            rdf::CiteEntry2{
                text: "2018 Data Science Bowl".try_into().unwrap(),
                doi: Some("10.1007/978-3-319-24574-4_28".try_into().unwrap()),
                url: Some("https://www.kaggle.com/c/data-science-bowl-2018".to_owned().try_into().unwrap()),
            },

        ]);
        out.cover_images.set_value(vec![
            PathBuf::try_from(
                "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/cover0.png"
            ).unwrap()
        ]);

        out.staging_authors.set_value(vec![
            rdf::Author2{
                name: "Constantin Pape;@bioimage-io".to_owned().try_into().unwrap(),
                affiliation: Some("EMBL Heidelberg".try_into().unwrap()),
                orcid: Some("0000-0001-6562-7187".to_owned().try_into().unwrap()),
                email: None,
                github_user: None
            },
            rdf::Author2{
                name: "Fynn Beuttenmueller".to_owned().try_into().unwrap(),
                affiliation: Some("EMBL Heidelberg".try_into().unwrap()),
                orcid: Some("0000-0002-8567-6389".to_owned().try_into().unwrap()),
                email: None,
                github_user: None,
            },
        ]);
        out.attachments_widget.set_value(vec![]);
        out.staging_citations.set_value(vec![
            rdf::CiteEntry2{
                text: "Ronneberger, Olaf et al. U-net: Convolutional networks for biomedical image segmentation. MICCAI 2015."
                    .try_into().unwrap(),
                doi: Some("10.1007/978-3-319-24574-4_28".try_into().unwrap()),
                url: None,
            },
            rdf::CiteEntry2{
                text: "2018 Data Science Bowl".try_into().unwrap(),
                doi: None,
                url: Some("https://www.kaggle.com/c/data-science-bowl-2018".to_owned().try_into().unwrap()),
            }
        ]);
        out.staging_git_repo.set_value(Some(
            "https://github.com/bioimage-io/spec-bioimage-io/tree/main/example_descriptions/models/unet2d_nuclei_broad"
                .to_owned().try_into().unwrap()
        ));
        out.staging_maintainers.set_value(vec![
            rdf::Maintainer{
                name: Some("Constantin Pape".to_owned().try_into().unwrap()),
                github_user: "constantinpape".try_into().unwrap(),
                affiliation: None,
                email: None,
                orcid: None,
            },
            rdf::Maintainer{
                name: Some("Fynn Beuttenmueller".to_owned().try_into().unwrap()),
                github_user: "fynnbe".to_owned().try_into().unwrap(),
                affiliation: None,
                email: None,
                orcid: None,
            }
        ]);
        out.staging_tags.set_value(vec![
            "unet2d".to_owned().try_into().unwrap(),
            "pytorch".to_owned().try_into().unwrap(),
            "nucleus".to_owned().try_into().unwrap(),
            "segmentation".to_owned().try_into().unwrap(),
            "dsb2018".to_owned().try_into().unwrap(),
        ]);
        out.staging_documentation.set_value(
            "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/README.md"
        );
        out.staging_license.value = rdf::LicenseId::MIT;

        out.model_interface_widget.inputs_widget.staging = vec![
            {
                let mut input_tensor_widget = InputTensorWidget::default();
                input_tensor_widget.id_widget.raw = "raw".into();
                input_tensor_widget.description_widget.raw = "raw input".into();
                input_tensor_widget.axes_widget.staging = vec![
                    InputAxisWidget::new(Some(
                        modelrdf::BatchAxis::default().into()
                    )),
                    InputAxisWidget::new(Some(
                        modelrdf::ChannelAxis{
                            id: SpecialAxisId::new(),
                            channel_names: vec!["raw_intensity".to_owned().try_into().unwrap()].try_into().unwrap(),
                            description: "".try_into().unwrap(),
                        }.into()
                    )),
                    InputAxisWidget::new(Some(
                        modelrdf::SpaceInputAxis{
                            id: "y".to_owned().try_into().unwrap(),
                            size: NonZeroUsize::try_from(512usize).unwrap().into(),
                            description: Default::default(),
                            scale: Default::default(),
                            unit: Default::default(),
                        }.into()
                    )),
                    InputAxisWidget::new(Some(
                        modelrdf::SpaceInputAxis{
                            id: "x".to_owned().try_into().unwrap(),
                            size: NonZeroUsize::try_from(512usize).unwrap().into(),
                            description: Default::default(),
                            scale: Default::default(),
                            unit: Default::default(),
                        }.into()
                    )),
                ];
                input_tensor_widget.test_tensor_widget.set_path(
                    PathBuf::from(
                        "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/test_input.npy"
                    )
                );
                input_tensor_widget
            }
        ];

        out.model_interface_widget.outputs_widget.staging = vec![
            {
                let mut output_tensor_widget = OutputTensorWidget::default();
                output_tensor_widget.id_widget.raw = "probability".into();
                output_tensor_widget.description_widget.raw = "probability in [0,1]".into();
                output_tensor_widget.axes_widget.staging = vec![
                    OutputAxisWidget::new(Some(
                        modelrdf::BatchAxis::default().into()
                    )),
                    OutputAxisWidget::new(Some(
                        modelrdf::ChannelAxis{
                            id: SpecialAxisId::new(),
                            channel_names: vec!["probability".to_owned().try_into().unwrap()].try_into().unwrap(),
                            description: "".try_into().unwrap(),
                        }.into()
                    )),
                    OutputAxisWidget::new(Some(
                        modelrdf::SpaceOutputAxis{
                            id: "y".to_owned().try_into().unwrap(),
                            size: OutputSpacetimeSize::Haloed{
                                halo: 32u64.try_into().unwrap(),
                                size: AxisSizeReference{
                                    offset: Default::default(),
                                    qualified_axis_id: QualifiedAxisId{
                                        tensor_id: "raw".to_owned().try_into().unwrap(),
                                        axis_id: "y".to_owned().try_into().unwrap(),
                                    },
                                }.into()
                            },
                            description: Default::default(),
                            scale: Default::default(),
                            unit: Default::default(),
                        }.into()
                    )),
                    OutputAxisWidget::new(Some(
                        modelrdf::SpaceOutputAxis{
                            id: "x".to_owned().try_into().unwrap(),
                            size: OutputSpacetimeSize::Haloed{
                                halo: 32u64.try_into().unwrap(),
                                size: AxisSizeReference{
                                    offset: Default::default(),
                                    qualified_axis_id: QualifiedAxisId{
                                        tensor_id: "raw".to_owned().try_into().unwrap(),
                                        axis_id: "x".to_owned().try_into().unwrap(),
                                    },
                                }.into()
                            },
                            description: Default::default(),
                            scale: Default::default(),
                            unit: Default::default(),
                        }.into()
                    )),
                ];
                output_tensor_widget.test_tensor_widget.set_path(
                    PathBuf::from(
                        "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/test_output.npy"
                    )
                );
                output_tensor_widget
            }
        ];

        out.weights_widget.torchscript_weights_widget.0 = Some({
            let mut torchscript_weights_widget = TorchscriptWeightsWidget::default();
            torchscript_weights_widget.base_widget.source_widget.set_path(PathBuf::from(
                "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/weights.pt"
            ));
            torchscript_weights_widget.pytorch_version_widget.raw = "1.5.1".into();
            torchscript_weights_widget
        });

        out
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
                        self.icon_widget.draw_and_parse(ui, egui::Id::from("Icon"));
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
                    let now = std::time::Instant::now();
                    let save_button_clicked = ui.button("Save Model").clicked();
                    self.packing_notice.draw(ui, now);

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

                                let covers: Vec<_> = self.cover_images.state().into_iter().map(|cover_img_res|{
                                    cover_img_res.map_err(|_| GuiError::new("Check cover images for errors".into()))
                                }).collect::<Result<Vec<_>, _>>()?;

                                let attachments_state = self.attachments_widget.state();
                                let attachments = attachments_state.into_iter().map(|file_widget_state|{
                                    match file_widget_state.loaded_value(){
                                        Some(Ok(val)) => Ok(val.clone()),
                                        _ => return Err(GuiError::new("Check model attachments for errors".into()))
                                    }
                                }).collect::<Result<Vec<_>, _>>()?;

                                let cite = self.staging_citations.state().collect_result().map_err(|_| GuiError::new("Check cites for errors".into()))?;
                                let non_empty_cites = NonEmptyList::try_from(cite)
                                    .map_err(|_| GuiError::new("Cites are empty".into()))?;

                                let tags: Vec<String> = self.staging_tags.state()
                                    .into_iter()
                                    .map(|res|{
                                        res.map(|tag| String::from(tag))
                                    }).collect::<Result<_>>()
                                    .map_err(|_| GuiError::new("Check tags for errors".into()))?;

                                let authors = NonEmptyList::try_from(
                                    self.staging_authors.state().collect_result().map_err(|_| GuiError::new("Check authors for errors".into()))?
                                ).map_err(|_| GuiError::new("Empty authors".into()))?;

                                Ok(ZooModel {
                                    description: self.staging_description.state().map_err(|_| GuiError::new("Check resource text description for errors".into()))?,
                                    covers,
                                    attachments,
                                    cite: non_empty_cites,
                                    git_repo: self.staging_git_repo.state().transpose().map_err(|_| GuiError::new("Check git repo field for errors".into()))?,
                                    icon: self.icon_widget.state().transpose().map_err(|_| GuiError::new("Check icons field for errors".into()))?,
                                    links: Vec::<String>::new(),// FIXME: grab from widget,
                                    maintainers: self.staging_maintainers.state().collect_result().map_err(|_| GuiError::new("Check maintainers field for errors".into()))?,
                                    tags,
                                    version: self.staging_version.state()
                                        .transpose()
                                        .map_err(|_| GuiError::new("Review resource version field".into()))?,
                                    authors,
                                    documentation: self.staging_documentation.state().to_owned(),
                                    license: self.staging_license.state(),
                                    name: self.staging_name.state().map_err(|_| GuiError::new("Check resoure name for errors".into()))?,
                                    weights: self.weights_widget.state().map_err(|_| GuiError::new("Check model weights for errors".into()))?.as_ref().clone(),
                                    interface: model_interface,
                                })
                            })();

                            let zoo_model = match zoo_model_res{
                                Ok(zoo_model) => {
                                    self.packing_notice.update_message(Ok(format!("Model saved successfully")));
                                    zoo_model
                                }
                                Err(err) => {
                                    self.packing_notice.update_message(Err(err.to_string()));
                                    break 'done PackingStatus::Done;
                                }
                            };

                            ui.ctx().request_repaint();
                            let Some(path) = rfd::FileDialog::new().save_file() else {
                                break 'done PackingStatus::Done;
                            };
                            PackingStatus::Packing {
                                path: path.clone(),
                                task: poll_promise::Promise::spawn_thread("dumping_to_zip", move || {
                                    let file = std::fs::File::create(&path)?;
                                    zoo_model.pack_into(file)
                                }),
                            }
                        }
                        PackingStatus::Packing { path, task } => match task.try_take() {
                            Ok(value) => {
                                self.packing_notice.update_message(match &value{
                                    Ok(_) => Ok(format!("Model saved to {}", path.to_string_lossy())),
                                    Err(err) => Err(format!("Error saving model: {err}")),
                                });
                                PackingStatus::Done
                            },
                            Err(task) => {
                                self.packing_notice.update_message(Ok(format!("Packing into {}...", path.to_string_lossy())));
                                ui.ctx().request_repaint();
                                PackingStatus::Packing { path, task }
                            }
                        },
                    }
                })
            });
        });
    }
}
