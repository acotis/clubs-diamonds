
use super::ThreadStatus;
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
    fn set_thread_statuses(&mut self, _thread_statuses: Vec<Option<ThreadStatus>>) {}
    fn finished_expression_length(&mut self, _length: usize, _count: u128) {}

    fn draw(&mut self) {}
    fn handle_inputs(&mut self) -> Vec<UISignal> {vec![]}
}

