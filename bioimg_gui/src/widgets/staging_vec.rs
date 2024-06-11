use std::marker::PhantomData;

use super::{util::group_frame, StatefulWidget, ValueWidget};

pub trait ItemWidgetConf{
    const ITEM_NAME: &'static str;
    const INLINE_ITEM: bool = false;
    const MIN_NUM_ITEMS: usize = 0;
    const GROUP_FRAME: bool = true;
}

pub struct StagingVec<Stg, Conf=Stg>{
    pub staging: Vec<Stg>,
    marker: PhantomData<Conf>,
}

impl<Stg, Conf> ValueWidget for StagingVec<Stg, Conf>
where
    Stg: ValueWidget + Default
{
    type Value<'a> = Vec<<Stg as ValueWidget>::Value<'a>>;
    fn set_value<'a>(&mut self, value: Self::Value<'a>){
        self.staging = value.into_iter().map(|item_value|{
            let mut widget = Stg::default();
            widget.set_value(item_value);
            widget
        }).collect();
    }
}

impl<Stg: Default, Conf: ItemWidgetConf> Default for StagingVec<Stg, Conf>{
    fn default() -> Self {
        Self{
            staging:  (0..Conf::MIN_NUM_ITEMS).map(|_| Stg::default()).collect(),
            marker: PhantomData,
        }
    }
}

impl<Stg: StatefulWidget, Conf> StatefulWidget for StagingVec<Stg, Conf>
where
    Stg: Default,
    Conf: ItemWidgetConf,
{
    type Value<'p> = Vec<Stg::Value<'p>>
    where
        Stg: 'p,
        Conf: 'p,
        Stg::Value<'p>: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            self.staging.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                let item_label = format!("{} #{}", Conf::ITEM_NAME, idx + 1);
                let render_item = |ui: &mut egui::Ui| {
                    if Conf::INLINE_ITEM{
                        ui.horizontal(|ui|{
                            ui.label(item_label);
                            staging_item.draw_and_parse(ui, id.with(idx));
                        });
                    }else{
                        ui.label(item_label);
                        staging_item.draw_and_parse(ui, id.with(idx));
                    }
                };
                if Conf::GROUP_FRAME{
                    group_frame(ui, render_item);
                }else{
                    render_item(ui);
                }
            });
            ui.horizontal(|ui| {
                if ui.button(format!("+ Add {}", Conf::ITEM_NAME)).clicked() {
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                ui.add_enabled_ui(self.staging.len() > Conf::MIN_NUM_ITEMS, |ui|{
                    if ui.button(format!("- Remove {}", Conf::ITEM_NAME)).clicked() {
                        self.staging.resize_with(self.staging.len() - 1, Stg::default);
                    }
                });
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.staging.iter().map(|item_widget| item_widget.state()).collect()
    }
}
