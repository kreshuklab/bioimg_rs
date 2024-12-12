pub fn github_user_label(ui: &mut egui::Ui, staging_user: Option<&str>) -> egui::Response{
    ui.strong("Github User: ").on_hover_ui(|ui|{
        ui.vertical(|ui|{
            ui.label("The Author's github user, if any, without the '@' symbol.");
            match staging_user{
                None  => {
                    ui.label("A valid user should be able to be inspected at https://api.github.com/users/AUTHOR_USERNAME_HERE");
                },
                Some(username) => 'link_to_username: {
                    if username.is_empty(){
                        ui.label("A valid user should be able to be inspected at https://api.github.com/users/AUTHOR_USERNAME_HERE");
                        break 'link_to_username
                    }
                    ui.horizontal(|ui|{
                        ui.label("A valid user should be able to be inspected at");
                        ui.hyperlink(format!("https://api.github.com/users/{username}"));
                    });
                }
            }
        });
    })
}

pub fn affiliation_label(ui: &mut egui::Ui) -> egui::Response{
    ui.strong("Affiliation: ").on_hover_text("The company, institute or entity for which the author works, if any.")
}

pub fn orcid_label(ui: &mut egui::Ui, person_title: &str) -> egui::Response{
    ui.strong("Orcid: ").on_hover_ui(|ui| {
        ui.horizontal(|ui|{
            ui.label(format!("The {person_title}'s"));
            ui.hyperlink_to("ORCID number", "https://orcid.org/");
            ui.label(" if they have one");
        });
    })
}
