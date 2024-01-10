use std::path::PathBuf;

use super::DrawAndParse;

#[derive(thiserror::Error, Debug)]
pub enum FilePickerError{
    #[error("Empty")]
    Empty,
    #[error("User cancelled")]
    UserCancelled,
    #[error("{0}")]
    IoError(#[source] #[from] std::io::Error),
}

pub struct ImageWidget{
    contents: Option<(PathBuf, Vec<u8>)>,
}
impl ImageWidget{
    pub fn path(&self) -> Option<&PathBuf>{
        self.contents.as_ref().map(|(path, _)| path)
    }
    pub fn data(&self) -> Option<&[u8]>{
        self.contents.as_ref().map(|(_, data)| data.as_slice())
    }
}

impl<'data> DrawAndParse for &'data mut ImageWidget{
    type Parsed = &'data [u8];
    type Error = FilePickerError;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) -> Result<Self::Parsed, Self::Error>{
        ui.horizontal(|ui| -> Result<(), Self::Error>{
            match &self.path(){
                None => ui.label("None"),
                Some(path) => ui.label(path.to_string_lossy())
            };

            if ui.button("Open...").clicked(){
                let path_buf = rfd::FileDialog::new()
                    .set_directory("/")
                    .pick_file();
                if path_buf.as_ref() != self.path(){
                    self.contents.take();
                    if let Some(pth) = path_buf{
                        let data = std::fs::read(&pth)?; //FIXME: async + web?
                        self.contents = Some((
                            pth, data
                        ));
                    }
                }
            }
            Ok(())
        }).inner?;

        self.data().ok_or(FilePickerError::Empty)
    }
}