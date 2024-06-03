use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;
use bioimg_spec::rdf::model::postprocessing as postproc;
use bioimg_spec::rdf::model as modelrdf;

use crate::result::Result;
use super::scale_mean_variance_widget::ScaleMeanVarianceWidget;
use super::{binarize_widget::BinarizePreprocessingWidget, clip_widget::ClipWidget, fixed_zero_mean_unit_variance_widget::FixedZmuvWidget, scale_linear_widget::ScaleLinearWidget, scale_range_widget::ScaleRangeWidget, search_and_pick_widget::SearchAndPickWidget, staging_vec::ItemWidgetConf, zero_mean_unit_variance_widget::ZeroMeanUnitVarianceWidget, StatefulWidget, ValueWidget};

#[derive(PartialEq, Eq, Default, Clone, strum::VariantArray, strum::AsRefStr, strum::VariantNames, strum::Display)]
pub enum PostprocessingWidgetMode {
    #[default]
    Binarize,
    Clip,
    #[strum(serialize="Scale Linear")]
    ScaleLinear,
    Sigmoid,
    #[strum(serialize="Zero-Mean, Unit-Variance")]
    ZeroMeanUnitVariance,
    #[strum(serialize="Scale Range")]
    ScaleRange,
    #[strum(serialize="Ensure Data Type")]
    EnsureDtype,
    #[strum(serialize="Fixed Zero-Mean, Unit-Variance")]
    FixedZmuv,
    #[strum(serialize="Scale Mean Variance")]
    ScaleMeanVariance,
}

#[derive(Default)]
pub struct PostprocessingWidget{
    pub mode_widget: SearchAndPickWidget<PostprocessingWidgetMode>,
    pub binarize_widget: BinarizePreprocessingWidget,
    pub clip_widget: ClipWidget,
    pub scale_linear_widget: ScaleLinearWidget,
    // pub sigmoid sigmoid has no widget since it has no params
    pub zero_mean_unit_variance_widget: ZeroMeanUnitVarianceWidget,
    pub scale_range_widget: ScaleRangeWidget,
    pub ensure_dtype_widget: SearchAndPickWidget<modelrdf::DataType>,
    pub fixed_zmuv_widget: FixedZmuvWidget,
    pub scale_mean_var_widget: ScaleMeanVarianceWidget,
}

impl ValueWidget for PostprocessingWidget{
    type Value<'v> = postproc::PostprocessingDescr;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            postproc::PostprocessingDescr::Binarize(binarize) => {
                self.mode_widget.value = PostprocessingWidgetMode::Binarize;
                self.binarize_widget.set_value(binarize)
            },
            postproc::PostprocessingDescr::Clip(clip) => {
                self.mode_widget.value = PostprocessingWidgetMode::Clip;
                self.clip_widget.set_value(clip)
            },
            postproc::PostprocessingDescr::ScaleLinear(scale_linear) => {
                self.mode_widget.value = PostprocessingWidgetMode::ScaleLinear;
                self.scale_linear_widget.set_value(scale_linear);
            },
            postproc::PostprocessingDescr::Sigmoid(_) => {
                self.mode_widget.value = PostprocessingWidgetMode::Sigmoid;
            },
            postproc::PostprocessingDescr::ZeroMeanUnitVariance(val) => {
                self.mode_widget.value = PostprocessingWidgetMode::ZeroMeanUnitVariance;
                self.zero_mean_unit_variance_widget.set_value(val);
            },
            postproc::PostprocessingDescr::ScaleRange(val) => {
                self.mode_widget.value = PostprocessingWidgetMode::ScaleRange;
                self.scale_range_widget.set_value(val);
            },
            postproc::PostprocessingDescr::EnsureDtype(val) => {
                self.mode_widget.value = PostprocessingWidgetMode::EnsureDtype;
                self.ensure_dtype_widget.set_value(val.dtype);
            },
            postproc::PostprocessingDescr::FixedZeroMeanUnitVariance(val) => {
                self.mode_widget.value = PostprocessingWidgetMode::FixedZmuv;
                self.fixed_zmuv_widget.set_value(val);
            },
            postproc::PostprocessingDescr::ScaleMeanVarianceDescr(val) => {
                self.mode_widget.value = PostprocessingWidgetMode::ScaleMeanVariance;
                self.scale_mean_var_widget.set_value(val);
            }
        }
    }
}

impl ItemWidgetConf for PostprocessingWidget{
    const ITEM_NAME: &'static str = "Preprocessing";
}

impl StatefulWidget for PostprocessingWidget{
    type Value<'p> = Result<postproc::PostprocessingDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Preprocessing Type: ");
                self.mode_widget.draw_and_parse(ui, id.with("preproc type".as_ptr()));
            });
            match self.mode_widget.value{
                PostprocessingWidgetMode::Binarize => {
                    self.binarize_widget.draw_and_parse(ui, id.with("binarize_widget".as_ptr()))
                },
                PostprocessingWidgetMode::Clip => {
                    self.clip_widget.draw_and_parse(ui, id.with("clip_widget".as_ptr()))
                },
                PostprocessingWidgetMode::ScaleLinear => {
                    self.scale_linear_widget.draw_and_parse(ui, id.with("scale_linear_widget".as_ptr()))
                },
                PostprocessingWidgetMode::Sigmoid => {
                    ()
                },
                PostprocessingWidgetMode::ZeroMeanUnitVariance => {
                    self.zero_mean_unit_variance_widget.draw_and_parse(ui, id.with("zero_mean_unit_variance_widget".as_ptr()))
                },
                PostprocessingWidgetMode::ScaleRange => {
                    self.scale_range_widget.draw_and_parse(ui, id.with("scale_range_widget".as_ptr()))
                },
                PostprocessingWidgetMode::EnsureDtype => {
                    ui.horizontal(|ui|{
                        ui.strong("Data Type: ");
                        self.ensure_dtype_widget.draw_and_parse(ui, id.with("ensure_dtype".as_ptr()))
                    });
                },
                PostprocessingWidgetMode::FixedZmuv => {
                    self.fixed_zmuv_widget.draw_and_parse(ui, id.with("fixed_zmuv".as_ptr()) )
                },
                PostprocessingWidgetMode::ScaleMeanVariance => {
                    self.scale_mean_var_widget.draw_and_parse(ui, id.with("scale_mean_var".as_ptr()))
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode_widget.value{
            PostprocessingWidgetMode::Binarize => {
                postproc::PostprocessingDescr::Binarize(self.binarize_widget.state()?)
            },
            PostprocessingWidgetMode::Clip => {
                postproc::PostprocessingDescr::Clip(
                    self.clip_widget.state().as_ref().map_err(|err| err.clone())?.clone()
                )
            },
            PostprocessingWidgetMode::ScaleLinear => {
                postproc::PostprocessingDescr::ScaleLinear(
                    self.scale_linear_widget.state()?
                )
            },
            PostprocessingWidgetMode::Sigmoid => {
                postproc::PostprocessingDescr::Sigmoid(modelrdfpreproc::Sigmoid)
            },
            PostprocessingWidgetMode::ZeroMeanUnitVariance => {
                postproc::PostprocessingDescr::ZeroMeanUnitVariance(
                    self.zero_mean_unit_variance_widget.state()?
                )
            },
            PostprocessingWidgetMode::ScaleRange => {
                postproc::PostprocessingDescr::ScaleRange(
                    self.scale_range_widget.state()?
                )
            },
            PostprocessingWidgetMode::EnsureDtype => {
                postproc::PostprocessingDescr::EnsureDtype(modelrdfpreproc::EnsureDtype {
                    dtype: self.ensure_dtype_widget.state()
                })
            },
            PostprocessingWidgetMode::FixedZmuv => {
                postproc::PostprocessingDescr::FixedZeroMeanUnitVariance(
                    self.fixed_zmuv_widget.state()?
                )
            },
            PostprocessingWidgetMode::ScaleMeanVariance => {
                postproc::PostprocessingDescr::ScaleMeanVarianceDescr(
                    self.scale_mean_var_widget.state()?
                )
            },
        })
    }
}
