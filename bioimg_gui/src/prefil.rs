use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::sync::Arc;

use crate::widgets::inout_tensor_widget::InputTensorWidget;
use crate::widgets::input_axis_widget::InputAxisWidget;
use crate::BioimgGui;
use bioimg_spec::rdf;
use bioimg_spec::rdf::model::axes::output_axes::OutputSpacetimeSize;
use bioimg_spec::rdf::model::{self as modelrdf, AxisSizeReference, QualifiedAxisId, SpecialAxisId};
use bioimg_runtime as rt;
use crate::widgets::weights_widget::TorchscriptWeightsWidget;
use crate::widgets::ValueWidget;


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
            include_str!("/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/README.md")
        );
        out.staging_license.value = rdf::LicenseId::MIT;

        out.model_interface_widget.inputs_widget.staging = vec![
            {
                let mut input_tensor_widget = InputTensorWidget::default();
                input_tensor_widget.id_widget.set_value("raw".to_owned().try_into().unwrap());
                input_tensor_widget.description_widget.set_value("raw input".to_owned().try_into().unwrap());
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

        out.model_interface_widget.outputs_widget.set_value(vec![
            (
                "probability".to_owned().try_into().unwrap(),
                "probability in [0,1]".to_owned().try_into().unwrap(),
                vec![
                    modelrdf::BatchAxis::default().into(),
                    modelrdf::ChannelAxis{
                        id: SpecialAxisId::new(),
                        channel_names: vec!["probability".to_owned().try_into().unwrap()].try_into().unwrap(),
                        description: "".try_into().unwrap(),
                    }.into(),
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
                    }.into(),
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
                    }.into(),
                ],
                Arc::new(
                    rt::NpyArray::try_read(
                        &PathBuf::from(
                            "/home/builder/source/spec-bioimage-io/example_descriptions/models/unet2d_nuclei_broad/test_output.npy"
                        )
                    ).unwrap()
                ),
            ),
        ]);

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
