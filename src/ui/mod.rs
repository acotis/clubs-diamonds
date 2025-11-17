
mod utils;
mod default_ui;
mod null_ui;

pub use default_ui::DefaultUI;
pub use null_ui::NullUI;

// Data sent from the manager thread to the UI.

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ThreadStatus {
    Searching(String),
    Paused(String),
}

// Data sent from the UI to the manager thread.

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UISignal {
    Quit,
    IncreaseThreadCount,
    DecreaseThreadCount,
    PauseUnpause,
}

// The trait itself.

pub trait UI {
    fn new() -> Self;
    fn require_manual_exit() -> bool;

    fn set_description(&mut self, _description: String) {}
    fn set_inspector_enabled(&mut self, _enabled: bool) {}
    fn set_debug_banner_enabled(&mut self, _enabled: bool) {}
    fn push_solution(&mut self, _face: String, _score: usize, _inspection: Option<String>) {}
    fn set_total_count(&mut self, _total_count: u128) {}
    fn set_target_thread_count(&mut self, _target_thread_count: usize) {}
    fn set_thread_statuses(&mut self, _thread_statuses: Vec<Option<ThreadStatus>>) {}
    fn finished_expression_length(&mut self, _length: usize, _count: u128) {}
    fn draw(&mut self) {}

    fn handle_inputs(&mut self) -> Vec<UISignal>;
}

