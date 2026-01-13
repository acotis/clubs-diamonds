
use crate::search::writer::Partition;
use crate::search::writer::Children;
use super::super::*;

pub struct AtomWriter {
    length: usize,
    next_write: u8,
}

impl AtomWriter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            next_write: if length == 1 {
                1
            } else {
                10
            }
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write == 4 || self.next_write == 40 {
            return false;
        }

        match self.next_write {
            1..=2   => {dest[0] = ConstPivot(self.next_write).encode();                         self.next_write += 1;}
            10..=40 => {dest[0] = ConstPivot(self.next_write).encode(); dest[1] = Nop.encode(); self.next_write += 10;}
            3       => {dest[0] = VarPivot(0).encode(); self.next_write += 1;}
            _ => panic!()
        }

        true
    }
}

