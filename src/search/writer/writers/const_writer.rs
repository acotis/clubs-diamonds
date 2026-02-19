
use super::super::*;

#[derive(Debug, Clone)]
pub struct ConstWriter {
    length: usize,
    next_write: u128,
    remove_neg_at: u128,
    stop_at: u128,
}

impl ConstWriter {
    pub fn new(length: usize, max_constant: u128) -> Self {
        match length {
             1 => {Self {length, next_write: 0, remove_neg_at: 0, stop_at: 10.min(max_constant)}},
             2 => {Self {length, next_write: 0, remove_neg_at: 10, stop_at: 100.min(max_constant)}}
             3 => {Self {length, next_write: 10, remove_neg_at: 100, stop_at: 1000.min(max_constant)}}
             4 => {Self {length, next_write: 100, remove_neg_at: 1000, stop_at: 10000.min(max_constant)}}
             5 => {Self {length, next_write: 1000, remove_neg_at: 10000, stop_at: 100000.min(max_constant)}}
             6 => {Self {length, next_write: 10000, remove_neg_at: 100000, stop_at: 1000000.min(max_constant)}}
             7 => {Self {length, next_write: 100000, remove_neg_at: 1000000, stop_at: 10000000.min(max_constant)}}
             8 => {Self {length, next_write: 1000000, remove_neg_at: 10000000, stop_at: 100000000.min(max_constant)}}
             9 => {Self {length, next_write: 10000000, remove_neg_at: 100000000, stop_at: 1000000000.min(max_constant)}}
            10 => {Self {length, next_write: 100000000, remove_neg_at: 1000000000, stop_at: 10000000000.min(max_constant)}}
            11 => {Self {length, next_write: 1000000000, remove_neg_at: 10000000000, stop_at: 100000000000.min(max_constant)}}
            12 => {Self {length, next_write: 10000000000, remove_neg_at: 100000000000, stop_at: 1000000000000.min(max_constant)}}
            13 => {Self {length, next_write: 100000000000, remove_neg_at: 1000000000000, stop_at: 10000000000000.min(max_constant)}}
            14 => {Self {length, next_write: 1000000000000, remove_neg_at: 10000000000000, stop_at: 100000000000000.min(max_constant)}}
            15 => {Self {length, next_write: 10000000000000, remove_neg_at: 100000000000000, stop_at: 1000000000000000.min(max_constant)}}
            16 => {Self {length, next_write: 100000000000000, remove_neg_at: 1000000000000000, stop_at: 10000000000000000.min(max_constant)}}
            17 => {Self {length, next_write: 1000000000000000, remove_neg_at: 10000000000000000, stop_at: 100000000000000000.min(max_constant)}}
            18 => {Self {length, next_write: 10000000000000000, remove_neg_at: 100000000000000000, stop_at: 1000000000000000000.min(max_constant)}}
            19 => {Self {length, next_write: 100000000000000000, remove_neg_at: 1000000000000000000, stop_at: 10000000000000000000.min(max_constant)}}
            20 => {Self {length, next_write: 1000000000000000000, remove_neg_at: 10000000000000000000, stop_at: 100000000000000000000.min(max_constant)}}
            21 => {Self {length, next_write: 10000000000000000000, remove_neg_at: 100000000000000000000, stop_at: 1000000000000000000000.min(max_constant)}}
            22 => {Self {length, next_write: 100000000000000000000, remove_neg_at: 1000000000000000000000, stop_at: 10000000000000000000000.min(max_constant)}}
            23 => {Self {length, next_write: 1000000000000000000000, remove_neg_at: 10000000000000000000000, stop_at: 100000000000000000000000.min(max_constant)}}
            24 => {Self {length, next_write: 10000000000000000000000, remove_neg_at: 100000000000000000000000, stop_at: 1000000000000000000000000.min(max_constant)}}
            25 => {Self {length, next_write: 100000000000000000000000, remove_neg_at: 1000000000000000000000000, stop_at: 10000000000000000000000000.min(max_constant)}}
            26 => {Self {length, next_write: 1000000000000000000000000, remove_neg_at: 10000000000000000000000000, stop_at: 100000000000000000000000000.min(max_constant)}}
            27 => {Self {length, next_write: 10000000000000000000000000, remove_neg_at: 100000000000000000000000000, stop_at: 1000000000000000000000000000.min(max_constant)}}
            28 => {Self {length, next_write: 100000000000000000000000000, remove_neg_at: 1000000000000000000000000000, stop_at: 10000000000000000000000000000.min(max_constant)}}
            29 => {Self {length, next_write: 1000000000000000000000000000, remove_neg_at: 10000000000000000000000000000, stop_at: 100000000000000000000000000000.min(max_constant)}}
            30 => {Self {length, next_write: 10000000000000000000000000000, remove_neg_at: 100000000000000000000000000000, stop_at: 1000000000000000000000000000000.min(max_constant)}}
            31 => {Self {length, next_write: 100000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000, stop_at: 10000000000000000000000000000000.min(max_constant)}}
            32 => {Self {length, next_write: 1000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000, stop_at: 100000000000000000000000000000000.min(max_constant)}}
            33 => {Self {length, next_write: 10000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000, stop_at: 1000000000000000000000000000000000.min(max_constant)}}
            34 => {Self {length, next_write: 100000000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000000, stop_at: 10000000000000000000000000000000000.min(max_constant)}}
            35 => {Self {length, next_write: 1000000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000000, stop_at: 100000000000000000000000000000000000.min(max_constant)}}
            36 => {Self {length, next_write: 10000000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000000, stop_at: 1000000000000000000000000000000000000.min(max_constant)}}
            37 => {Self {length, next_write: 100000000000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000000000, stop_at: 10000000000000000000000000000000000000.min(max_constant)}}
            38 => {Self {length, next_write: 1000000000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000000000, stop_at: 100000000000000000000000000000000000000.min(max_constant)}}
            39 => {Self {length, next_write: 10000000000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000000000, stop_at: 340282366920938463463374607431768211455.min(max_constant)}}
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

