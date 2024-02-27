use std::num::NonZeroUsize;

use crate::result::Result;
use bioimg_spec::rdf::model as modelrdf;
use bioimg_spec::rdf::model::{axes::AxisId, tensor_id::TensorId};

use super::staging_string::StagingString;
use super::util::group_frame;
use super::{StagingNum, StatefulWidget};

#[derive(Default)]
pub struct AxisSizeReferenceWidget {
    pub staging_tensor_id: StagingString<TensorId>,
    pub staging_axis_id: StagingString<AxisId>,
    pub staging_offset: StagingNum<usize, usize>,
}

impl AxisSizeReferenceWidget {
    pub fn replicate_state_on(&self, other: &mut Self) {
        other.staging_tensor_id.raw = self.staging_tensor_id.raw.clone();
        other.staging_axis_id.raw = self.staging_axis_id.raw.clone();
        other.staging_offset.raw = self.staging_offset.raw;
    }
}

impl StatefulWidget for AxisSizeReferenceWidget {
    type Value<'p> = Result<modelrdf::AxisSizeReference>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Tensor Id: ");
                self.staging_tensor_id.draw_and_parse(ui, id.with("Tensor Id"));
            });

            ui.horizontal(|ui| {
                ui.strong("Axis Id: ");
                self.staging_axis_id.draw_and_parse(ui, id.with("Axis Id"));
            });

            ui.horizontal(|ui| {
                ui.strong("Offset: ");
                self.staging_offset.draw_and_parse(ui, id.with("Offset"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::AxisSizeReference {
            qualified_axis_id: modelrdf::QualifiedAxisId {
                tensor_id: self.staging_tensor_id.state()?,
                axis_id: self.staging_axis_id.state()?,
            },
            offset: self.staging_offset.state()?,
        })
    }
}

#[derive(Default)]
pub struct ParameterizedAxisSizeWidget {
    pub staging_min: StagingNum<usize, NonZeroUsize>,
    pub staging_step: StagingNum<usize, NonZeroUsize>,
}

impl ParameterizedAxisSizeWidget {
    pub fn replicate_state_on(&self, other: &mut Self) {
        other.staging_min.raw = self.staging_min.raw;
        other.staging_step.raw = self.staging_step.raw;
    }
}

impl StatefulWidget for ParameterizedAxisSizeWidget {
    type Value<'p> = Result<modelrdf::ParameterizedAxisSize>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.strong("Min: ");
                self.staging_min.draw_and_parse(ui, id.with("Min"));
            });

            ui.horizontal(|ui| {
                ui.strong("Step: ");
                self.staging_step.draw_and_parse(ui, id.with("Step"));
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(modelrdf::ParameterizedAxisSize {
            min: self.staging_min.state()?,
            step: self.staging_step.state()?,
        })
    }
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum AxisSizeMode {
    Fixed,
    Reference,
    Parameterized,
}

impl Default for AxisSizeMode {
    fn default() -> Self {
        AxisSizeMode::Fixed
    }
}

#[derive(Default)]
pub struct AnyAxisSizeWidget {
    pub mode: AxisSizeMode,

    pub staging_fixed_size: StagingNum<usize, modelrdf::FixedAxisSize>,
    pub staging_size_ref: AxisSizeReferenceWidget,
    pub staging_parameterized: ParameterizedAxisSizeWidget,
}

impl StatefulWidget for AnyAxisSizeWidget {
    type Value<'p> = Result<modelrdf::AnyAxisSize>;

    fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.mode, AxisSizeMode::Fixed, "Fixed");
                ui.radio_value(&mut self.mode, AxisSizeMode::Parameterized, "Parameterized");
                ui.radio_value(&mut self.mode, AxisSizeMode::Reference, "Reference");
            });

            group_frame(ui, |ui| match self.mode {
                AxisSizeMode::Fixed => {
                    ui.horizontal(|ui| {
                        ui.strong("Extent: ");
                        self.staging_fixed_size.draw_and_parse(ui, id.with("Fixed"));
                    });
                }
                AxisSizeMode::Parameterized => {
                    self.staging_parameterized.draw_and_parse(ui, id.with("Parameterized"));
                }
                AxisSizeMode::Reference => {
                    self.staging_size_ref.draw_and_parse(ui, id.with("Reference"));
                }
            });
        });
    }

    fn state<'p>(&'p self) -> Self::Value<'p> {
        Ok(match self.mode {
            AxisSizeMode::Fixed => {
                modelrdf::AnyAxisSize::Resolved(modelrdf::ResolvedAxisSize::Fixed(self.staging_fixed_size.state()?))
            }
            AxisSizeMode::Parameterized => {
                modelrdf::AnyAxisSize::Resolved(modelrdf::ResolvedAxisSize::Parameterized(self.staging_parameterized.state()?))
            }
            AxisSizeMode::Reference => modelrdf::AnyAxisSize::Reference(self.staging_size_ref.state()?),
        })
    }
}
