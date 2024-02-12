use std::{
    ops::Deref,
    path::{Path, PathBuf},
};

use bioimg_spec::rdf::model::shape;
use egui::{load::SizedTexture, ImageSource};

use super::{error_display::show_error, file_widget::ParsedFile};

macro_rules! impl_NpyArray_try_read {
    ($($element_type:ident),+) => {
        paste::paste! {

            pub enum NpyArray {
                $(
                    [<Array $element_type:upper>](ndarray::ArrayD<$element_type>),
                )*
            }

            impl NpyArray {
                fn try_read(npy_path: &Path) -> anyhow::Result<Self> {
                    $(
                        match ndarray_npy::read_npy::<_, ndarray::ArrayD<$element_type>>(npy_path) {
                            Ok(arr) => return Ok(Self::[<Array $element_type:upper>](arr)),
                            Err(err) => match err {
                                ndarray_npy::ReadNpyError::WrongDescriptor(_) => (),
                                other_err => anyhow::bail!(other_err),
                            },
                        }
                    )+
                    anyhow::bail!("Can't interpret npy file at {}", npy_path.to_string_lossy())
                }

                pub fn shape(&self) -> &[usize] {
                    match self {
                        $(
                            Self::[<Array $element_type:upper>](arr) => arr.shape(),
                        )*
                    }
                }
            }
        }
    };
}

impl_NpyArray_try_read!(u8, i8, u16, i16, u32, i32, u64, i64, f32, f64);

pub struct GuiNpyArray {
    path: PathBuf,
    contents: NpyArray,
    context: egui::Context,
    texture_handle: Option<egui::TextureHandle>,
}

impl Deref for GuiNpyArray {
    type Target = NpyArray;
    fn deref(&self) -> &Self::Target {
        &self.contents
    }
}

impl Drop for GuiNpyArray {
    fn drop(&mut self) {
        if self.texture_handle.is_some() {
            self.context.forget_image(&self.path.to_string_lossy());
        }
    }
}

impl ParsedFile for anyhow::Result<GuiNpyArray> {
    fn parse(path: PathBuf, ctx: egui::Context) -> Self {
        let npy_array = NpyArray::try_read(&path)?;
        Ok(GuiNpyArray {
            path: path.clone(),
            contents: npy_array,
            context: ctx,
            texture_handle: None, //FIXME: try to make it into an image
        })
    }

    fn render(&self, ui: &mut egui::Ui, _id: egui::Id) {
        let loaded_cover_image = match self {
            Ok(loaded_cover_image) => loaded_cover_image,
            Err(err) => {
                show_error(ui, err.to_string());
                return;
            }
        };

        if let Some(texture_handle) = &loaded_cover_image.texture_handle {
            let image_source = ImageSource::Texture(SizedTexture {
                id: texture_handle.id(),
                size: egui::Vec2 { x: 20.0, y: 20.0 },
            });
            let ui_img = egui::Image::new(image_source);
            ui.add(ui_img);
        };

        let shape = loaded_cover_image.contents.shape();
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
