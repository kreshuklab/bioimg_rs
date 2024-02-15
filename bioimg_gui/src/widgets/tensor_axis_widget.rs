use std::num::NonZeroUsize;

use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::rdf;

use super::axis_size_widget::AnyAxisSizeWidget;
use super::util::group_frame;
use super::{InputLines, StagingString, StagingVec, StatefulWidget, StagingNum};
use crate::result::{GuiError, Result};

pub struct BatchAxisWidget {
    pub staging_id: StagingString<modelrdf::axes::AxisId>,
    pub staging_description: StagingString<BoundedString<0, { 128 - 1 }>>,
    pub staging_allow_auto_size: bool,
}

impl Default for BatchAxisWidget {
    fn default() -> Self {
        Self {
            staging_id: StagingString {
                raw: "batch".into(),
                parsed: modelrdf::axes::AxisId::try_from("batch".to_owned()).map_err(GuiError::from),
                input_lines: InputLines::SingleLine,
            },
            staging_description: Default::default(),
            staging_allow_auto_size: true,
        }
    }
}

impl StatefulWidget for BatchAxisWidget {
    type Value<'p> = Result<modelrdf::axes::BatchAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Id: ");
                self.staging_id.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.staging_description.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.staging_allow_auto_size, "Allow auto size");
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::axes::BatchAxis {
            id: self.staging_id.state()?,
            description: self.staging_description.state()?,
            size: if self.staging_allow_auto_size {
                None
            } else {
                Some(rdf::LiteralInt::<1>)
            },
        })
    }
}

#[derive(Default)]
pub struct IndexAxisWidget {
    pub staging_id: StagingString<modelrdf::axes::AxisId>,
    pub staging_description: StagingString<BoundedString<0, { 128 - 1 }>>,
    pub staging_size: AnyAxisSizeWidget,
}

impl StatefulWidget for IndexAxisWidget {
    type Value<'p> = Result<modelrdf::axes::IndexAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Id: ");
                self.staging_id.draw_and_parse(ui, id.with("Id"));
            });

            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.staging_description.draw_and_parse(ui, id.with("Description"));
            });

            ui.horizontal(|ui| {
                ui.strong("Size: ");
                group_frame(ui, |ui| {
                    self.staging_size.draw_and_parse(ui, id.with("Size: "));
                });
            })
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::axes::IndexAxis {
            id: self.staging_id.state()?,
            description: self.staging_description.state()?,
            size: self.staging_size.state()?,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
enum ChannelNamesMode {
    #[default]
    Explicit,
    Pattern,
}

pub struct ChannelAxisWidget {
    pub staging_id: StagingString<modelrdf::axes::AxisId>,
    pub staging_description: StagingString<BoundedString<0, { 128 - 1 }>>,

    pub channel_names_mode: ChannelNamesMode,

    pub staging_pattern_extent: StagingNum<usize, NonZeroUsize>,
    pub staging_pattern_prefix: StagingString<String>,
    pub staging_pattern_suffix: StagingString<String>,

    pub staging_explicit_names: StagingVec<StagingString<rdf::Identifier<String>>>,
}

impl Default for ChannelAxisWidget{
    fn default() -> Self {
        Self{
            staging_id: Default::default(),
            staging_description: Default::default(),

            channel_names_mode: Default::default(),

            staging_pattern_extent: Default::default(),
            staging_pattern_prefix: Default::default(),
            staging_pattern_suffix: Default::default(),

            staging_explicit_names: StagingVec{item_name: "Channel Name".into(), staging: vec![]}
        }
    }
}

impl StatefulWidget for ChannelAxisWidget {
    type Value<'p> = Result<modelrdf::ChannelAxis>;
    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Id: ");
                self.staging_id.draw_and_parse(ui, id.with("id"));
            });
            ui.horizontal(|ui| {
                ui.strong("Description: ");
                self.staging_description.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Channel Names: ");
                ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Pattern, "Pattern");
                ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Explicit, "Explicit");
            });
            match self.channel_names_mode {
                ChannelNamesMode::Pattern => {
                    ui.horizontal(|ui| {
                        ui.strong("Extent: ");
                        self.staging_pattern_extent.draw_and_parse(ui, id.with("extent"));
                        
                        ui.strong("Prefix: ");
                        self.staging_pattern_prefix.draw_and_parse(ui, id.with("prefix"));

                        ui.strong("Suffix: ");
                        self.staging_pattern_suffix.draw_and_parse(ui, id.with("suffix"));
                    });
                }
                ChannelNamesMode::Explicit => {
                    self.staging_explicit_names.draw_and_parse(ui, id.with("explicit"));
                }
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let id = self.staging_id.state()?;
        let description = self.staging_description.state()?;
        
        let channel_names = match self.channel_names_mode{
            ChannelNamesMode::Pattern => {
                let extent: usize = self.staging_pattern_extent.state()?.into();
                (0..extent)
                    .map(|idx|{
                        let prefix = self.staging_pattern_prefix.state()?;
                        let suffix = self.staging_pattern_suffix.state()?;
                        let identifier = rdf::Identifier::<String>::try_from(format!("{prefix}{idx}{suffix}"))?;
                        Ok(identifier)
                    })
                    .collect::<Result<Vec<_>>>()?
            }
            ChannelNamesMode::Explicit => {
                let channel_names_result : Result<Vec<rdf::Identifier<_>>, GuiError> = self.staging_explicit_names
                    .state()
                    .into_iter()
                    .collect();
                channel_names_result?
            }
        };

        Ok(modelrdf::ChannelAxis{
            id,
            description,
            channel_names
        })
    }
}
