use egui::InnerResponse;

use crate::widgets::from_widget::{SourceWidget, Stage};

#[derive(thiserror::Error, Debug)]
pub enum FancyStringParsingError {
    #[error("String is too long to be fancy")]
    TooLong,
}

#[derive(Clone, Debug)]
pub struct FancyString(String);
impl TryFrom<String> for FancyString {
    type Error = FancyStringParsingError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 10 {
            Err(FancyStringParsingError::TooLong)
        } else {
            Ok(Self(value))
        }
    }
}
impl SourceWidget<String> for FancyString {
    //FIXME: blanked impl for T: TryFrom ?
    fn raw_widget(ui: &mut egui::Ui, raw: &mut String) {
        ui.text_edit_singleline(raw);
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AgeParsingError {
    #[error("Too old")]
    TooOld,
}

pub struct Age(u8);
impl TryFrom<u8> for Age {
    type Error = AgeParsingError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 120 {
            return Err(AgeParsingError::TooOld);
        }
        return Ok(Self(value));
    }
}

pub struct Person {
    name: FancyString,
    age: Age,
}

#[derive(thiserror::Error, Debug)]
pub enum PersonBuildError {
    #[error("Bad name: {0}")]
    BadName(#[from] FancyStringParsingError),
    #[error("Bad age: {0}")]
    BadAge(#[from] AgeParsingError),
}

#[derive(Debug)]
pub struct StagingPerson {
    pub name: Stage<FancyString, String>,
    pub age: u8, //also normal int
}

impl TryFrom<StagingPerson> for Person {
    type Error = PersonBuildError;
    fn try_from(value: StagingPerson) -> Result<Self, Self::Error> {
        let name = match value.name.parsed {
            Err(_) => return Err(PersonBuildError::BadName(FancyStringParsingError::TooLong)), //FIXME: wrong variant
            Ok(val) => val,
        };
        let age = Age::try_from(value.age)?;
        return Ok(Person { age, name });
    }
}

fn staging_person_form(ui: &mut egui::Ui, staging_person: &mut StagingPerson) -> InnerResponse<()> {
    ui.vertical(|ui| {
        egui::Frame::none()
            .fill(egui::Color32::DARK_BLUE)
            .stroke(egui::Stroke {
                width: 1.0,
                color: egui::Color32::GOLD,
            })
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Person's name: ");
                    staging_person.name.show(ui);
                });

                ui.horizontal(|ui| {
                    let age_res = Age::try_from(staging_person.age);

                    ui.label("Person's age: ");
                    ui.add(egui::DragValue::new(&mut staging_person.age).speed(1.0));
                    if let Err(_) = age_res {
                        ui.label(egui::RichText::new("Bad age").color(egui::Color32::from_rgb(110, 0, 0)));
                    };
                })
            });
    })
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct TemplateApp {
    staging_person: StagingPerson,

    // Example stuff:
    label: String,
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            staging_person: StagingPerson {
                name: Stage {
                    raw: String::default(),
                    parsed: Err(FancyStringParsingError::TooLong), //FIXME: wrong variant
                },
                age: 0,
            },
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            staging_person_form(ui, &mut self.staging_person);

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to("eframe", "https://github.com/emilk/egui/tree/master/crates/eframe");
        ui.label(".");
    });
}
