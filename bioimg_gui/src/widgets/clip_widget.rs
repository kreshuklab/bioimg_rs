use bioimg_spec::rdf::model as modelrdf;

use super::{staging_num::StagingNum, StatefulWidget};

use crate::result::{GuiError, Result};

pub struct ClipWidget{
    pub min_widget: StagingNum<f32, f32>,
    pub max_widget: StagingNum<f32, f32>,
}

impl StatefulWidget for ClipWidget{
    type Value<'p> = Result<modelrdf::preprocessing::ClipDescr>;
}
