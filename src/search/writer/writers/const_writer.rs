
use super::super::*;

#[derive(Debug)]
pub struct ConstWriter {
    length: usize,
    next_write: u8,
}

impl ConstWriter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            next_write: 1,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write == 4 || self.length == 1 && self.next_write == 3 {
            return false;
        }

        if self.length == 2 {
            dest[1] = OpPivot(Op::NOT).encode();
        }

        dest[0] = ConstPivot(self.next_write).encode();
        self.next_write += 1;
        true
    }
}

