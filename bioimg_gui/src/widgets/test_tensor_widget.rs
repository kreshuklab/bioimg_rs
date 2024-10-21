use std::{io::{Cursor, Read}, path::{Path, PathBuf}, sync::Arc};

use bioimg_runtime::{npy_array::ArcNpyArray, NpyArray};

use crate::{project_data::TestTensorWidgetRawData, result::GuiError};

use super::{error_display::show_error, Restore, StatefulWidget, ValueWidget};


#[derive(Default)]
pub enum TestTensorWidget{
    #[default]
    Empty,
    Loaded{path: Option<PathBuf>, data: ArcNpyArray},
    Error{message: String}
}

impl ValueWidget for TestTensorWidget{
    type Value<'v> = ArcNpyArray;

    fn set_value<'v>(&mut self, data: Self::Value<'v>) {
        *self = Self::Loaded { path: None, data}
    }

}

impl Restore for TestTensorWidget{
    type RawData = TestTensorWidgetRawData;

    fn dump(&self) -> Self::RawData {
        match self{
            Self::Empty  | Self::Error { .. }=> TestTensorWidgetRawData::Empty,
            Self::Loaded { path, data } => TestTensorWidgetRawData::Loaded {
                path: path.clone(),
                data: {
                    let mut v = vec![];
                    data.write_npy(&mut v).expect("Should not have failed to write into a vec");
                    v
                }
            }
        }
    }

    fn restore(&mut self, raw: Self::RawData) {
        *self = match raw{
            TestTensorWidgetRawData::Empty => Self::Empty,
            TestTensorWidgetRawData::Loaded { path, data } => {
                match NpyArray::try_load(Cursor::new(data)){
                    Ok(data) => TestTensorWidget::Loaded { path, data: Arc::new(data) },
                    Err(_e) => TestTensorWidget::Error { message: "Could not deserialize npy data".to_owned() }
                }
            }
        }
    }
}

impl TestTensorWidget{
    pub fn try_load(path: &Path) -> Result<ArcNpyArray, GuiError>{
        let mut data = vec![];
        std::fs::File::open(&path)?.read_to_end(&mut data)?;
        let data = NpyArray::try_load(&mut data.as_slice())?;
        Ok(Arc::new(data))
    }

    pub fn load(&mut self, path: &Path){
        *self = match Self::try_load(path){
            Ok(img) => TestTensorWidget::Loaded{path: Some(path.to_owned()), data: img},
            Err(e) => TestTensorWidget::Error { message: e.to_string() }
        }
    }
}

impl StatefulWidget for TestTensorWidget{
    type Value<'p> = Result<ArcNpyArray, GuiError>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui|{
            if ui.button("Open...").clicked(){
                if let Some(path) = rfd::FileDialog::new().add_filter("bioimage model", &["npy"],).pick_file() {
                    self.load(&path);
                } else {
                    *self = TestTensorWidget::Empty;
                }
            }
            match self{
                Self::Empty => (),
                Self::Loaded { path, data } => {
                    let shape = data.shape();
                    let last_item_idx = shape.len() - 1;
                    let shape_str = shape
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
                    ui.weak(format!("C-order shape: [{shape_str}] "));
                    if let Some(p) = path{
                        ui.weak("from");
                        ui.weak(p.to_string_lossy());
                    }
                },
                Self::Error { message } => {
                    show_error(ui, &message);
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match self{
            Self::Empty => Err(GuiError::new("Empty")),
            Self::Error { message } => Err(GuiError::new(message)),
            Self::Loaded { data, .. } => Ok(Arc::clone(data)),
        }
    }
}
