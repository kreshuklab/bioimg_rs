use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;
use bioimg_spec::rdf::model as modelrdf;

use crate::result::Result;
use super::{binarize_widget::BinarizePreprocessingWidget, clip_widget::ClipWidget, fixed_zero_mean_unit_variance_widget::FixedZmuvWidget, scale_linear_widget::ScaleLinearWidget, scale_range_widget::ScaleRangeWidget, search_and_pick_widget::SearchAndPickWidget, staging_vec::ItemWidgetConf, zero_mean_unit_variance_widget::ZeroMeanUnitVarianceWidget, StatefulWidget, ValueWidget};

#[derive(PartialEq, Eq, Default, Clone, strum::VariantArray, strum::AsRefStr, strum::VariantNames, strum::Display)]
pub enum PreprocessingWidgetMode {
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
}

#[derive(Default)]
pub struct PreprocessingWidget{
    pub mode_widget: SearchAndPickWidget<PreprocessingWidgetMode>,
    pub binarize_widget: BinarizePreprocessingWidget,
    pub clip_widget: ClipWidget,
    pub scale_linear_widget: ScaleLinearWidget,
    // pub sigmoid sigmoid has no widget since it has no params
    pub zero_mean_unit_variance_widget: ZeroMeanUnitVarianceWidget,
    pub scale_range_widget: ScaleRangeWidget,
    pub ensure_dtype_widget: SearchAndPickWidget<modelrdf::DataType>,
    pub fixed_zmuv_widget: FixedZmuvWidget,
}

impl ValueWidget for PreprocessingWidget{
    type Value<'v> = modelrdfpreproc::PreprocessingDescr;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        match value{
            modelrdf::PreprocessingDescr::Binarize(binarize) => {
                self.mode_widget.value = PreprocessingWidgetMode::Binarize;
                self.binarize_widget.set_value(binarize)
            },
            modelrdf::PreprocessingDescr::Clip(clip) => {
                self.mode_widget.value = PreprocessingWidgetMode::Clip;
                self.clip_widget.set_value(clip)
            },
            modelrdf::PreprocessingDescr::ScaleLinear(scale_linear) => {
                self.mode_widget.value = PreprocessingWidgetMode::ScaleLinear;
                self.scale_linear_widget.set_value(scale_linear);
            },
            modelrdf::PreprocessingDescr::Sigmoid(_) => {
                self.mode_widget.value = PreprocessingWidgetMode::Sigmoid;
            },
            modelrdf::PreprocessingDescr::ZeroMeanUnitVariance(val) => {
                self.mode_widget.value = PreprocessingWidgetMode::ZeroMeanUnitVariance;
                self.zero_mean_unit_variance_widget.set_value(val);
            },
            modelrdf::PreprocessingDescr::ScaleRange(val) => {
                self.mode_widget.value = PreprocessingWidgetMode::ScaleRange;
                self.scale_range_widget.set_value(val);
            },
            modelrdf::PreprocessingDescr::EnsureDtype(val) => {
                self.mode_widget.value = PreprocessingWidgetMode::EnsureDtype;
                self.ensure_dtype_widget.set_value(val.dtype);
            },
            modelrdf::PreprocessingDescr::FixedZeroMeanUnitVariance(val) => {
                self.mode_widget.value = PreprocessingWidgetMode::FixedZmuv;
                self.fixed_zmuv_widget.set_value(val);
            }
        }
    }
}

impl ItemWidgetConf for PreprocessingWidget{
    const ITEM_NAME: &'static str = "Preprocessing";
}

impl StatefulWidget for PreprocessingWidget{
    type Value<'p> = Result<modelrdfpreproc::PreprocessingDescr>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Preprocessing Type: ");
                self.mode_widget.draw_and_parse(ui, id.with("preproc type".as_ptr()));
            });
            match self.mode_widget.value{
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
                    ui.horizontal(|ui|{
                        ui.strong("Data Type: ");
                        self.ensure_dtype_widget.draw_and_parse(ui, id.with("ensure_dtype".as_ptr()))
                    });
                },
                PreprocessingWidgetMode::FixedZmuv => {
                    self.fixed_zmuv_widget.draw_and_parse(ui, id.with("fixed_zmuv".as_ptr()) )
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode_widget.value{
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
            },
            PreprocessingWidgetMode::FixedZmuv => {
                modelrdfpreproc::PreprocessingDescr::FixedZeroMeanUnitVariance(
                    self.fixed_zmuv_widget.state()?
                )
            }
        })
    }
}
