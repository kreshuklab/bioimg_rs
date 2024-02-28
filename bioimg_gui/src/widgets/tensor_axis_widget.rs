use std::num::NonZeroUsize;

use bioimg_spec::rdf;
use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::model as modelrdf;

use super::axis_size_widget::AxisSizeMode;
use super::enum_widget::EnumWidget;
use super::staging_opt::StagingOpt;
use super::staging_string::StagingString;
use super::staging_vec::StagingVec;
use super::StatefulWidget;
use super::{axis_size_widget::AnyAxisSizeWidget, staging_num::StagingNum};
use crate::result::{GuiError, Result};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum ChannelNamesMode {
    #[default]
    Explicit,
    Pattern,
}

#[derive(PartialEq, Eq, Default, Copy, Clone)]
pub enum AxisType {
    #[default]
    Space,
    Time,
    Channel,
    Batch,
    Index,
}

pub struct InputTensorAxisWidget {
    pub id_widget: StagingString<modelrdf::axes::AxisId>,
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,
    pub axis_type: AxisType,

    //channel stuff
    pub channel_names_mode: ChannelNamesMode,
    pub channel_extent_widget: StagingNum<usize, NonZeroUsize>,
    pub channel_name_prefix_widget: StagingString<String>,
    pub channel_name_suffix_widget: StagingString<String>,

    pub staging_explicit_names: StagingVec<StagingString<rdf::Identifier<String>>>,
    // used by space, time, index
    pub size_widget: AnyAxisSizeWidget,
    // batch size stuff
    pub staging_allow_auto_size: bool,

    pub space_unit_widget: StagingOpt<EnumWidget<modelrdf::SpaceUnit>>,
    pub time_unit_widget: StagingOpt<EnumWidget<modelrdf::TimeUnit>>,
    pub scale_widget: StagingNum<f32, modelrdf::AxisScale>,
}

impl InputTensorAxisWidget {
    #[allow(dead_code)]
    pub fn new(id: impl Into<String>) -> Self {
        let mut id_widget = StagingString::<modelrdf::axes::AxisId>::default();
        id_widget.raw = id.into();
        Self { id_widget, ..Default::default() }
    }
    pub fn set_fixed(&mut self, extent: usize) {
        self.size_widget.mode = AxisSizeMode::Fixed;
        self.size_widget.staging_fixed_size.raw = extent;
        self.scale_widget.raw = 1.0;
    }
}

impl Default for InputTensorAxisWidget {
    fn default() -> Self {
        Self {
            id_widget: Default::default(),
            description_widget: Default::default(),
            axis_type: Default::default(),

            //channel stuff
            channel_names_mode: Default::default(),
            channel_extent_widget: Default::default(),
            channel_name_prefix_widget: Default::default(),
            channel_name_suffix_widget: Default::default(),

            staging_explicit_names: StagingVec::new("Channel Name"),
            // used by space, time, index
            size_widget: Default::default(),
            // batch size stuff
            staging_allow_auto_size: Default::default(),

            space_unit_widget: Default::default(),
            time_unit_widget: Default::default(),
            scale_widget: Default::default(),
        }
    }
}

impl StatefulWidget for InputTensorAxisWidget {
    type Value<'p> = Result<modelrdf::InputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Axis Id: ");
                self.id_widget.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axis Type: ");
                ui.radio_value(&mut self.axis_type, AxisType::Space, "Space");
                ui.radio_value(&mut self.axis_type, AxisType::Time, "Time");
                ui.radio_value(&mut self.axis_type, AxisType::Batch, "Batch");
                ui.radio_value(&mut self.axis_type, AxisType::Channel, "Channel");
                ui.radio_value(&mut self.axis_type, AxisType::Index, "Index");
            });
            match self.axis_type {
                AxisType::Space => {
                    ui.horizontal(|ui| {
                        ui.strong("Size: ");
                        self.size_widget.draw_and_parse(ui, id.with("size"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Unit: ");
                        self.space_unit_widget.draw_and_parse(ui, id.with("space unit"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Scale: ");
                        self.scale_widget.draw_and_parse(ui, id.with("scale"));
                    });
                }
                AxisType::Time => {
                    ui.horizontal(|ui| {
                        ui.strong("Size: ");
                        self.size_widget.draw_and_parse(ui, id.with("size"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Unit: ");
                        self.time_unit_widget.draw_and_parse(ui, id.with("space unit"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Scale: ");
                        self.scale_widget.draw_and_parse(ui, id.with("scale"));
                    });
                }
                AxisType::Index => {
                    ui.horizontal(|ui| {
                        ui.strong("Size: ");
                        self.size_widget.draw_and_parse(ui, id.with("size"));
                    });
                }
                AxisType::Batch => {
                    ui.horizontal(|ui| {
                        ui.strong("Allow arbitrary batch size: ");
                        ui.add(egui::widgets::Checkbox::without_text(&mut self.staging_allow_auto_size));
                    });
                }
                AxisType::Channel => {
                    ui.horizontal(|ui| {
                        ui.strong("Channel Names: ");
                        ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Pattern, "Pattern");
                        ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Explicit, "Explicit");
                    });
                    match self.channel_names_mode {
                        ChannelNamesMode::Pattern => {
                            ui.horizontal(|ui| {
                                ui.strong("Number of Channels: ");
                                self.channel_extent_widget.draw_and_parse(ui, id.with("extent"));
                            });
                            ui.horizontal(|ui| {
                                ui.strong("Prefix: ");
                                self.channel_name_prefix_widget.draw_and_parse(ui, id.with("prefix"));
                            });
                            ui.horizontal(|ui| {
                                ui.strong("Suffix: ");
                                self.channel_name_suffix_widget.draw_and_parse(ui, id.with("suffix"));
                            });
                            if !self.channel_name_prefix_widget.raw.is_empty() || !self.channel_name_suffix_widget.raw.is_empty()
                            {
                                ui.weak(format!(
                                    "e.g.: Channel #7 will be named \"{}7{}\"",
                                    &self.channel_name_prefix_widget.raw, &self.channel_name_suffix_widget.raw,
                                ));
                            }
                        }
                        ChannelNamesMode::Explicit => {
                            self.staging_explicit_names.draw_and_parse(ui, id.with("explicit"));
                        }
                    };
                }
            };
        });
    }
    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.axis_type {
            AxisType::Space => modelrdf::InputAxis::Space(modelrdf::SpaceInputAxis {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                unit: self.space_unit_widget.state(),
                scale: self.scale_widget.state()?,
                size: self.size_widget.state()?,
            }),
            AxisType::Time => modelrdf::InputAxis::Time(modelrdf::TimeInputAxis {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                unit: self.time_unit_widget.state(),
                scale: self.scale_widget.state()?,
                size: self.size_widget.state()?,
            }),
            AxisType::Batch => modelrdf::InputAxis::Batch(modelrdf::BatchAxis {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                size: if self.staging_allow_auto_size {
                    None
                } else {
                    Some(rdf::LiteralInt::<1>)
                },
            }),
            AxisType::Channel => {
                let id = self.id_widget.state()?;
                let description = self.description_widget.state()?;

                let channel_names = match self.channel_names_mode {
                    ChannelNamesMode::Pattern => {
                        let extent: usize = self.channel_extent_widget.state()?.into();
                        (0..extent)
                            .map(|idx| {
                                let prefix = self.channel_name_prefix_widget.state()?;
                                let suffix = self.channel_name_suffix_widget.state()?;
                                let identifier = rdf::Identifier::<String>::try_from(format!("{prefix}{idx}{suffix}"))?;
                                Ok(identifier)
                            })
                            .collect::<Result<Vec<_>>>()?
                    }
                    ChannelNamesMode::Explicit => {
                        let channel_names_result: Result<Vec<rdf::Identifier<_>>, GuiError> =
                            self.staging_explicit_names.state().into_iter().collect();
                        channel_names_result?
                    }
                };

                modelrdf::InputAxis::Channel(modelrdf::ChannelAxis { id, description, channel_names })
            }
            AxisType::Index => modelrdf::InputAxis::Index(modelrdf::axes::IndexAxis {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                size: self.size_widget.state()?,
            }),
        })
    }
}

#[derive(Default)]
pub struct OutputTensorAxisWidget {
    pub input_tensor_widget: InputTensorAxisWidget,
    pub halo_widget: StagingNum<usize, usize>,
}

impl OutputTensorAxisWidget {
    pub fn set_fixed(&mut self, extent: usize) {
        self.input_tensor_widget.set_fixed(extent)
    }
}

impl StatefulWidget for OutputTensorAxisWidget {
    type Value<'p> = Result<modelrdf::OutputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.input_tensor_widget.draw_and_parse(ui, id.with("base"));
        ui.horizontal(|ui| {
            ui.strong("Halo: ");
            self.halo_widget.draw_and_parse(ui, id.with("halo"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let base = self.input_tensor_widget.state()?;
        let halo = self.halo_widget.state()?;
        Ok(match base {
            modelrdf::InputAxis::Batch(batch_axis) => modelrdf::OutputAxis::Batch(batch_axis),
            modelrdf::InputAxis::Channel(channel_axis) => modelrdf::OutputAxis::Channel(channel_axis),
            modelrdf::InputAxis::Index(index_axis) => modelrdf::OutputAxis::Index(index_axis),
            modelrdf::InputAxis::Time(base) => modelrdf::OutputAxis::Time(modelrdf::TimeOutputAxis {
                id: base.id,
                description: base.description,
                unit: base.unit,
                scale: base.scale,
                size: base.size,
                halo,
            }),
            modelrdf::InputAxis::Space(base) => modelrdf::OutputAxis::Space(modelrdf::SpaceOutputAxis {
                id: base.id,
                description: base.description,
                unit: base.unit,
                scale: base.scale,
                size: base.size,
                halo,
            }),
        })
    }
}
