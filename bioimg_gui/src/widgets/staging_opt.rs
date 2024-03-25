use super::{util::group_frame, StatefulWidget};

#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: StatefulWidget>(pub Option<Stg>);

impl<Stg> StatefulWidget for StagingOpt<Stg>
where
    Stg: Default + StatefulWidget,
{
    type Value<'p> = Option<Stg::Value<'p>>
    where
        Stg::Value<'p>: 'p,
        Stg: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.horizontal(|ui| {
            if self.0.is_none() {
                ui.label("None");
                if ui.button("Add").clicked() {
                    self.0 = Some(Stg::default())
                }
            } else {
                let x_clicked = ui.button("🗙").clicked();
                group_frame(ui, |ui| {
                    self.0.as_mut().unwrap().draw_and_parse(ui, id);
                });
                if x_clicked {
                    self.0.take();
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.0.as_ref().map(|inner_widget| inner_widget.state())
    }
}
