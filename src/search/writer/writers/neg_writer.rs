
use crate::search::writer::Partition;
use crate::search::writer::Children;
use super::super::*;

#[derive(Debug)]
pub struct NegWriter {
    length: usize,
    next_op: u8,
    child: Box<Writer>,
}

impl NegWriter {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            child: Box::new(Writer::new(length - 1, WriterContext {location: CHILD_OF_NEG, const_allowed: false})),
            next_op: NEG,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_op == NEG {
            if self.child.write(dest) {
                dest[self.length-1] = NEG;
                self.next_op = NOT;
                return true;
            }
        } else {
            dest[self.length-1] = NOT;
            self.next_op = NEG;
            return true;
        }

        return false;
    }
}

