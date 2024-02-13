use bioimg_spec::rdf::bounded_string::BoundedString;

use super::StagingString;

pub struct InputTensorWidget {
    staging_id: StagingString<BoundedString<1, 1023>>,
}
