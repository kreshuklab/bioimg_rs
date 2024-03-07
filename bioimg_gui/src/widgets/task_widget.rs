use std::thread::JoinHandle;

#[derive(Default)]
pub enum TaskState<Out> {
    #[default]
    Nothing,
    Processing {
        task: JoinHandle<Out>,
    },
    Done {
        result: Out,
    },
}

pub struct RenderTaskStateParams<'a, Out, F>
where
    F: FnOnce() -> Out,
{
    ui: &'a mut egui::Ui,
    state: &'a mut TaskState<Out>,
    button_text: &'a str,
    target: F,
}

#[allow(dead_code)]
pub fn render_task_state<Meta, Out, F>(params: RenderTaskStateParams<'_, Out, F>)
where
    F: FnOnce() -> Out,
    F: Send + 'static,
    Out: Send + 'static,
{
    let button_clicked = params.ui.button(params.button_text).clicked();
    *params.state = match std::mem::take(params.state) {
        TaskState::Nothing => {
            if button_clicked {
                TaskState::Processing { task: std::thread::spawn(params.target) }
            } else {
                TaskState::Nothing
            }
        }
        TaskState::Processing { task } => {
            if task.is_finished() {
                TaskState::Done { result: task.join().unwrap() }
            } else {
                params.ui.label("Processing");
                TaskState::Processing { task }
            }
        }
        TaskState::Done { result } => TaskState::Done { result },
    }
}
