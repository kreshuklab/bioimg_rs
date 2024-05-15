use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;
use bioimg_spec::rdf::model as modelrdf;

use crate::result::Result;
use super::{binarize_widget::BinarizePreprocessingWidget, clip_widget::ClipWidget, enum_widget::EnumWidget, scale_linear_widget::ScaleLinearWidget, scale_range_widget::ScaleRangeWidget, staging_vec::ItemWidgetConf, zero_mean_unit_variance_widget::ZeroMeanUnitVarianceWidget, StatefulWidget};

#[derive(PartialEq, Eq, Default)]
pub enum PreprocessingWidgetMode {
    #[default]
    Binarize,
    Clip,
    ScaleLinear,
    Sigmoid,
    ZeroMeanUnitVariance,
    ScaleRange,
    EnsureDtype,
}

#[derive(Default)]
pub struct PreprocessingWidget{
    pub mode: PreprocessingWidgetMode,
    pub binarize_widget: BinarizePreprocessingWidget,
    pub clip_widget: ClipWidget,
    pub scale_linear_widget: ScaleLinearWidget,
    // pub sigmoid sigmoid has no widget since it has no params
    pub zero_mean_unit_variance_widget: ZeroMeanUnitVarianceWidget,
    pub scale_range_widget: ScaleRangeWidget,
    pub ensure_dtype_widget: EnumWidget<modelrdf::DataType>,
}

impl ItemWidgetConf for PreprocessingWidget{
    const ITEM_NAME: &'static str = "Preprocessing";
}

impl StatefulWidget for PreprocessingWidget{
    type Value<'p> = Result<modelrdfpreproc::PreprocessingDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui|{
            ui.strong("Preprocessing Type: ");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::Binarize, "Binarize");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::Clip, "Clip");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::ScaleLinear, "Scale Linear");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::Sigmoid, "Sigmoid");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::ZeroMeanUnitVariance, "Zero-mean Unit-variance");
            ui.radio_value(&mut self.mode, PreprocessingWidgetMode::ScaleRange, "Scale Range");
        });

        match self.mode{
            PreprocessingWidgetMode::Binarize => {
                self.binarize_widget.draw_and_parse(ui, id.with("binarize_widget".as_ptr()))
            },
            PreprocessingWidgetMode::Clip => {
                self.clip_widget.draw_and_parse(ui, id.with("clip_widget".as_ptr()))
            },
            PreprocessingWidgetMode::ScaleLinear => {
                self.scale_linear_widget.draw_and_parse(ui, id.with("scale_linear_widget".as_ptr()))
            },
            PreprocessingWidgetMode::Sigmoid => {
                ()
            },
            PreprocessingWidgetMode::ZeroMeanUnitVariance => {
                self.zero_mean_unit_variance_widget.draw_and_parse(ui, id.with("zero_mean_unit_variance_widget".as_ptr()))
            },
            PreprocessingWidgetMode::ScaleRange => {
                self.scale_range_widget.draw_and_parse(ui, id.with("scale_range_widget".as_ptr()))
            },
            PreprocessingWidgetMode::EnsureDtype => {
                self.ensure_dtype_widget.draw_and_parse(ui, id.with("ensure_dtype".as_ptr()))
            },
        }
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode{
            PreprocessingWidgetMode::Binarize => {
                modelrdfpreproc::PreprocessingDescr::Binarize(self.binarize_widget.state()?)
            },
            PreprocessingWidgetMode::Clip => {
                modelrdfpreproc::PreprocessingDescr::Clip(
                    self.clip_widget.state().as_ref().map_err(|err| err.clone())?.clone()
                )
            },
            PreprocessingWidgetMode::ScaleLinear => {
                modelrdfpreproc::PreprocessingDescr::ScaleLinear(
                    self.scale_linear_widget.state()?
                )
            },
            PreprocessingWidgetMode::Sigmoid => {
                modelrdfpreproc::PreprocessingDescr::Sigmoid(modelrdfpreproc::Sigmoid)
            },
            PreprocessingWidgetMode::ZeroMeanUnitVariance => {
                modelrdfpreproc::PreprocessingDescr::ZeroMeanUnitVariance(
                    self.zero_mean_unit_variance_widget.state()?
                )
            },
            PreprocessingWidgetMode::ScaleRange => {
                modelrdfpreproc::PreprocessingDescr::ScaleRange(
                    self.scale_range_widget.state()?
                )
            },
            PreprocessingWidgetMode::EnsureDtype => {
                modelrdfpreproc::PreprocessingDescr::EnsureDtype(modelrdfpreproc::EnsureDtype{
                    dtype: self.ensure_dtype_widget.state()
                })
            }
        })
    }
}
