use std::io::{Cursor, Read};
use std::time::Instant;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use bioimg_runtime::{npy_array::ArcNpyArray, zip_archive_ext::SeekReadSend, NpyArray};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard, RawMutex};

use crate::{project_data::TestTensorWidgetRawData, result::GuiError};

use super::{error_display::show_error, util::TaskChannel, Restore, StatefulWidget, ValueWidget};


#[derive(Default)]
pub enum TestTensorWidgetState{
    #[default]
    Empty,
    Loaded{path: Option<PathBuf>, data: ArcNpyArray},
    Error{message: String}
}

pub struct TestTensorWidget{
    state: Arc<Mutex<(Instant, TestTensorWidgetState)>>,
}

impl Default for TestTensorWidget{
    fn default() -> Self {
        Self{
            state: Arc::new(Mutex::new(
                (Instant::now(), Default::default()))
            ),
        }
    }
}


impl ValueWidget for TestTensorWidget{
    type Value<'v> = ArcNpyArray;

    fn set_value<'v>(&mut self, data: Self::Value<'v>) {
        self.state = Arc::new(Mutex::new(
            (Instant::now(), TestTensorWidgetState::Loaded { path: None, data})
        ));
    }

}

impl Restore for TestTensorWidget{
    type RawData = TestTensorWidgetRawData;

    fn dump(&self) -> Self::RawData {
        match &*self.state(){
            TestTensorWidgetState::Empty  | &TestTensorWidgetState::Error { .. }=> TestTensorWidgetRawData::Empty,
            TestTensorWidgetState::Loaded { path, data } => TestTensorWidgetRawData::Loaded {
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
        self.state = Arc::new(Mutex::new(match raw{
            TestTensorWidgetRawData::Empty => (Instant::now(), TestTensorWidgetState::Empty),
            TestTensorWidgetRawData::Loaded { path, data } => {
                let state = match NpyArray::try_load(Cursor::new(data)){
                    Ok(data) => TestTensorWidgetState::Loaded { path, data: Arc::new(data) },
                    Err(_e) => TestTensorWidgetState::Error { message: "Could not deserialize npy data".to_owned() }
                };
                (Instant::now(), state)
            }
        }));
    }
}

impl TestTensorWidget{
    pub fn try_load(mut path: impl Read) -> Result<ArcNpyArray, GuiError>{
        let mut data = vec![];
        path.read_to_end(&mut data)?;
        let data = NpyArray::try_load(&mut data.as_slice())?;
        Ok(Arc::new(data))
    }
    pub fn with_state<F: FnOnce(&TestTensorWidgetState)>(&self, f: F){
        let guard = self.state.lock();
        f(&guard.1)
    }
    pub fn state(&self) -> MappedMutexGuard<'_, TestTensorWidgetState>{
        let guard = self.state.lock();
        MutexGuard::map(guard, |state| &mut state.1)
    }

}

impl StatefulWidget for TestTensorWidget{
    type Value<'p> = Result<ArcNpyArray, GuiError>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, _id: egui::Id) {
        ui.horizontal(|ui|{
            if ui.button("Open...").clicked(){
                let current_state = Arc::clone(&self.state);
                #[cfg(not(target_arch="wasm32"))]
                std::thread::spawn(move ||{
                    let Some(path) = rfd::FileDialog::new().add_filter("numpy array", &["npy"],).pick_file() else {
                        *current_state.lock() = (Instant::now(), TestTensorWidgetState::Empty);
                        return
                    };
                    let instant = Instant::now();
                    let file = match std::fs::File::open(&path){
                        Ok(file) => file,
                        Err(e) => {
                            let mut guard = current_state.lock();
                            if instant > guard.0{
                                *guard = (instant, TestTensorWidgetState::Error{ message: e.to_string()});
                            }
                            return
                        }
                    };
                    let reader = std::io::BufReader::new(file);
                    let new_state = match Self::try_load(reader){
                        Ok(data) => TestTensorWidgetState::Loaded { path: Some(path.to_owned()), data },
                        Err(e) => TestTensorWidgetState::Error { message: e.to_string() }
                    };
                    let mut guard = current_state.lock();
                    if instant > guard.0{
                        *guard = (instant, new_state);
                    }
                });
                #[cfg(target_arch="wasm32")]
                wasm_bindgen_futures::spawn_local(async move {
                    let Some(file) = rfd::AsyncFileDialog::new().add_filter("numpy array", &["npy"],).pick_file().await else {
                        sender.send(TestTensorWidgetState::Empty).unwrap();
                        return
                    };
                    let contents = file.read().await;
                    let reader: Box<dyn SeekReadSend + 'static> = Box::new(std::io::Cursor::new(contents));
                    let task = match Self::try_load(reader){
                        Ok(data) => TestTensorWidgetState::Loaded{path: None, data},
                        Err(e) => TestTensorWidgetState::Error{message: e.to_string()},
                    };
                    sender.send(task).unwrap();
                })
            }
            match &*self.state(){
                TestTensorWidgetState::Empty => (),
                TestTensorWidgetState::Loaded { path, data } => {
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
                TestTensorWidgetState::Error { message } => {
                    show_error(ui, &message);
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        match &*self.state(){
            TestTensorWidgetState::Empty => Err(GuiError::new("Empty")),
            TestTensorWidgetState::Error { message } => Err(GuiError::new(message)),
            TestTensorWidgetState::Loaded { data, .. } => Ok(Arc::clone(data)),
        }
    }
}
