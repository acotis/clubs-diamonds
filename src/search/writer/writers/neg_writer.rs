
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct NegWriter<N: Number> {
    length: usize,
    next_op: u8,
    child: Box<Writer<N>>,
}

impl<N: Number> NegWriter<N> {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            child: Box::new(Writer::new(length - 1, WriterContext {location: CHILD_OF_NEG, const_allowed: false})),
            next_op: NOT,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_op == NOT {
            if self.child.write(dest) {
                dest[self.length-1] = NOT;
                self.next_op = if N::is_signed() {NEG} else {NOT};
                return true;
            }
        } else {
            dest[self.length-1] = NEG;
            self.next_op = NOT;
            return true;
        }

        return false;
    }
}

