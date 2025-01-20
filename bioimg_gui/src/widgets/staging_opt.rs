use super::{collapsible_widget::SummarizableWidget, util::group_frame, Restore, StatefulWidget, ValueWidget};

#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg>(pub Option<Stg>);

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

impl<Stg> SummarizableWidget for StagingOpt<Stg>
where
    Stg: SummarizableWidget
{
    fn summarize(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        if let Some(inner) = &mut self.0{
            inner.summarize(ui, id.with("inner".as_ptr()));
        } else {
            ui.weak("None");
        }
    }
}

impl<Stg> ValueWidget for StagingOpt<Stg>
where
    Stg: ValueWidget + Default
{
    type Value<'a> = Option< <Stg as ValueWidget>::Value<'a> >;
    fn set_value<'a>(&mut self, value: Self::Value<'a>){
        self.0 = value.map(|val|{
            let mut widget = Stg::default();
            widget.set_value(val);
            widget
        });
    }
}

impl<W: Restore + Default> Restore for StagingOpt<W>{
    type RawData = Option<W::RawData>;
    fn dump(&self) -> Self::RawData {
        self.0.as_ref().map(|val| val.dump())
    }
    fn restore(&mut self, raw: Self::RawData) {
        match raw{
            None => {
                self.0 = None
            },
            Some(val) => {
                let mut inner = W::default();
                inner.restore(val);
                self.0 = Some(inner)
            }
        }
    }
}
