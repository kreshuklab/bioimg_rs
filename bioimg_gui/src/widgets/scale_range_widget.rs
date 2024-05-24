
use bioimg_spec::rdf::model::{self, preprocessing::PreprocessingEpsilon};
use bioimg_spec::rdf::model::{preprocessing as modelrdfpreproc, TensorId};



use crate::result::{GuiError, Result, VecResultExt};
use super::staging_vec::ItemWidgetConf;
use super::ValueWidget;
use super::{staging_num::StagingNum, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::StagingVec, StatefulWidget};

pub struct PercentilesWidget{
    pub min_widget: StagingNum<f32, f32>,
    pub max_widget: StagingNum<f32, f32>,
    pub parsed: Result<modelrdfpreproc::ScaleRangePercentile>,
}

impl ValueWidget for PercentilesWidget{
    type Value<'v> = modelrdfpreproc::scale_range::ScaleRangePercentile;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.min_widget.set_value(value.min());
        self.max_widget.set_value(value.max());
    }
}

impl Default for PercentilesWidget{
    fn default() -> Self {
        Self{
            min_widget: StagingNum::new_with_raw(0.0),
            max_widget: StagingNum::new_with_raw(100.0),
            parsed: Err(GuiError::new("empty".to_owned())),
        }
    }
}

impl StatefulWidget for PercentilesWidget{
    type Value<'p>  = &'p Result<modelrdfpreproc::ScaleRangePercentile>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Min Percentile: ");
            self.min_widget.draw_and_parse(ui, id.with("min".as_ptr()));
            ui.strong("Max Percentile: ");
            self.max_widget.draw_and_parse(ui, id.with("max".as_ptr()));
        });
        self.parsed = || -> Result<modelrdfpreproc::ScaleRangePercentile>{
            Ok(modelrdfpreproc::ScaleRangePercentile::try_from_min_max(
                self.min_widget.state()?,
                self.max_widget.state()?,
            )?)
        }();
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        &self.parsed
    }
}

pub struct AxesItemConfig;
impl ItemWidgetConf for AxesItemConfig{
    const ITEM_NAME: &'static str = "Axis Id";
    const INLINE_ITEM: bool = true;
    const MIN_NUM_ITEMS: usize = 1;
}

#[derive(Default)]
pub struct ScaleRangeWidget{
    pub axes_widget: StagingOpt<StagingVec<StagingString<model::AxisId>, AxesItemConfig>>,
    pub percentiles_widget: PercentilesWidget,
    pub epsilon_widget: StagingNum<f32, PreprocessingEpsilon>,
    pub reference_tensor: StagingOpt<StagingString<TensorId>>,
}

impl ValueWidget for ScaleRangeWidget{
    type Value<'v> = modelrdfpreproc::ScaleRangeDescr;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.axes_widget.set_value(value.axes);
        self.percentiles_widget.set_value(value.percentiles);
        self.epsilon_widget.set_value(value.eps);
        self.reference_tensor.set_value(value.reference_tensor);
    }
}

impl StatefulWidget for ScaleRangeWidget{
    type Value<'p> = Result<modelrdfpreproc::ScaleRangeDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Axes: ");
                self.axes_widget.draw_and_parse(ui, id.with("axes_widget".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Percentiles: ");
                self.percentiles_widget.draw_and_parse(ui, id.with("percentiles_widget".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Epsilon: ");
                self.epsilon_widget.draw_and_parse(ui, id.with("epsilong_widget".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Reference Tensor: ");
                self.reference_tensor.draw_and_parse(ui, id.with("reference_tensor".as_ptr()));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdfpreproc::ScaleRangeDescr{
            axes: self.axes_widget.state().map(|val| val.collect_result()).transpose()?,
            percentiles: self.percentiles_widget.state().as_ref().map_err(|err| err.clone())?.clone(),
            eps: self.epsilon_widget.state()?,
            reference_tensor: self.reference_tensor.state().transpose()?,
        })
    }
}
