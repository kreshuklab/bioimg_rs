use std::sync::Arc;

use bioimg_runtime::model_interface::{InputSlot, OutputSlot};
use bioimg_runtime::npy_array::ArcNpyArray;
use paste::paste;

use crate::result::{GuiError, Result};
use bioimg_spec::rdf::model as modelrdf;

use super::error_display::show_error;
use super::file_widget::{FileWidget, FileWidgetState};
use super::gui_npy_array::GuiNpyArray;
use super::staging_string::StagingString;
use super::staging_vec::StagingVec;
use super::input_axis_widget::InputAxisWidget;
use super::output_axis_widget::OutputAxisWidget;
use super::StatefulWidget;
use crate::widgets::staging_vec::ItemWidgetConf;

#[rustfmt::skip]
macro_rules!  declare_inout_tensor_widget {($inout:ident) => { paste!{
    pub struct [<$inout TensorWidget>] {
        pub id_widget: StagingString<modelrdf::TensorId>,
        pub description_widget: StagingString<modelrdf::TensorTextDescription>,
        pub axes_widget: StagingVec< [<$inout AxisWidget>] >,
        pub test_tensor_widget: FileWidget<Result<GuiNpyArray>>,
    }

    impl ItemWidgetConf for [<$inout TensorWidget>]{
        const ITEM_NAME: &'static str = concat!(stringify!($inout), " Tensor");
    }

    impl Default for [<$inout TensorWidget>] {
        fn default() -> Self {
            Self {
                id_widget: Default::default(),
                description_widget: Default::default(),
                axes_widget: StagingVec::default(),
                test_tensor_widget: Default::default(),
            }
        }
    }

    impl StatefulWidget for [<$inout TensorWidget>] {
        type Value<'p> = Result< [<$inout Slot>]<ArcNpyArray> >;

        fn draw_and_parse(&mut self, ui: &mut egui::Ui, id: egui::Id) {
            if let FileWidgetState::Finished { path, value: Ok(gui_npy_arr) } = self.test_tensor_widget.state() {
                if self.id_widget.raw.is_empty() {
                    self.id_widget.raw = path
                        .file_stem()
                        .map(|osstr| String::from(osstr.to_string_lossy()))
                        .unwrap_or(String::default());
                }
                let sample_shape = gui_npy_arr.shape();
                let mut extents = sample_shape.iter().skip(self.axes_widget.staging.len());

                while let Some(extent) = extents.next() {
                    let mut axis_widget = [<$inout AxisWidget>]::default();
                    axis_widget.axis_type = modelrdf::AxisType::Space;
                    axis_widget.space_axis_widget.prefil_parameterized_size(*extent);
                    self.axes_widget.staging.push(axis_widget)
                }
            };
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.strong("Test Tensor: ");
                    self.test_tensor_widget.draw_and_parse(ui, id.with("test tensor"));
                    if matches!(self.test_tensor_widget.state(), FileWidgetState::Empty) {
                        show_error(ui, "Missing a npy test tensor");
                    }
                });
                ui.horizontal(|ui| {
                    ui.strong("Tensor Id: ");
                    self.id_widget.draw_and_parse(ui, id.with("Id"));
                });
                ui.horizontal(|ui| {
                    ui.strong("Description: ");
                    self.description_widget.draw_and_parse(ui, id.with("Description"));
                });
                ui.horizontal(|ui| {
                    ui.strong("Axes: ");
                    self.axes_widget.draw_and_parse(ui, id.with("Axes"));
                });
            });
        }

        fn state<'p>(&'p self) -> Self::Value<'p> {
            let FileWidgetState::Finished { value: Ok(gui_npy_array), .. } = self.test_tensor_widget.state() else {
                return Err(GuiError::new("Test tensor is missing".into()));
            };
            let axes = self.axes_widget.state().into_iter().collect::<Result<Vec<_>>>()?;
            let input_axis_group = modelrdf::[<$inout AxisGroup>]::try_from(axes)?; //FIXME: parse in draw_and_parse?
            Ok( [<$inout Slot>] {
                id: self.id_widget.state()?,
                description: self.description_widget.state()?,
                axes: input_axis_group,
                test_tensor: Arc::clone(gui_npy_array.contents()),
            })
        }
    }
}};}

declare_inout_tensor_widget!(Input);
declare_inout_tensor_widget!(Output);
