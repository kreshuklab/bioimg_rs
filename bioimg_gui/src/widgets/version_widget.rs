use bioimg_spec::rdf;

use crate::result::Result;

use super::{staging_string::StagingString, StatefulWidget, ValueWidget};

#[derive(Default)]
pub struct VersionWidget{
    pub inner: StagingString<rdf::Version>,
}

impl StatefulWidget for VersionWidget{
    type Value<'p> = Result<rdf::Version>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        self.inner.draw_and_parse(ui, id);
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        self.inner.state()
    }
}

impl ValueWidget for VersionWidget{
    type Value<'v> = rdf::Version;
    fn set_value<'v>(&mut self, value: Self::Value<'v>) {
        self.inner.raw = value.into()
    }
} 
