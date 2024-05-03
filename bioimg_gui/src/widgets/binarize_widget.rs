use bioimg_spec::rdf::{model as modelrdf, NonEmptyList};
use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;

use crate::result::{GuiError, Result, VecResultExt};

use super::error_display::show_if_error;
use super::{staging_num::StagingNum, staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}, StatefulWidget};

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum BinarizeMode{
    #[default]
    Simple,
    AlongAxis,
}

#[derive(Default)]
pub struct SimpleBinarizeWidget{
    pub threshold_widget: StagingNum<f32, f32>,
}

impl StatefulWidget for SimpleBinarizeWidget{
    type Value<'p> = Result<modelrdfpreproc::SimpleBinarizeDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Threshold: ");
            self.threshold_widget.draw_and_parse(ui, id.with("threshold"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdfpreproc::SimpleBinarizeDescr{threshold: self.threshold_widget.state()?})
    }
}

pub struct ThresholdsItemWidgetConf;
impl ItemWidgetConf for ThresholdsItemWidgetConf{
    const ITEM_NAME: &'static str = "Threshold";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

pub struct BinarizeAlongAxisWidget{
    pub thresholds_widget: StagingVec<StagingNum<f32, f32>, ThresholdsItemWidgetConf>,
    pub axis_id_widget: StagingString<modelrdf::axes::NonBatchAxisId>,
    pub parsed: Result<modelrdfpreproc::BinarizeAlongAxisDescr>,
}

impl Default for BinarizeAlongAxisWidget{
    fn default() -> Self {
        BinarizeAlongAxisWidget{
            thresholds_widget: Default::default(),
            axis_id_widget: Default::default(),
            parsed: Err(GuiError::new("empty".to_owned())),
        }
    }
}

impl StatefulWidget for BinarizeAlongAxisWidget{
    type Value<'p> = &'p Result<modelrdf::preprocessing::BinarizeAlongAxisDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Thresholds: ");
            self.thresholds_widget.draw_and_parse(ui, id.with("ts"));
        });
        ui.horizontal(|ui|{
            ui.strong("Axis Id: ");
            self.axis_id_widget.draw_and_parse(ui, id.with("id"))
        });
        self.parsed = || -> Result<modelrdfpreproc::BinarizeAlongAxisDescr> {
            let thresholds: NonEmptyList<f32> = self.thresholds_widget.state()
                .collect_result()?
                .try_into()
                .map_err(|_| GuiError::new("Could not make a non-empty list".into()))?;
            Ok(modelrdfpreproc::BinarizeAlongAxisDescr{
                axis: self.axis_id_widget.state()?,
                threshold: thresholds
            })
        }();
        show_if_error(ui, &self.parsed);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}

#[derive(Default)]
pub struct BinarizePreprocessingWidget{
    pub mode: BinarizeMode,
    pub simple_binarize_widget: SimpleBinarizeWidget,
    pub binarize_along_axis_wiget: BinarizeAlongAxisWidget,
}

impl StatefulWidget for BinarizePreprocessingWidget{
    type Value<'p> = Result<modelrdf::preprocessing::BinarizeDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Mode: ");
            ui.radio_value(&mut self.mode, BinarizeMode::Simple, "Simple");
            ui.radio_value(&mut self.mode, BinarizeMode::AlongAxis, "Along Axis");
        });
        match self.mode{
            BinarizeMode::Simple => self.simple_binarize_widget.draw_and_parse(ui, id.with("simp")),
            BinarizeMode::AlongAxis => self.binarize_along_axis_wiget.draw_and_parse(ui, id.with("axis")),
        }
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode{
            BinarizeMode::Simple => modelrdf::preprocessing::BinarizeDescr::Simple(
                self.simple_binarize_widget.state()?
            ),
            BinarizeMode::AlongAxis => modelrdf::preprocessing::BinarizeDescr::AlongAxis(
                self.binarize_along_axis_wiget.state().as_ref().map_err(|err| err.clone())?.clone()
            ),
        })
    }
}



