
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct ShiftWriter<N: Number, const C: usize> {
    length: usize,
    constant_cap: u8,
    next_op: u8,
    bytes_for_left: usize,
    children: Children<N, C>,
}

impl<N: Number, const C: usize> ShiftWriter<N, C> {
    pub fn new(length: usize, constant_cap: u8) -> Self {
        Self {
            length,
            constant_cap,
            next_op: LSL,
            bytes_for_left: length - 3,
            children: Children::two_context(
                LEFT_CHILD_OF_SHIFT,
                RIGHT_CHILD_OF_SHIFT,
                constant_cap,
                Nop.encode(), // we write our op manually
                &[length - 3, 1],
            ).allow_multi_constants().non_commutative(),
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            if self.next_op == LSL {
                if self.children.write(dest) {
                    dest[self.length-2] = LSL;
                    dest[self.length-1] = Nop.encode();
                    self.next_op = LSR;
                    return true;
                }
            } else {
                dest[self.length-2] = LSR;
                self.next_op = LSL;
                return true;
            }

            if self.bytes_for_left > 1 {
                self.bytes_for_left -= 1;
                self.children = Children::two_context(
                    LEFT_CHILD_OF_SHIFT,
                    RIGHT_CHILD_OF_SHIFT,
                    self.constant_cap,
                    Nop.encode(), // we write our op manually
                    &[self.bytes_for_left, self.length - self.bytes_for_left - 2],
                ).allow_multi_constants().non_commutative();

                continue;
            }

            return false;
        }
    }
}

