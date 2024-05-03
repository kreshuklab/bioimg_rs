use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;

use crate::result::{GuiError, Result, VecResultExt};
use super::{error_display::show_if_error, staging_num::StagingNum, staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}, StatefulWidget};

#[derive(PartialEq, Eq, Default)]
pub enum ScaleLinearMode{
    #[default]
    Simple,
    AlongAxis,
}

pub struct SimpleScaleLinearWidget{
    pub gain_widget: StagingNum<f32, f32>,
    pub offset_widget: StagingNum<f32, f32>,
}

impl Default for SimpleScaleLinearWidget{
    fn default() -> Self {
        Self {
            gain_widget: StagingNum::new_with_raw(1.0),
            offset_widget: StagingNum::new_with_raw(0.0),
        }
    }
}

impl StatefulWidget for SimpleScaleLinearWidget{
    type Value<'p> = Result<modelrdfpreproc::SimpleScaleLinearDescr>;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Gain: ");
            self.gain_widget.draw_and_parse(ui, id.with("gain"));
            ui.strong(" Offset: ");
            self.offset_widget.draw_and_parse(ui, id.with("off"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdfpreproc::SimpleScaleLinearDescr{
            gain: self.gain_widget.state()?,
            offset: self.offset_widget.state()?,
        })
    }
}

// ///////////////////////////////////

pub struct GainOffsetItemConfig;
impl ItemWidgetConf for GainOffsetItemConfig{
    const ITEM_NAME: &'static str = "Gain & Offset";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

pub struct ScaleLinearAlongAxisWidget{
    pub axis_widget: StagingString<modelrdf::axes::NonBatchAxisId>,
    pub gain_offsets_widget: StagingVec<SimpleScaleLinearWidget, GainOffsetItemConfig>,
    pub parsed: Result<modelrdfpreproc::ScaleLinearAlongAxisDescr>,
}

impl Default for ScaleLinearAlongAxisWidget{
    fn default() -> Self {
        Self {
            axis_widget: Default::default(),
            gain_offsets_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned()))
        }
    }
}

impl StatefulWidget for ScaleLinearAlongAxisWidget{
    type Value<'p> = &'p Result<modelrdfpreproc::ScaleLinearAlongAxisDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Axis");
                self.axis_widget.draw_and_parse(ui, id.with("ax".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Gains and Offsets:");
                self.gain_offsets_widget.draw_and_parse(ui, id.with("go".as_ptr()));
            });
            self.parsed = || -> Result<modelrdfpreproc::ScaleLinearAlongAxisDescr>{
                Ok(modelrdfpreproc::ScaleLinearAlongAxisDescr{
                    axis: self.axis_widget.state()?,
                    gain_offsets: self.gain_offsets_widget.state()
                        .collect_result()?
                        .iter()
                        .map(|simple| (simple.gain, simple.offset))
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
pub struct ScaleLinearWidget{
    pub mode: ScaleLinearMode,
    pub simple_widget: SimpleScaleLinearWidget,
    pub along_axis_widget: ScaleLinearAlongAxisWidget,
}

impl StatefulWidget for ScaleLinearWidget{
    type Value<'p> = Result<modelrdfpreproc::ScaleLinearDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Mode: ");
                ui.radio_value(&mut self.mode, ScaleLinearMode::Simple, "Simple");
                ui.radio_value(&mut self.mode, ScaleLinearMode::AlongAxis, "Along Axis");
            });
            match self.mode{
                ScaleLinearMode::Simple => self.simple_widget.draw_and_parse(ui, id.with("simple".as_ptr())),
                ScaleLinearMode::AlongAxis => self.along_axis_widget.draw_and_parse(ui, id.with("along axis".as_ptr())),
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode{
            ScaleLinearMode::Simple => modelrdfpreproc::ScaleLinearDescr::Simple(
                self.simple_widget.state()?
            ),
            ScaleLinearMode::AlongAxis => modelrdfpreproc::ScaleLinearDescr::AlongAxis(
                self.along_axis_widget.state().as_ref().map_err(|err| err.clone())?.clone()
            )
        })
    }
}
