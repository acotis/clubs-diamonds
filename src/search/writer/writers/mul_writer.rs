
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct MulWriter<N: Number> {
    length: usize,
    next_op: u8,
    bytes_for_left: usize,
    children: Children<N>,
}

impl<N: Number> MulWriter<N> {
    pub fn new(length: usize) -> Self {
        Self {
            length,
            next_op: MUL,
            bytes_for_left: length - 2,
            children: Children::two_context(
                LEFT_CHILD_OF_MUL,
                RIGHT_CHILD_OF_MUL,
                Nop.encode(), // we write our op manually
                &[length - 2, 1],
            ).non_commutative(),
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            match self.next_op {
                MUL => {
                    if self.children.write(dest) {
                        dest[self.length-1] = MUL;
                        self.next_op = DIV;
                        return true;
                    }
                }
                DIV => {
                    dest[self.length-1] = DIV;
                    self.next_op = MOD;
                    return true;
                }
                MOD => {
                    dest[self.length-1] = MOD;
                    self.next_op = MUL;
                    return true;
                }
                _ => panic!(),
            }

            if self.bytes_for_left > 1 {
                self.bytes_for_left -= 1;
                self.children = Children::two_context(
                    LEFT_CHILD_OF_MUL,
                    RIGHT_CHILD_OF_MUL,
                    Nop.encode(), // we write our op manually
                    &[self.bytes_for_left, self.length - self.bytes_for_left - 1],
                ).non_commutative();

                continue;
            }

            return false;
        }
    }
}

