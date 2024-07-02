use crate::project_data::CollapsibleWidgetRawData;

use super::{Restore, StatefulWidget, ValueWidget};

pub trait SummarizableWidget{
    fn summarize(&mut self, ui: &mut egui::Ui, id: egui::Id);
}

#[derive(Default)]
pub struct CollapsibleWidget<W>{
    pub is_closed: bool,
    pub inner: W,
}

impl<W: Restore> Restore for CollapsibleWidget<W>{
    type RawData = CollapsibleWidgetRawData<W>;

    fn dump(&self) -> Self::RawData {
        CollapsibleWidgetRawData{
            is_closed: self.is_closed,
            inner: self.inner.dump()
        }
    }

    fn restore(&mut self, raw: Self::RawData) {
        self.is_closed.restore(raw.is_closed);
        self.inner.restore(raw.inner)
    }
}

impl<W> StatefulWidget for CollapsibleWidget<W>
where
    W: StatefulWidget + SummarizableWidget,
{
    type Value<'p> = W::Value<'p> where W: 'p;
    
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){
        let frame = egui::Frame::none()
            .inner_margin(4.0)
            .stroke(ui.visuals().window_stroke);
        frame.show(ui, |ui|{
            if self.is_closed{
                ui.horizontal(|ui|{
                    if ui.button("⏷").clicked(){
                        self.is_closed = false;
                    }
                    self.inner.summarize(ui, id.with("summary".as_ptr()));
                });
            }else{
                ui.horizontal(|ui|{
                    if ui.button("⏶").clicked(){
                        self.is_closed = true;
                    }
                    ui.vertical(|ui|{
                        self.inner.draw_and_parse(ui, id.with("inner".as_ptr()));
                    })
                });
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.inner.state()
    }
}

impl<W: ValueWidget> ValueWidget for CollapsibleWidget<W>{
    type Value<'v> = W::Value<'v>;

    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.inner.set_value(value);
        self.is_closed = false; //FIXME: we rely on drawing to update widgets, so we need this open =/
    }
}
