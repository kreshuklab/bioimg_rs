use bioimg_spec::rdf::model::{self as modelrdf, preprocessing::zero_mean_unit_variance::ZmuvStdDeviation};
use bioimg_spec::rdf::model::preprocessing as preproc;

use crate::result::{GuiError, Result, VecResultExt};
use super::staging_float::StagingFloat;
use super::{error_display::show_if_error, staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}, StatefulWidget, ValueWidget};

#[derive(PartialEq, Eq, Default)]
pub enum Mode{
    #[default]
    Simple,
    AlongAxis,
}

pub struct SimpleFixedZmuvWidget{
    pub mean_widget: StagingFloat<f32>,
    pub std_widget: StagingFloat<ZmuvStdDeviation>,
}

impl ValueWidget for SimpleFixedZmuvWidget{
    type Value<'v> = preproc::SimpleFixedZmuv;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.mean_widget.set_value(value.mean);
        self.std_widget.set_value(value.std);
    }
}

impl Default for SimpleFixedZmuvWidget{
    fn default() -> Self {
        Self {
            mean_widget: StagingFloat::new_with_raw(1.0),
            std_widget: StagingFloat::new_with_raw(0.0),
        }
    }
}

impl StatefulWidget for SimpleFixedZmuvWidget{
    type Value<'p> = Result<preproc::SimpleFixedZmuv>;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Mean: ");
            self.mean_widget.draw_and_parse(ui, id.with("mean".as_ptr()));
            ui.strong(" Standard Deviation: ");
            self.std_widget.draw_and_parse(ui, id.with("std".as_ptr()));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(preproc::SimpleFixedZmuv{
            mean: self.mean_widget.state()?,
            std: self.std_widget.state()?,
        })
    }
}

// ///////////////////////////////////

pub struct MeanAndStdItemConfig;
impl ItemWidgetConf for MeanAndStdItemConfig{
    const ITEM_NAME: &'static str = "Stats for for Axis ";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

pub struct FixedZmuvAlongAxisWidget{
    pub axis_widget: StagingString<modelrdf::axes::NonBatchAxisId>,
    pub mean_and_std_widget: StagingVec<SimpleFixedZmuvWidget, MeanAndStdItemConfig>,
    pub parsed: Result<preproc::FixedZmuvAlongAxis>,
}

impl ValueWidget for FixedZmuvAlongAxisWidget{
    type Value<'v> = preproc::FixedZmuvAlongAxis;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.axis_widget.set_value(value.axis);
        self.mean_and_std_widget.set_value(
            value.mean_and_std.into_inner().into_iter()
                .map(|simple| preproc::SimpleFixedZmuv{mean: simple.mean, std: simple.std})
                .collect()
        );
    }
}

impl Default for FixedZmuvAlongAxisWidget{
    fn default() -> Self {
        Self {
            axis_widget: Default::default(),
            mean_and_std_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned()))
        }
    }
}

impl StatefulWidget for FixedZmuvAlongAxisWidget{
    type Value<'p> = &'p Result<preproc::FixedZmuvAlongAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Axis");
                self.axis_widget.draw_and_parse(ui, id.with("ax".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Gains and Offsets:");
                self.mean_and_std_widget.draw_and_parse(ui, id.with("go".as_ptr()));
            });
            self.parsed = || -> Result<preproc::FixedZmuvAlongAxis>{
                Ok(preproc::FixedZmuvAlongAxis{
                    axis: self.axis_widget.state()?,
                    mean_and_std: self.mean_and_std_widget.state()
                        .collect_result()?
                        .iter()
                        .map(|simple| preproc::SimpleFixedZmuv{mean: simple.mean, std: simple.std})
                        .collect::<Vec<_>>()
                        .try_into()
                        .map_err(|_| GuiError::new("Could not create a non-empty list of Gain + Offsets".to_owned()))?
                })
            }();
            show_if_error(ui, &self.parsed)
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}

// //////////////////////////

#[derive(Default)]
pub struct FixedZmuvWidget{
    pub mode: Mode,
    pub simple_widget: SimpleFixedZmuvWidget,
    pub along_axis_widget: FixedZmuvAlongAxisWidget,
}

impl ValueWidget for FixedZmuvWidget{
    type Value<'v> = preproc::FixedZmuv;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            preproc::FixedZmuv::Simple(simple) => {
                self.mode = Mode::Simple;
                self.simple_widget.set_value(simple)
            },
            preproc::FixedZmuv::AlongAxis(val) => {
                self.mode = Mode::AlongAxis;
                self.along_axis_widget.set_value(val)
            },
        }
    }
}

impl StatefulWidget for FixedZmuvWidget{
    type Value<'p> = Result<preproc::FixedZmuv>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Mode: ");
                ui.radio_value(&mut self.mode, Mode::Simple, "Simple");
                ui.radio_value(&mut self.mode, Mode::AlongAxis, "Along Axis");
            });
            match self.mode{
                Mode::Simple => self.simple_widget.draw_and_parse(ui, id.with("simple".as_ptr())),
                Mode::AlongAxis => self.along_axis_widget.draw_and_parse(ui, id.with("along axis".as_ptr())),
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode{
            Mode::Simple => preproc::FixedZmuv::Simple(
                self.simple_widget.state()?
            ),
            Mode::AlongAxis => preproc::FixedZmuv::AlongAxis(
                self.along_axis_widget.state().as_ref().map_err(|err| err.clone())?.clone()
            )
        })
    }
}
