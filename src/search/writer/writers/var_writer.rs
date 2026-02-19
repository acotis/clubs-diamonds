
use super::super::*;

#[derive(Debug, Clone)]
pub struct VarWriter<const C: usize> {
    next_write: u8,
}

impl<const C: usize> VarWriter<C> {
    pub fn new(_length: usize, _constant_cap: u128) -> Self {
        Self {
            next_write: 0,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write == C as u8 {
            return false;
        }

        dest[0] = VarPivot(self.next_write).encode();
        self.next_write += 1;
        true
    }
}

