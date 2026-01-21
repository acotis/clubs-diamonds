
use super::super::*;

#[derive(Debug, Clone)]
pub struct VarWriter {
    next_write: u8,
}

impl VarWriter {
    pub fn new(_length: usize) -> Self {
        Self {
            next_write: 1,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write > 1 {
            return false;
        }

        dest[0] = VarPivot(0).encode();
        self.next_write += 1;
        true
    }
}

