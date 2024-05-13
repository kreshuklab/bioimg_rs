use std::{io::Read, path::PathBuf, sync::Arc};

use bioimg_runtime::npy_array::NpyArray;

use super::{error_display::show_error, file_widget::ParsedFile};
use crate::result::Result;

impl ParsedFile for Result<Arc<NpyArray>> {
    fn parse(path: PathBuf, _ctx: egui::Context) -> Self {
        let mut data = vec![];
        std::fs::File::open(&path)?.read_to_end(&mut data)?;
        Ok(Arc::new(NpyArray::try_load(&mut data.as_slice())?))
    }

    fn render(&self, ui: &mut egui::Ui, _id: egui::Id) {
        let loaded_cover_image = match self {
            Ok(loaded_cover_image) => loaded_cover_image,
            Err(err) => {
                show_error(ui, err.to_string());
                return;
            }
        };

        let shape = loaded_cover_image.shape();
        let last_item_idx = shape.len() - 1;
        let shape_str =
            shape
                .iter()
                .map(|v| v.to_string())
                .enumerate()
                .fold(String::with_capacity(128), |mut acc, (idx, size)| {
                    acc += size.as_str();
                    if idx < last_item_idx {
                        acc += ", "
                    }
                    acc
                });
        ui.weak(format!("C-order shape: [{shape_str}]"));
    }
}
