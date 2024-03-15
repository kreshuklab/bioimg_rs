use bioimg_spec::rdf::Version;

use super::{author_widget::StagingAuthor2, staging_opt::StagingOpt, staging_string::StagingString, staging_vec::StagingVec, LocalFileRefWidget, StatefulWidget};

pub struct WeightsWidget{

}

// impl StatefulWidget for WeightsWidget{
//     fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id){

//     }
// }

struct WeightsDescrBaseWidget{
    pub source_widget: LocalFileRefWidget,
    pub authors_widget: StagingOpt<StagingVec<StagingAuthor2>>,
    // pub parent_widget: Option<WeightsFormat>,
}

impl WeightsDescrBaseWidget{
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui|{
                ui.strong("Source: ");
                self.source_widget.draw_and_parse(ui, id.with("source"));
            });
            ui.horizontal(|ui|{
                ui.strong("Authors: ");
                self.authors_widget.draw_and_parse(ui, id.with("authors"));
            });
        });
    }
}



pub struct KerasHdf5WeightsWidget{
    pub base_widget: WeightsDescrBaseWidget,
    pub tensorflow_version_widget: StagingString<Version>,
}

// impl StatefulWidget for KerasHdf5WeightsWidget{
//     fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
//         ui.vertical(|ui|{
//             self.base_widget.draw_and_parse(ui, id.with("base"));
//             ui.horizontal(|ui|{
//                 ui.strong("Tensor Flow Version: ");
//                 self.tensorflow_version_widget.draw_and_parse(ui, id.with("tfversion"));
//             });
//         })
//     }
// }

