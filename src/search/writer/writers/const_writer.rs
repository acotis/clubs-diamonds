
use super::super::*;

#[derive(Debug)]
pub struct ConstWriter {
    length: usize,
    next_write: u8,
    remove_neg_at: u8,
    stop_at: u8
}

impl ConstWriter {
    pub fn new(length: usize) -> Self {
        match length {
            1 => {
                Self {
                    length,
                    next_write: 1, // todo: should this be 0?
                    remove_neg_at: 1,
                    stop_at: 10,
                }
            },
            2 => {
                Self {
                    length,
                    next_write: 1,
                    remove_neg_at: 10,
                    stop_at: 100,
                }
            }
            3 => {
                Self {
                    length,
                    next_write: 10,
                    remove_neg_at: 100,
                    stop_at: 156,
                }
            }
            _ => panic!(),
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write == self.stop_at {
            return false;
        }

        dest[..self.length].fill(Nop.encode());

        if self.next_write >= self.remove_neg_at {
            dest[self.length-1] = ConstPivot(self.next_write).encode();
        } else {
            dest[self.length-2] = ConstPivot(self.next_write).encode();
            dest[self.length-1] = OpPivot(Op::NOT).encode();
        }

        self.next_write += 1;
        true
    }
}

