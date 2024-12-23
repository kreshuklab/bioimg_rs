use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::model::axes::AxisType;
use bioimg_spec::rdf::model as modelrdf;

use super::axis_physical_size_widget::PhysicalSizeWidget;
use super::collapsible_widget::{CollapsibleWidget, SummarizableWidget};
use super::search_and_pick_widget::SearchAndPickWidget;
use super::staging_string::StagingString;
use super::staging_vec::ItemWidgetConf;
use super::axis_widget::{axis_description_label, axis_id_label, BatchAxisWidget, ChannelAxisWidget, IndexAxisWidget};
use super::{Restore, StatefulWidget, ValueWidget};
use super::axis_size_widget::AnyAxisSizeWidget;
use crate::result::Result;


#[derive(Default, Restore)]
pub struct InputSpaceAxisWidget {
    pub id_widget: StagingString<modelrdf::axes::AxisId>,
    pub description_widget: StagingString<BoundedString<0, 128>>,

    pub size_widget: AnyAxisSizeWidget,
    pub physical_size_widget: PhysicalSizeWidget<modelrdf::SpaceUnit>,
}

impl InputSpaceAxisWidget{
    pub fn prefil_parameterized_size(&mut self, min: usize){
        self.size_widget.prefil_parameterized(min);
        self.physical_size_widget.raw_scale = 1.0.to_string();
    }
}
impl ValueWidget for InputSpaceAxisWidget{
    type Value<'v> = modelrdf::SpaceInputAxis;
    fn set_value(&mut self, value: modelrdf::SpaceInputAxis){
        self.id_widget.set_value(value.id);
        self.description_widget.set_value(value.description);
        self.size_widget.set_value(value.size);
        self.physical_size_widget.set_value((value.scale, value.unit));
    }
}

impl StatefulWidget for InputSpaceAxisWidget{
    type Value<'p> = Result<modelrdf::SpaceInputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                axis_id_label(ui);
                self.id_widget.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                axis_description_label(ui);
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Size: ");
                self.size_widget.draw_and_parse(ui, id.with("size"));
            });
            self.physical_size_widget.draw_and_parse(ui, id.with("physical_size"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let (scale, unit) = self.physical_size_widget.state()?;
        Ok(modelrdf::SpaceInputAxis {
            id: self.id_widget.state()?.clone(),
            description: self.description_widget.state()?.clone(),
            size: self.size_widget.state()?,
            scale,
            unit
        })
    }
}

#[derive(Default, Restore)]
pub struct InputTimeAxisWidget {
    pub id_widget: StagingString<modelrdf::axes::AxisId>,
    pub description_widget: StagingString<BoundedString<0, 128>>,

    pub size_widget: AnyAxisSizeWidget,
    pub physical_size_widget: PhysicalSizeWidget<modelrdf::TimeUnit>,
}

impl ValueWidget for InputTimeAxisWidget{
    type Value<'v> = modelrdf::TimeInputAxis;
    fn set_value(&mut self, value: modelrdf::TimeInputAxis){
        self.id_widget.set_value(value.id);
        self.description_widget.set_value(value.description);
        self.size_widget.set_value(value.size);
        self.physical_size_widget.set_value((value.scale, value.unit));
    }
}


impl StatefulWidget for InputTimeAxisWidget{
    type Value<'p> = Result<modelrdf::TimeInputAxis>;

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
                self.size_widget.draw_and_parse(ui, id.with("size"));
            });
            self.physical_size_widget.draw_and_parse(ui, id.with("physical_size"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let (scale, unit) = self.physical_size_widget.state()?;
        Ok(modelrdf::TimeInputAxis {
            id: self.id_widget.state()?.clone(),
            description: self.description_widget.state()?.clone(),
            size: self.size_widget.state()?,
            scale,
            unit,
        })
    }
}

#[derive(Default, Restore)]
pub struct InputAxisWidget {
    pub axis_type_widget: SearchAndPickWidget<AxisType>,
    pub batch_axis_widget: BatchAxisWidget,
    pub channel_axis_widget: ChannelAxisWidget,
    pub index_axis_widget: IndexAxisWidget,
    pub space_axis_widget: InputSpaceAxisWidget,
    pub time_axis_widget: InputTimeAxisWidget,
}

impl InputAxisWidget{
    pub fn new(value: Option<modelrdf::InputAxis>) -> Self{
        let mut out = Self::default();
        if let Some(val) = value{
            out.set_value(val);
        }
        out
    }
}

impl ValueWidget for InputAxisWidget{
    type Value<'v> = modelrdf::InputAxis;
    fn set_value(&mut self, value: modelrdf::InputAxis){
        match value{
            modelrdf::InputAxis::Batch(axis) => {
                self.axis_type_widget.set_value(AxisType::Batch);
                self.batch_axis_widget.set_value(axis);
            },
            modelrdf::InputAxis::Channel(axis) => {
                self.axis_type_widget.set_value(AxisType::Channel);
                self.channel_axis_widget.set_value(axis);
            },
            modelrdf::InputAxis::Index(axis) => {
                self.axis_type_widget.set_value(AxisType::Index);
                self.index_axis_widget.set_value(axis);
            },
            modelrdf::InputAxis::Space(axis) => {
                self.axis_type_widget.set_value(AxisType::Space);
                self.space_axis_widget.set_value(axis);
            },
            modelrdf::InputAxis::Time(axis) => {
                self.axis_type_widget.set_value(AxisType::Time);
                self.time_axis_widget.set_value(axis);
            },
        }
    }
}

impl ItemWidgetConf for InputAxisWidget{
    const ITEM_NAME: &'static str = "Input Axis";
}

impl ItemWidgetConf for CollapsibleWidget<InputAxisWidget>{
    const ITEM_NAME: &'static str = "Input Axis";
    const GROUP_FRAME: bool = false;
}

impl SummarizableWidget for InputAxisWidget{
    fn summarize(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        match self.state(){
            Ok(axis) => {
                ui.label(axis.to_string());
            },
            Err(err) => {
                let rich_text = egui::RichText::new(err.to_string()).color(egui::Color32::RED);
                ui.label(rich_text);
            }
        }
    }
}

impl StatefulWidget for InputAxisWidget{
    type Value<'p> = Result<modelrdf::InputAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Type: ");
                self.axis_type_widget.draw_and_parse(ui, id.with("axis_type".as_ptr()));
            });
            match self.axis_type_widget.value{
                AxisType::Space => self.space_axis_widget.draw_and_parse(ui, id.with("space")),
                AxisType::Time => self.time_axis_widget.draw_and_parse(ui, id.with("time")),
                AxisType::Batch => self.batch_axis_widget.draw_and_parse(ui, id.with("batch")),
                AxisType::Channel => self.channel_axis_widget.draw_and_parse(ui, id.with("channel")),
                AxisType::Index => self.index_axis_widget.draw_and_parse(ui, id.with("index")),
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.axis_type_widget.value{
            AxisType::Space => modelrdf::InputAxis::Space(self.space_axis_widget.state()?),
            AxisType::Time => modelrdf::InputAxis::Time(self.time_axis_widget.state()?),
            AxisType::Batch => modelrdf::InputAxis::Batch(self.batch_axis_widget.state()?),
            AxisType::Channel => modelrdf::InputAxis::Channel(self.channel_axis_widget.state()?),
            AxisType::Index => modelrdf::InputAxis::Index(self.index_axis_widget.state()?),
        })
    }
}
