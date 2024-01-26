pub fn string_parser<T>(ui: &mut egui::Ui, raw: &mut String, result: &mut Result<T, T::Error>)
where
    T: TryFrom<String>
{
    ui.text_edit_singleline(raw);
    *result = T::try_from(raw.clone());
}

pub struct UiStringable<T: TryFrom<String>>{
    raw: String,
    parsed: Result<T, T::Error>,
}