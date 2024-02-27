use std::{error::Error, marker::PhantomData};

use crate::result::Result;

use super::{util::group_frame, StatefulWidget};

pub struct StagingFromVec<ItemWidget, Parsed> {
    pub item_name: String,
    pub staging_widgets: Vec<ItemWidget>,
    marker: PhantomData<Parsed>,
}

impl<ItemWidget: Default, Parsed> StagingFromVec<ItemWidget, Parsed> {
    #[allow(dead_code)]
    pub fn new(item_name: impl Into<String>) -> Self {
        Self {
            staging_widgets: vec![ItemWidget::default()],
            item_name: item_name.into(),
            marker: PhantomData,
        }
    }
}

impl<ItemWidget, Parsed> StatefulWidget for StagingFromVec<ItemWidget, Parsed>
where
    ItemWidget: Default + StatefulWidget,
    for<'a> Parsed: TryFrom<Vec<ItemWidget::Value<'a>>>,
    for<'a> <Parsed as TryFrom<Vec<ItemWidget::Value<'a>>>>::Error: Error,
{
    type Value<'p> = Result<Parsed> where Parsed: 'p, ItemWidget: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id) {
        let item_name = &self.item_name;
        ui.vertical(|ui| {
            self.staging_widgets.iter_mut().enumerate().for_each(|(idx, staging_item)| {
                ui.label(format!("{item_name} #{}", idx + 1));
                group_frame(ui, |ui| {
                    staging_item.draw_and_parse(ui, id.with(idx));
                });
            });
            ui.horizontal(|ui| {
                if ui.button(format!("+ Add {item_name}")).clicked() {
                    self.staging_widgets
                        .resize_with(self.staging_widgets.len() + 1, ItemWidget::default);
                }
                if ui.button(format!("- Remove {item_name}")).clicked() && self.staging_widgets.len() > 1 {
                    self.staging_widgets
                        .resize_with(self.staging_widgets.len() - 1, ItemWidget::default);
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let items: Vec<_> = self.staging_widgets.iter().map(|item_widget| item_widget.state()).collect();
        return Ok(Parsed::try_from(items)?);
    }
}
