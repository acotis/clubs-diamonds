
use super::super::*;

#[derive(Debug, Clone)]
pub struct ConstWriter {
    length: usize,
    next_write: u128,
    remove_neg_at: u128,
    stop_at: u128,
}

impl ConstWriter {
    pub fn new(length: usize, constant_cap: u128) -> Self {
        match length {
            1 => {
                Self {
                    length,
                    next_write: 0,
                    remove_neg_at: 0,
                    stop_at: 10.min(constant_cap),
                }
            },
            2 => {
                Self {
                    length,
                    next_write: 0,
                    remove_neg_at: 10,
                    stop_at: 100.min(constant_cap),
                }
            }
            3 => {
                Self {
                    length,
                    next_write: 10,
                    remove_neg_at: 100,
                    stop_at: 156.min(constant_cap),
                }
            }
            4 => {
                Self {
                    length,
                    next_write: 100,
                    remove_neg_at: 156,
                    stop_at: 156.min(constant_cap),
                }

            }
            _ => panic!(),
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.next_write >= self.stop_at {
            return false;
        }

        // TODO: stop doing this part??
        dest[..self.length].fill(Nop.encode());

        let mut digit_pos = self.length - 1;
        let mut value_remaining = self.next_write;

        if self.next_write < self.remove_neg_at {
            dest[digit_pos] = OpPivot(Op::NOT).encode();
            digit_pos -= 1;
        }

        loop {
            dest[digit_pos] = if value_remaining > 63 {
                ContinuationDigit(value_remaining as u8 & 63).encode()
            } else {
                FirstDigit(value_remaining as u8).encode()
            };

            value_remaining >>= 6;
            digit_pos -= 1;

            if value_remaining == 0 {
                break; // break here so that we do write a FirstDigit(0) for the constant 0.
            }
        }

        self.next_write += 1;
        true
    }
}

