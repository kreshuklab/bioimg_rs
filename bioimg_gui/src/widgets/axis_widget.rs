use std::num::NonZeroUsize;

use bioimg_spec::rdf::non_empty_list::NonEmptyList;
use bioimg_spec::rdf;
use bioimg_spec::rdf::bounded_string::BoundedString;
use bioimg_spec::rdf::model::{self as modelrdf};

use super::channel_name_widget::ChannelNamesWidget;
use super::staging_string::StagingString;
use super::{StatefulWidget, ValueWidget};
use super::{axis_size_widget::AnyAxisSizeWidget, staging_num::StagingNum};
use crate::result::{GuiError, Result};

#[derive(Default)]
pub struct BatchAxisWidget {
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,
    pub staging_allow_auto_size: bool,
}

impl BatchAxisWidget{
    pub fn set_value(&mut self, value: modelrdf::BatchAxis){
        self.description_widget.raw = value.description.into();
        self.staging_allow_auto_size = value.size.is_none();
    }
}

impl StatefulWidget for BatchAxisWidget{
    type Value<'p> = Result<modelrdf::BatchAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Allow arbitrary batch size: ");
                ui.add(egui::widgets::Checkbox::without_text(&mut self.staging_allow_auto_size));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::BatchAxis{
            id: modelrdf::SpecialAxisId::new(),
            description: self.description_widget.state()?,
            size: self.staging_allow_auto_size.then_some(rdf::LiteralInt::<1>),
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum ChannelNamesMode {
    #[default]
    Explicit,
    Pattern,
}

#[derive(Default)]
pub struct ChannelAxisWidget {
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,

    pub channel_names_mode: ChannelNamesMode,
    pub channel_extent_widget: StagingNum<usize, NonZeroUsize>,
    pub channel_name_prefix_widget: StagingString<String>,
    pub channel_name_suffix_widget: StagingString<String>,

    pub staging_explicit_names: ChannelNamesWidget,
}

impl ChannelAxisWidget{
    pub fn set_value(&mut self, value: modelrdf::ChannelAxis){
        self.description_widget.raw = value.description.into();
        self.channel_names_mode = ChannelNamesMode::Explicit;
        self.staging_explicit_names.staging = Vec::from(value.channel_names).into_iter().map(|ident|{
            StagingString::new_with_raw(String::from(ident))
        }).collect();
    }
}

impl StatefulWidget for ChannelAxisWidget{
    type Value<'p> = Result<modelrdf::ChannelAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
            ui.horizontal(|ui| {
                ui.strong("Channel Names: ");
                ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Pattern, "Pattern");
                ui.radio_value(&mut self.channel_names_mode, ChannelNamesMode::Explicit, "Explicit");
            });
            match self.channel_names_mode {
                ChannelNamesMode::Pattern => {
                    ui.horizontal(|ui| {
                        ui.strong("Number of Channels: ");
                        self.channel_extent_widget.draw_and_parse(ui, id.with("extent"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Prefix: ");
                        self.channel_name_prefix_widget.draw_and_parse(ui, id.with("prefix"));
                    });
                    ui.horizontal(|ui| {
                        ui.strong("Suffix: ");
                        self.channel_name_suffix_widget.draw_and_parse(ui, id.with("suffix"));
                    });
                    if !self.channel_name_prefix_widget.raw.is_empty() || !self.channel_name_suffix_widget.raw.is_empty(){
                        ui.weak(format!(
                            "e.g.: Channel #7 will be named \"{}7{}\"",
                            &self.channel_name_prefix_widget.raw, &self.channel_name_suffix_widget.raw,
                        ));
                    }
                }
                ChannelNamesMode::Explicit => {
                    self.staging_explicit_names.draw_and_parse(ui, id.with("explicit"));
                }
            };
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        let channel_names: NonEmptyList<rdf::Identifier<String>> = match self.channel_names_mode {
            ChannelNamesMode::Pattern => {
                let extent: usize = self.channel_extent_widget.state()?.into();
                (0..extent)
                    .map(|idx| {
                        let prefix = self.channel_name_prefix_widget.state()?;
                        let suffix = self.channel_name_suffix_widget.state()?;
                        let identifier = rdf::Identifier::<String>::try_from(format!("{prefix}{idx}{suffix}"))?;
                        Ok(identifier)
                    })
                    .collect::<Result<Vec<_>>>()?
                    .try_into()
                    .map_err(|_| GuiError::new("Empty list of channel names".to_owned()))?

            }
            ChannelNamesMode::Explicit => {
                let channel_names_result: Result<Vec<rdf::Identifier<_>>, GuiError> =
                    self.staging_explicit_names.state().into_iter().collect();
                NonEmptyList::try_from(channel_names_result?)
                    .map_err(|_| GuiError::new("Empty list of channel names".to_owned()))?
            }
        };

        Ok(modelrdf::ChannelAxis {
            id: modelrdf::SpecialAxisId::new(),
            description: self.description_widget.state()?,
            channel_names
        })
    }
}

#[derive(Default)]
pub struct IndexAxisWidget {
    pub description_widget: StagingString<BoundedString<0, { 128 - 1 }>>,
    pub size_widget: AnyAxisSizeWidget,
}

impl IndexAxisWidget{
    pub fn set_value(&mut self, value: modelrdf::IndexAxis){
        self.description_widget.set_value(value.description);
        self.size_widget.set_value(value.size);
    }
}

impl StatefulWidget for IndexAxisWidget{
    type Value<'p> = Result<modelrdf::IndexAxis>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui|{
            ui.horizontal(|ui| {
                ui.strong("Axis Description: ");
                self.description_widget.draw_and_parse(ui, id.with("description"));
            });
        });
        ui.horizontal(|ui| {
            ui.strong("Size: ");
            self.size_widget.draw_and_parse(ui, id.with("size"));
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::IndexAxis {
            id: modelrdf::SpecialAxisId::new(),
            description: self.description_widget.state()?,
            size: self.size_widget.state()?
        })
    }
}
