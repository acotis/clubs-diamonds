
use super::UI;
use super::UISignal;

pub struct NullUI;

impl UI for NullUI {
    fn new() -> Self {NullUI}
    fn require_manual_exit() -> bool {false}
    fn handle_inputs(&mut self) -> Vec<UISignal> {vec![]}
}

