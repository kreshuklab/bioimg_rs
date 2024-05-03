use bioimg_spec::rdf::{model as modelrdf, NonEmptyList};

use crate::result::{GuiError, Result, VecResultExt};

use super::{staging_num::StagingNum, staging_string::StagingString, staging_vec::{ItemWidgetConf, StagingVec}, StatefulWidget};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum BinarizeMode{
    Simple,
    AlongAxis,
}

#[derive(Default)]
pub struct SimpleBinarizeWidget{
    pub threshold_widget: StagingNum<f32, f32>,
}

impl StatefulWidget for SimpleBinarizeWidget{
    type Value<'p> = Result<modelrdf::preprocessing::SimpleBinarizeDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Threshold: ");
            self.threshold_widget.draw_and_parse(ui, id.with("threshold"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::preprocessing::SimpleBinarizeDescr{threshold: self.threshold_widget.state()?})
    }
}

pub struct ThresholdsItemWidgetConf;
impl ItemWidgetConf for ThresholdsItemWidgetConf{
    const ITEM_NAME: &'static str = "Threshold";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

#[derive(Default)]
pub struct BinarizeAlongAxisWidget{
    pub thresholds_widget: StagingVec<StagingNum<f32, f32>, ThresholdsItemWidgetConf>,
    pub axis_id_widget: StagingString<modelrdf::axes::NonBatchAxisId>
}

impl StatefulWidget for BinarizeAlongAxisWidget{
    type Value<'p> = Result<modelrdf::preprocessing::BinarizeAlongAxisDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Thresholds: ");
            self.thresholds_widget.draw_and_parse(ui, id.with("ts"));
        });
        ui.horizontal(|ui|{
            ui.strong("Axis Id: ");
            self.axis_id_widget.draw_and_parse(ui, id.with("id"))
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let thresholds: NonEmptyList<f32> = self.thresholds_widget.state()
            .collect_result()?
            .try_into()
            .map_err(|_| GuiError::new("Could not make a non-empty list".into()))?;
        Ok(modelrdf::preprocessing::BinarizeAlongAxisDescr{
            axis: self.axis_id_widget.state()?,
            threshold: thresholds
        })
    }
}

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
                self.binarize_along_axis_wiget.state()?
            ),
        })
    }
}



