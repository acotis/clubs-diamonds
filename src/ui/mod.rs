
mod default_ui;

pub use default_ui::DefaultUI;

use crate::search::Thread;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UISignal {
    Quit,
    IncreaseThreadCount,
    DecreaseThreadCount,
}

pub trait UI {
    fn new() -> Self;

    fn set_description(&mut self, description: String);
    fn push_solution(&mut self, face: String, score: usize, inspection: Option<String>);
    fn set_total_count(&mut self, total_count: u128);
    fn set_target_thread_count(&mut self, target_thread_count: usize);
    fn set_thread_statuses(&mut self, thread_statuses: Vec<Thread>);
    fn push_news_item(&mut self, news_item: String);

    fn draw(&mut self);
    fn handle_inputs(&mut self) -> Vec<UISignal>;
}

