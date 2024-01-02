use std::fmt::Display;

pub trait SourceWidget<RAW>
where
    Self: TryFrom<RAW>,
{
    fn raw_widget(ui: &mut egui::Ui, raw: &mut RAW);
}

pub struct MyString(String);
impl TryFrom<String> for MyString {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() % 2 == 0 {
            Ok(Self(value))
        } else {
            Err(format!("String {value} does not have even number of chars"))
        }
    }
}
impl SourceWidget<String> for MyString {
    fn raw_widget(ui: &mut egui::Ui, raw: &mut String) {
        ui.text_edit_singleline(raw);
    }
}

#[derive(Debug)]
pub struct Stage<PARSED, RAW>
where
    PARSED: TryFrom<RAW>,
{
    pub raw: RAW,
    pub parsed: Result<PARSED, PARSED::Error>,
}

impl<PARSED, RAW> Stage<PARSED, RAW>
where
    PARSED: TryFrom<RAW>,
    PARSED::Error: Display,
    RAW: Clone,
{
    pub fn show(&mut self, ui: &mut egui::Ui)
    where
        PARSED: SourceWidget<RAW>,
    {
        PARSED::raw_widget(ui, &mut self.raw);
        self.parsed = PARSED::try_from(self.raw.clone());
        if let Err(ref err) = self.parsed {
            let error_text = format!("{err}");
            ui.label(egui::RichText::new(error_text).color(egui::Color32::from_rgb(110, 0, 0)));
        };
    }
}
