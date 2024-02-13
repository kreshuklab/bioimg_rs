use bioimg_spec::rdf;
use bioimg_spec::rdf::bounded_string::BoundedString;

use super::{StagingString, StatefulWidget};

pub struct AxisNameWidget {}

pub struct BatchAxisWidget {
    pub staging_id: StagingString<rdf::model::AxisId>,
    pub staging_description: StagingString<BoundedString<1, { 128 - 1 }>>,
    pub staging_auto_size: bool,
}

// impl StatefulWidget for BatchAxisWidget {
//     type Value<'p> = anyhow::Result<rdf::model::BatchAxis>;

//     fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
//         ui.vertical(|ui| {
//             ui.horizontal(|ui| {
//                 ui.strong("Id: ");
//                 self.staging_id.draw_and_parse(ui, id.with("id"));
//             });
//             ui.horizontal(|ui| {
//                 ui.strong("Description: ");
//                 self.staging_description.draw_and_parse(ui, id.with("description"));
//             });
//             ui.horizontal(|ui| {
//                 ui.checkbox(&mut self.staging_auto_size, "Auto size");
//             });
//         });
//     }

//     fn state<'p>(&'p self) -> Self::Value<'p> {
//         let id = self.staging_id.state()?;
//     }
// }
