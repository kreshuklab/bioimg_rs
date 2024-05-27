use bioimg_spec::rdf::model::preprocessing as modelrdfpreproc;
use bioimg_spec::rdf::model as modelrdf;

use crate::result::{GuiError, Result, VecResultExt};
use super::staging_vec::ItemWidgetConf;
use super::ValueWidget;
use super::{staging_num::StagingNum, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::StagingVec, StatefulWidget};

pub struct ZeroMeanAxesItemConfig;
impl ItemWidgetConf for ZeroMeanAxesItemConfig{
    const ITEM_NAME: &'static str = "Axis";
    const INLINE_ITEM: bool = true;
}

pub struct ZeroMeanUnitVarianceWidget{
    pub axes_widget: StagingOpt<StagingVec< StagingString<modelrdf::AxisId>, ZeroMeanAxesItemConfig >>,
    pub epsilon_widget: StagingNum<f32, f32>,
}


impl ValueWidget for ZeroMeanUnitVarianceWidget{
    type Value<'v> = modelrdfpreproc::Zmuv;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.axes_widget.set_value(value.axes.map(|val| val.into_inner()));
        self.epsilon_widget.set_value(value.eps);
    }
}

impl Default for ZeroMeanUnitVarianceWidget{
    fn default() -> Self {
        Self{
            axes_widget: Default::default(),
            epsilon_widget: StagingNum::new_with_raw(1e-6),
        }
    }
}

impl StatefulWidget for ZeroMeanUnitVarianceWidget{
    type Value<'p> = Result<modelrdfpreproc::Zmuv>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Axes: ");
                self.axes_widget.draw_and_parse(ui, id.with("axes".as_ptr()));
            });
            ui.horizontal(|ui|{
                ui.strong("Epsilon: ");
                self.epsilon_widget.draw_and_parse(ui, id.with("epsilon".as_ptr()));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdfpreproc::Zmuv{
            axes: match self.axes_widget.state(){
                None => None,
                Some(vals) => {
                    let vals = vals.collect_result()?;
                    Some(
                        vals.try_into().map_err(|_| GuiError::new("Needs at least one axis id".to_owned()))?
                    )
                }
            },
            eps: self.epsilon_widget.state()?
        })
    }
}
