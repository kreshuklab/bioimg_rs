use super::{
    age_widget::{Age, AgeParsingError},
    fancy_string_widget::{FancyString, FancyStringParsingError},
    ParsingWidget,
};

pub struct Person {
    name: FancyString,
    age: Age,
}

#[derive(Debug, Clone, Default)]
pub struct RawPerson {
    pub name: String,
    pub age: u8,
}

#[derive(thiserror::Error, Debug)]
pub enum PersonBuildError {
    #[error("Bad name: {0}")]
    BadName(#[from] FancyStringParsingError),
    #[error("Bad age: {0}")]
    BadAge(#[from] AgeParsingError),
    #[error("Empty")]
    Empty,
}

impl TryFrom<RawPerson> for Person {
    type Error = PersonBuildError;
    fn try_from(raw: RawPerson) -> Result<Self, Self::Error> {
        let name = FancyString::try_from(raw.name)?;
        let age = Age::try_from(raw.age)?;
        return Ok(Person { age, name });
    }
}

impl ParsingWidget<RawPerson> for Person {
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut RawPerson) -> Result<Self, Self::Error> {
        let name = ui.horizontal(|ui|{
            ui.label("Person's name: ");
            FancyString::draw_and_parse(ui, &mut raw.name)
        }).inner?;
        let age = ui.horizontal(|ui|{
            ui.label("Person's age: ");
            Age::draw_and_parse(ui, &mut raw.age)
        }).inner?;
        Ok(Person { name, age })
    }
}