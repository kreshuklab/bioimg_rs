use super::{
    age_widget::{Age, AgeParsingError, StagingAge},
    fancy_string_widget::{FancyString, FancyStringParsingError, StagingFancy},
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

impl ParsingWidget for Person {
    type Raw = RawPerson;
    fn draw_and_parse(ui: &mut egui::Ui, raw: &mut RawPerson) -> Result<Person, PersonBuildError> {
        let name = ui.horizontal(|ui|{
            ui.label("Person's name: ");
            FancyString::draw_and_parse(ui, &mut raw.name)
        }).inner;
        let age = ui.horizontal(|ui|{
            ui.label("Person's age: ");
            Age::draw_and_parse(ui, &mut raw.age)
        }).inner;
        Ok(Person { name: name?, age: age? })
    }
}



pub struct StagingPerson{
    staging_name: StagingFancy,
    staging_age: StagingAge,
    parsed: Result<Person, PersonBuildError>,
}

impl StagingPerson{
    pub fn draw_and_update(&mut self, ui: &mut egui::Ui){
        ui.horizontal(|ui|{
            ui.label("Person's name: ");
            self.staging_name.draw_and_update(ui);
        });
        ui.horizontal(|ui|{
            ui.label("Person's age: ");
            self.staging_age.draw_and_update(ui);
        });

        let name = match &self.staging_name.parsed{
            Err(err) => {
                self.parsed = Err(err.clone().into());
                return
            },
            Ok(name) => name.clone(),
        };
        let age = match &self.staging_age.parsed{
            Err(err) => {
                self.parsed = Err(err.clone().into());
                return
            },
            Ok(age) => age.clone(),
        };
        self.parsed = Ok(Person{name, age})
    }
}