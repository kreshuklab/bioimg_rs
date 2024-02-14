// use crate::rdf::{bounded_string::BoundedString, identifier::Identifier, model as modelrdf};

// pub enum InvalidRdfChannelAxis {
//     BadNumberOfChannels,
// }

// pub struct ChannelAxis {
//     pub id: modelrdf::axes2::AxisId,
//     pub description: BoundedString<0, { 128 - 1 }>,
//     pub channel_names: Vec<Identifier<String>>,
// }

// impl TryFrom<modelrdf::axes2::ChannelAxis> for ChannelAxis {
//     fn try_from(value: modelrdf::axes2::ChannelAxis) -> Result<Self, Self::Error> {}
// }
