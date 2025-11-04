
use super::Thread;
use super::UI;
use super::UISignal;

pub struct NullUI;

impl UI for NullUI {
    fn new() -> Self {NullUI}

    fn set_description(&mut self, _description: String) {}
    fn set_inspector_enabled(&mut self, _enabled: bool) {}
    fn push_solution(&mut self, _face: String, _score: usize, _inspection: Option<String>) {}
    fn set_total_count(&mut self, _total_count: u128) {}
    fn set_target_thread_count(&mut self, _target_thread_count: usize) {}
    fn set_thread_statuses(&mut self, _thread_statuses: Vec<Thread>) {}
    fn push_news_item(&mut self, _news_item: String) {}

    fn draw(&mut self) {}
    fn handle_inputs(&mut self) -> Vec<UISignal> {vec![]}
}

