
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
             1 => {Self {length, next_write: 0, remove_neg_at: 0, stop_at: 10.min(constant_cap)}},
             2 => {Self {length, next_write: 0, remove_neg_at: 10, stop_at: 100.min(constant_cap)}}
             3 => {Self {length, next_write: 10, remove_neg_at: 100, stop_at: 1000.min(constant_cap)}}
             4 => {Self {length, next_write: 100, remove_neg_at: 1000, stop_at: 10000.min(constant_cap)}}
             5 => {Self {length, next_write: 1000, remove_neg_at: 10000, stop_at: 100000.min(constant_cap)}}
             6 => {Self {length, next_write: 10000, remove_neg_at: 100000, stop_at: 1000000.min(constant_cap)}}
             7 => {Self {length, next_write: 100000, remove_neg_at: 1000000, stop_at: 10000000.min(constant_cap)}}
             8 => {Self {length, next_write: 1000000, remove_neg_at: 10000000, stop_at: 100000000.min(constant_cap)}}
             9 => {Self {length, next_write: 10000000, remove_neg_at: 100000000, stop_at: 1000000000.min(constant_cap)}}
            10 => {Self {length, next_write: 100000000, remove_neg_at: 1000000000, stop_at: 10000000000.min(constant_cap)}}
            11 => {Self {length, next_write: 1000000000, remove_neg_at: 10000000000, stop_at: 100000000000.min(constant_cap)}}
            12 => {Self {length, next_write: 10000000000, remove_neg_at: 100000000000, stop_at: 1000000000000.min(constant_cap)}}
            13 => {Self {length, next_write: 100000000000, remove_neg_at: 1000000000000, stop_at: 10000000000000.min(constant_cap)}}
            14 => {Self {length, next_write: 1000000000000, remove_neg_at: 10000000000000, stop_at: 100000000000000.min(constant_cap)}}
            15 => {Self {length, next_write: 10000000000000, remove_neg_at: 100000000000000, stop_at: 1000000000000000.min(constant_cap)}}
            16 => {Self {length, next_write: 100000000000000, remove_neg_at: 1000000000000000, stop_at: 10000000000000000.min(constant_cap)}}
            17 => {Self {length, next_write: 1000000000000000, remove_neg_at: 10000000000000000, stop_at: 100000000000000000.min(constant_cap)}}
            18 => {Self {length, next_write: 10000000000000000, remove_neg_at: 100000000000000000, stop_at: 1000000000000000000.min(constant_cap)}}
            19 => {Self {length, next_write: 100000000000000000, remove_neg_at: 1000000000000000000, stop_at: 10000000000000000000.min(constant_cap)}}
            20 => {Self {length, next_write: 1000000000000000000, remove_neg_at: 10000000000000000000, stop_at: 100000000000000000000.min(constant_cap)}}
            21 => {Self {length, next_write: 10000000000000000000, remove_neg_at: 100000000000000000000, stop_at: 1000000000000000000000.min(constant_cap)}}
            22 => {Self {length, next_write: 100000000000000000000, remove_neg_at: 1000000000000000000000, stop_at: 10000000000000000000000.min(constant_cap)}}
            23 => {Self {length, next_write: 1000000000000000000000, remove_neg_at: 10000000000000000000000, stop_at: 100000000000000000000000.min(constant_cap)}}
            24 => {Self {length, next_write: 10000000000000000000000, remove_neg_at: 100000000000000000000000, stop_at: 1000000000000000000000000.min(constant_cap)}}
            25 => {Self {length, next_write: 100000000000000000000000, remove_neg_at: 1000000000000000000000000, stop_at: 10000000000000000000000000.min(constant_cap)}}
            26 => {Self {length, next_write: 1000000000000000000000000, remove_neg_at: 10000000000000000000000000, stop_at: 100000000000000000000000000.min(constant_cap)}}
            27 => {Self {length, next_write: 10000000000000000000000000, remove_neg_at: 100000000000000000000000000, stop_at: 1000000000000000000000000000.min(constant_cap)}}
            28 => {Self {length, next_write: 100000000000000000000000000, remove_neg_at: 1000000000000000000000000000, stop_at: 10000000000000000000000000000.min(constant_cap)}}
            29 => {Self {length, next_write: 1000000000000000000000000000, remove_neg_at: 10000000000000000000000000000, stop_at: 100000000000000000000000000000.min(constant_cap)}}
            30 => {Self {length, next_write: 10000000000000000000000000000, remove_neg_at: 100000000000000000000000000000, stop_at: 1000000000000000000000000000000.min(constant_cap)}}
            31 => {Self {length, next_write: 100000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000, stop_at: 10000000000000000000000000000000.min(constant_cap)}}
            32 => {Self {length, next_write: 1000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000, stop_at: 100000000000000000000000000000000.min(constant_cap)}}
            33 => {Self {length, next_write: 10000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000, stop_at: 1000000000000000000000000000000000.min(constant_cap)}}
            34 => {Self {length, next_write: 100000000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000000, stop_at: 10000000000000000000000000000000000.min(constant_cap)}}
            35 => {Self {length, next_write: 1000000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000000, stop_at: 100000000000000000000000000000000000.min(constant_cap)}}
            36 => {Self {length, next_write: 10000000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000000, stop_at: 1000000000000000000000000000000000000.min(constant_cap)}}
            37 => {Self {length, next_write: 100000000000000000000000000000000000, remove_neg_at: 1000000000000000000000000000000000000, stop_at: 10000000000000000000000000000000000000.min(constant_cap)}}
            38 => {Self {length, next_write: 1000000000000000000000000000000000000, remove_neg_at: 10000000000000000000000000000000000000, stop_at: 100000000000000000000000000000000000000.min(constant_cap)}}
            39 => {Self {length, next_write: 10000000000000000000000000000000000000, remove_neg_at: 100000000000000000000000000000000000000, stop_at: 340282366920938463463374607431768211455.min(constant_cap)}}
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

