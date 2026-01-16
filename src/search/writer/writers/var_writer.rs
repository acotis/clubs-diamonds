
use super::super::*;

#[derive(Debug, Clone)]
pub struct VarWriter {
    length: usize,
    next_write: u8,
}

impl VarWriter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
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

