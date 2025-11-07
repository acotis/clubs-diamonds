
mod utils;
mod default_ui;
mod null_ui;

pub use default_ui::DefaultUI;
pub use null_ui::NullUI;

// Data sent from the manager thread to the UI.

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Thread {
    pub id: usize,
    pub status: Option<ThreadStatus>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ThreadStatus {
    Searching(String),
    Paused,
}

// Data sent from the UI to the manager thread.

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UISignal {
    Quit,
    IncreaseThreadCount,
    DecreaseThreadCount,
}

// The trait itself.

pub trait UI {
    fn new() -> Self;

    fn set_description(&mut self, description: String);
    fn set_inspector_enabled(&mut self, enabled: bool);
    fn push_solution(&mut self, face: String, score: usize, inspection: Option<String>);
    fn set_total_count(&mut self, total_count: u128);
    fn set_target_thread_count(&mut self, target_thread_count: usize);
    fn set_thread_statuses(&mut self, thread_statuses: Vec<Thread>);
    fn finished_expression_length(&mut self, length: usize, count: u128);

    fn draw(&mut self);
    fn handle_inputs(&mut self) -> Vec<UISignal>;
}

