use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::model::axes::output_axes::OutputSpacetimeSize;
use bioimg_spec::rdf::model::axes::AxisType;
use bioimg_spec::rdf::model::{self as modelrdf, ParameterizedAxisSize};

use super::search_and_pick_widget::SearchAndPickWidget;
use super::staging_float::StagingFloat;
use super::staging_opt::StagingOpt;
use super::staging_string::StagingString;
use super::staging_vec::ItemWidgetConf;
use super::axis_widget::{BatchAxisWidget, ChannelAxisWidget, IndexAxisWidget};
use super::util::group_frame;
use super::{StatefulWidget, ValueWidget};
use super::{axis_size_widget::AnyAxisSizeWidget, staging_num::StagingNum};
use crate::result::{GuiError, Result};

#[derive(Default)]
pub struct OutputSpacetimeSizeWidget{
    pub has_halo: bool,
    pub halo_widget: StagingNum<u64, modelrdf::Halo>,
    pub size_widget: AnyAxisSizeWidget,
}

impl OutputSpacetimeSizeWidget{
    pub fn prefil_parameterized(&mut self, min: usize){
        self.has_halo = false;
        self.size_widget.prefil_parameterized(min)
    }
    pub fn set_value(&mut self, value: OutputSpacetimeSize){
        match value{
            OutputSpacetimeSize::Standard{size} => {
                self.has_halo = false;
                self.size_widget.set_value(size);
            },
            OutputSpacetimeSize::Haloed { size, halo } => {
                self.has_halo = true;
                self.halo_widget.set_value(halo);
                self.size_widget.set_value(size.into());
            },
        }
    }
}

impl StatefulWidget for OutputSpacetimeSizeWidget{
    type Value<'p> = Result<OutputSpacetimeSize>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            self.size_widget.draw_and_parse(ui, id.with("size"));
            ui.horizontal(|ui| {
                ui.strong("Edges have artifacts: ");
                ui.add(egui::widgets::Checkbox::without_text(&mut self.has_halo));
            });
            if self.has_halo {
                ui.horizontal(|ui| {
                    ui.strong("Bad pixels counting from the edge: ");
                    self.halo_widget.draw_and_parse(ui, id.with("halo"));
                });
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(if self.has_halo{
            OutputSpacetimeSize::Haloed {
                size: self.size_widget.state()?
                    .try_into()
                    .map_err(|_: ParameterizedAxisSize| {
                        GuiError::new("Size can't be parameterized when output has halo".to_owned())
                    })?,
                halo: self.halo_widget.state()?,
            }
        }else{
            OutputSpacetimeSize::Standard{size: self.size_widget.state()?}
        })
    }
}

#[derive(Default)]
pub struct OutputSpaceAxisWidget {
    pub id_widget: StagingString<modelrdf::axes::AxisId>,
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,

    pub size_widget: OutputSpacetimeSizeWidget,
    pub unit_widget: StagingOpt<SearchAndPickWidget<modelrdf::SpaceUnit>>,
    pub scale_widget: StagingFloat<modelrdf::AxisScale>,
}

impl OutputSpaceAxisWidget{
    pub fn prefil_parameterized_size(&mut self, min: usize){
        self.size_widget.prefil_parameterized(min);
        self.scale_widget.raw = 1.0.to_string();
    }

    pub fn set_value(&mut self, value: modelrdf::SpaceOutputAxis){
        self.id_widget.set_value(value.id);
        self.description_widget.set_value(value.description);
        self.size_widget.set_value(value.size);
        self.scale_widget.set_value(value.scale);
    }
}

impl StatefulWidget for OutputSpaceAxisWidget{
    type Value<'p> = Result<modelrdf::SpaceOutputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Id: ");
                self.id_widget.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Size: ");
                group_frame(ui, |ui|{
                    self.size_widget.draw_and_parse(ui, id.with("size"));
                });
            });
            ui.horizontal(|ui| {
                ui.strong("Unit: ");
                self.unit_widget.draw_and_parse(ui, id.with("space unit"));
            });
            ui.horizontal(|ui| {
                ui.strong("Scale: ");
                self.scale_widget.draw_and_parse(ui, id.with("scale"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::SpaceOutputAxis {
            id: self.id_widget.state()?,
            description: self.description_widget.state()?,
            unit: self.unit_widget.state(),
            scale: self.scale_widget.state()?,
            size: self.size_widget.state()?,
        })
    }
}

#[derive(Default)]
pub struct OutputTimeAxisWidget {
    pub id_widget: StagingString<modelrdf::axes::AxisId>,
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,

    pub size_widget: OutputSpacetimeSizeWidget,
    pub unit_widget: StagingOpt<SearchAndPickWidget<modelrdf::TimeUnit>>,
    pub scale_widget: StagingFloat<modelrdf::AxisScale>,
}

impl OutputTimeAxisWidget{
    pub fn set_value(&mut self, value: modelrdf::TimeOutputAxis){
        self.id_widget.set_value(value.id);
        self.description_widget.set_value(value.description);
        self.size_widget.set_value(value.size);
        self.scale_widget.set_value(value.scale);
    }
}
impl ValueWidget for OutputAxisWidget{
    type Value<'v> = modelrdf::OutputAxis;
    fn set_value(&mut self, value: modelrdf::OutputAxis){
        match value{
            modelrdf::OutputAxis::Batch(axis) => {
                self.axis_type = AxisType::Batch;
                self.batch_axis_widget.set_value(axis);
            },
            modelrdf::OutputAxis::Channel(axis) => {
                self.axis_type = AxisType::Channel;
                self.channel_axis_widget.set_value(axis);
            },
            modelrdf::OutputAxis::Index(axis) => {
                self.axis_type = AxisType::Index;
                self.index_axis_widget.set_value(axis);
            },
            modelrdf::OutputAxis::Space(axis) => {
                self.axis_type = AxisType::Space;
                self.space_axis_widget.set_value(axis);
            },
            modelrdf::OutputAxis::Time(axis) => {
                self.axis_type = AxisType::Time;
                self.time_axis_widget.set_value(axis);
            },
        }
    }
}

impl StatefulWidget for OutputTimeAxisWidget{
    type Value<'p> = Result<modelrdf::TimeOutputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Id: ");
                self.id_widget.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Size: ");
                group_frame(ui, |ui|{
                    self.size_widget.draw_and_parse(ui, id.with("size"));
                });
            });
            ui.horizontal(|ui| {
                ui.strong("Unit: ");
                self.unit_widget.draw_and_parse(ui, id.with("time unit"));
            });
            ui.horizontal(|ui| {
                ui.strong("Scale: ");
                self.scale_widget.draw_and_parse(ui, id.with("scale"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::TimeOutputAxis {
            id: self.id_widget.state()?,
            description: self.description_widget.state()?,
            unit: self.unit_widget.state(),
            scale: self.scale_widget.state()?,
            size: self.size_widget.state()?
        })
    }
}

#[derive(Default)]
pub struct OutputAxisWidget {
    pub axis_type: AxisType,

    pub batch_axis_widget: BatchAxisWidget,
    pub channel_axis_widget: ChannelAxisWidget,
    pub index_axis_widget: IndexAxisWidget,
    pub space_axis_widget: OutputSpaceAxisWidget,
    pub time_axis_widget: OutputTimeAxisWidget,
}

impl ItemWidgetConf for OutputAxisWidget{
    const ITEM_NAME: &'static str = "Output Axis";
}

impl StatefulWidget for OutputAxisWidget{
    type Value<'p> = Result<modelrdf::OutputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Type: ");
                ui.radio_value(&mut self.axis_type, AxisType::Space, "Space");
                ui.radio_value(&mut self.axis_type, AxisType::Time, "Time");
                ui.radio_value(&mut self.axis_type, AxisType::Batch, "Batch");
                ui.radio_value(&mut self.axis_type, AxisType::Channel, "Channel");
                ui.radio_value(&mut self.axis_type, AxisType::Index, "Index");
            });
            match self.axis_type{
                AxisType::Space => self.space_axis_widget.draw_and_parse(ui, id.with("space")),
                AxisType::Time => self.time_axis_widget.draw_and_parse(ui, id.with("time")),
                AxisType::Batch => self.batch_axis_widget.draw_and_parse(ui, id.with("batch")),
                AxisType::Channel => self.channel_axis_widget.draw_and_parse(ui, id.with("channel")),
                AxisType::Index => self.index_axis_widget.draw_and_parse(ui, id.with("index")),
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.axis_type{
            AxisType::Space => modelrdf::OutputAxis::Space(self.space_axis_widget.state()?),
            AxisType::Time => modelrdf::OutputAxis::Time(self.time_axis_widget.state()?),
            AxisType::Batch => modelrdf::OutputAxis::Batch(self.batch_axis_widget.state()?),
            AxisType::Channel => modelrdf::OutputAxis::Channel(self.channel_axis_widget.state()?),
            AxisType::Index => modelrdf::OutputAxis::Index(self.index_axis_widget.state()?),
        })
    }
}
