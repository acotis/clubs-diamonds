
use clubs_diamonds::Expression;

// Note: These judges are implemented separately for each input type because
// for some reason this genuinely affects the performance compared to using
// generics, and a plain (non-generic) implementation is a more realistic
// use-case.
//
// If you need a copy of a judge that works for a different data type, simply
// copy the line of code that defines it and replace the data type. Leave
// enough space everywhere for the data type "usize" (5 columns) and leave
// enough space for the name to be 9 columns wide.

pub fn    u8_primes_11(expr: &Expression<   u8, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && expr.apply(&[4]) == Some(11) && true}
pub fn   u64_primes_11(expr: &Expression<  u64, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && expr.apply(&[4]) == Some(11) && true}

pub fn    u8_primes_7 (expr: &Expression<   u8, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn    i8_primes_7 (expr: &Expression<   i8, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   u16_primes_7 (expr: &Expression<  u16, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   i16_primes_7 (expr: &Expression<  i16, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   u32_primes_7 (expr: &Expression<  u32, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   i32_primes_7 (expr: &Expression<  i32, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   u64_primes_7 (expr: &Expression<  u64, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn   i64_primes_7 (expr: &Expression<  i64, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn  u128_primes_7 (expr: &Expression< u128, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn  i128_primes_7 (expr: &Expression< i128, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn usize_primes_7 (expr: &Expression<usize, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}
pub fn isize_primes_7 (expr: &Expression<isize, 1>) -> bool {expr.apply(&[0]) == Some(2) && expr.apply(&[1]) == Some(3) && expr.apply(&[2]) == Some(5) && expr.apply(&[3]) == Some(7) && true}

pub fn    i8_qubee    (expr: &Expression<   i8, 2>) -> bool {expr.apply(&[0, 0]) == Some(0) && expr.apply(&[0, 1]) == Some(0) && expr.apply(&[1, 0]) == Some(1) && expr.apply(&[1, 1]) == Some(0) && expr.apply(&[2, 0]) == Some(16) && expr.apply(&[2, 1]) == Some(8) && true}
pub fn   u32_qubee    (expr: &Expression<  u32, 2>) -> bool {expr.apply(&[0, 0]) == Some(0) && expr.apply(&[0, 1]) == Some(0) && expr.apply(&[1, 0]) == Some(1) && expr.apply(&[1, 1]) == Some(0) && expr.apply(&[2, 0]) == Some(16) && expr.apply(&[2, 1]) == Some(8) && true}
pub fn isize_qubee    (expr: &Expression<isize, 2>) -> bool {expr.apply(&[0, 0]) == Some(0) && expr.apply(&[0, 1]) == Some(0) && expr.apply(&[1, 0]) == Some(1) && expr.apply(&[1, 1]) == Some(0) && expr.apply(&[2, 0]) == Some(16) && expr.apply(&[2, 1]) == Some(8) && true}

pub fn   u64_rejectall(_xpr: &Expression<  u64, 1>) -> bool {false}
pub fn usize_113_shift(expr: &Expression<usize, 1>) -> bool {(1..10).all(|a| expr.apply(&[a]) == Some(113>>5%a))}
pub fn    u8_fortyfour(expr: &Expression<   u8, 1>) -> bool {(0..255).all(|a| expr.apply(&[a]) == Some(a^!0|44))}

