
use crate::search::writer::Children;
use super::super::*;

pub struct ShiftWriter {
    length: usize,
    next_op: u8,
    bytes_for_left: usize,
    left_child: FillerWriter,
    right_child: FillerWriter,
}

impl ShiftWriter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            next_op: LSL,
            bytes_for_left: length - 3,
            left_child: FillerWriter::new(length - 3),
            right_child: FillerWriter::new(1),
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.children.write(dest) {return true}

        // todo: re-instate quick-exit conditions.

        if self.partition.next() {
            self.children = Children::standard(OR, &self.partition.state());
            self.children.do_first_write(dest);
            return true;
        }

        false
    }
}

