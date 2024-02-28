use super::{util::group_frame, StatefulWidget};

pub struct StagingVec<Stg>
where
    Stg: StatefulWidget,
{
    pub item_name: String,
    pub staging: Vec<Stg>,
}

impl<Stg: StatefulWidget + Default> StagingVec<Stg> {
    pub fn new(item_name: impl Into<String>) -> Self {
        Self {
            staging: vec![],
            item_name: item_name.into(),
        }
    }
}

impl<Stg: StatefulWidget> StatefulWidget for StagingVec<Stg>
where
    Stg: Default,
{
    type Value<'p> = Vec<Stg::Value<'p>>
    where
        Stg: 'p,
        Stg::Value<'p>: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        let item_name = &self.item_name;
        ui.vertical(|ui| {
            self.staging.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                ui.label(format!("{item_name} #{}", idx + 1));
                group_frame(ui, |ui| {
                    staging_item.draw_and_parse(ui, id.with(idx));
                });
            });
            ui.horizontal(|ui| {
                if ui.button(format!("+ Add {item_name}")).clicked() {
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                if ui.button(format!("- Remove {item_name}")).clicked() && self.staging.len() > 0 {
                    self.staging.resize_with(self.staging.len() - 1, Stg::default);
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.staging.iter().map(|item_widget| item_widget.state()).collect()
    }
}
