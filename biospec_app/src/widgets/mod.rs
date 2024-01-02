use crate::spec::{Author2, StrictString};


#[derive(Default)]
pub struct StagingAuthor2{
    pub name: String,
    pub affiliation: String,
    pub email: String,
    pub github_user: String,
    pub orcid: String,
}

impl TryFrom<StagingAuthor2> for Author2{
    type Error = StagingAuthor2;
    fn try_from(value: StagingAuthor2) -> Result<Self, Self::Error> {
        let name: StrictString<1, 1023> = match value.name.try_into(){
            Ok(name) => name,
            Err(err) => return Err(StagingAuthor2{
                name: err.value(),
                ..value
            }),
        };
        let affiliation: StrictString<1, 1023> = match value.affiliation.try_into(){
            Ok(affiliation) => affiliation,
            Err(err) => return Err(StagingAuthor2{
                affiliation: err.value(),
                ..value
            }),
        };
        let email: StrictString<1, 1023> = match value.email.try_into(){
            Ok(email) => email,
            Err(err) => return Err(StagingAuthor2{
                email: err.value(),
                ..value
            }),
        };
        let github_user: StrictString<1, 1023> = match value.github_user.try_into(){
            Ok(github_user) => github_user,
            Err(err) => return Err(StagingAuthor2{
                github_user: err.value(),
                ..value
            }),
        };
        let orcid: StrictString<1, 1023> = match value.orcid.try_into(){
            Ok(orcid) => orcid,
            Err(err) => return Err(StagingAuthor2{
                orcid: err.value(),
                ..value
            }),
        };
        Ok(Author2{
            name,
            affiliation: Some(affiliation),
            email: Some(email),
            github_user: Some(github_user),
            orcid: Some(orcid)
        })
    }
}

pub struct Author2Editor<'data>{
    pub data: &'data mut StagingAuthor2
}


impl Author2Editor<'_>{
    pub fn show(&mut self, ui: &mut egui::Ui){
        egui::Frame::none()
            .fill(egui::Color32::RED)
            .show(ui, |ui| {
                ui.horizontal(|ui|{
                    ui.label("Name: "); ui.text_edit_singleline(&mut self.data.name);
                });
                ui.label("Label with red background");
            });
    }
}