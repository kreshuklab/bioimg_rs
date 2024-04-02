
use crate::result::Result;
use bioimg_runtime as rt;

use super::{
    error_display::show_error, image_widget::ImageWidget, staging_vec::ItemWidgetConf, StatefulWidget
};

#[derive(Default)]
pub struct CoverImageWidget{
    pub image_widget: ImageWidget,
}

impl StatefulWidget for CoverImageWidget{
    type Value<'p> = Result<rt::CoverImage>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                self.image_widget.draw_and_parse(ui, id);
            });
            match self.image_widget.state(){
                Err(err) => show_error(ui, err),
                Ok(img) => {
                    if let Err(err) = rt::CoverImage::try_from(img){
                        show_error(ui, err)
                    }
                }
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(rt::CoverImage::try_from(self.image_widget.state()?)?)
    }
}

impl ItemWidgetConf for CoverImageWidget{
    const ITEM_NAME: &'static str = "Cover Image";
    const MIN_NUM_ITEMS: usize = 1;
}