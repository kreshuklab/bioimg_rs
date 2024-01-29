pub mod author_widget;
pub mod file_widget;
pub mod cover_image_widget;
pub mod url_widget;
pub mod cite_widget;
pub mod error_display;
pub mod functional;

pub trait StatefulWidget{
    type Value<'p> where Self: 'p;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id);
    fn state<'p>(&'p self) -> Self::Value<'p>;
}


#[derive(Clone, Debug)]
pub enum InputLines{
    SingleLine,
    Multiline
}

#[derive(Debug)]
pub struct StagingString<T: TryFrom<String>>{
    raw: String,
    parsed: Result<T, T::Error>,
    input_lines: InputLines,
}

impl<T: TryFrom<String>> Default for StagingString<T>{
    fn default() -> Self {
        let raw = String::default();
        Self {
            raw: raw.clone(), parsed: T::try_from(raw), input_lines: InputLines::SingleLine
        }
    }
}

impl<T: TryFrom<String>> StagingString<T>{
    pub fn new(input_lines: InputLines) -> Self{
        let raw = String::default();
        Self{
            raw: raw.clone(),
            parsed: T::try_from(raw),
            input_lines,
        }
    }
}

impl<T> StatefulWidget for StagingString<T>
where
    T: TryFrom<String> + Clone,
    T::Error: Clone,
{
    type Value<'p> = Result<T, T::Error> where T: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, _id: egui::Id){
        match self.input_lines{
            InputLines::SingleLine => {
                ui.add( //FIXME: any way we can not hardcode this? at least use font size?
                    egui::TextEdit::singleline(&mut self.raw).min_size(egui::Vec2{x: 200.0, y: 10.0})
                );
            },
            InputLines::Multiline => {ui.text_edit_multiline(&mut self.raw);},
        }
        self.parsed = T::try_from(self.raw.clone())
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.parsed.clone()
    }
}


#[derive(Clone, Debug, Default)]
pub struct StagingOpt<Stg: StatefulWidget>(Option<Stg>);

impl<Stg> StatefulWidget for StagingOpt<Stg> where Stg: Default + StatefulWidget{
    type Value<'p> = Option<Stg::Value<'p>>
    where
        Stg::Value<'p>: 'p,
        Stg: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id){
        ui.horizontal(|ui|{
            if self.0.is_none(){
                ui.label("None");
                if ui.button("Add").clicked(){
                    self.0 = Some(Stg::default())
                }
            }else{
                let x_clicked = ui.button("🗙").clicked();
                self.0.as_mut().unwrap().draw_and_parse(ui, id);
                if x_clicked{
                    self.0.take();
                }
            }
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.0.as_ref().map(|inner_widget| inner_widget.state())
    }
}

pub struct StagingVec<Stg> where Stg: StatefulWidget{
    item_name: String,
    staging: Vec<Stg>,
}

impl<Stg: StatefulWidget + Default> StagingVec<Stg>{
    pub fn new(item_name: impl Into<String>) -> Self {
        Self{staging: vec![Stg::default()], item_name: item_name.into()}
    }
}

impl<Stg: StatefulWidget> StatefulWidget for StagingVec<Stg>
where
Stg: Default{
    type Value<'p> = Vec<Stg::Value<'p>>
    where
        Stg: 'p,
        Stg::Value<'p>: 'p;

    fn draw_and_parse<'p>(&'p mut self, ui: &mut egui::Ui, id: egui::Id){
        let item_name = &self.item_name;
        ui.vertical(|ui|{
            self.staging.iter_mut()
                .enumerate()
                .for_each(|(idx, staging_item)| {
                    ui.label(format!("{item_name} #{}", idx + 1));
                    staging_item.draw_and_parse(ui, id.with(idx));
                    ui.separator();
                });
            ui.horizontal(|ui|{
                if ui.button(format!("+ Add {item_name}")).clicked(){
                    self.staging.resize_with(self.staging.len() + 1, Stg::default);
                }
                if ui.button(format!("- Remove {item_name}")).clicked() && self.staging.len() > 1{
                    self.staging.resize_with(self.staging.len() - 1, Stg::default);
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.staging.iter().map(|item_widget| item_widget.state()).collect()
    }
}
