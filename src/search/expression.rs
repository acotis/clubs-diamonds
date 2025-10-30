
use std::marker::PhantomData;

use crate::search::number::Number;
use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op::{self, *};

/// Represents a syntactically-valid mathematical Rust expression. Can be applied to a set of input values to yield a result value. Can also be rendered as text using the `format!` macro or `.to_string()` method.
///
/// Currently, the only way to get your hands on an `Expression` is to be given it by a `Searcher`.

#[derive(Clone)]
pub struct Expression<N: Number, const C: usize> {
    pub(super) field: Vec<u8>,
    pub(super) nothing: PhantomData<N>,
}

impl<N: Number, const C: usize> Expression<N, C> {
    /// Apply this expression to an array of input values for its variables. Returns a result wrapped in an `Option`. The value `None` is returned if applying the expression to the given values would result in a runtime exception (for example, if it would end up dividing by zero).
    ///
    /// The length and entry type of the array must match the type parameters of the Expression. If the expression came from a `Searcher::<u32, 3>`, then it is an `Expression<u32, 3>` and you must supply an array of three `u32` values to this method.
    ///
    /// When applying the expression, the first entry in the array is assigned to the variable `a`, the second to `b`, the third to `c`, and so on.
    ///
    /// Example: when the expression `a*c+53%b` is applied to the array `[2, 10, 4]`, it will yield `Some(11)`.
    ///
    /// Example #2: when the expression `44/a` is applied to the array `[0]`, it will yield `None`.

    pub fn apply(&self, inputs: &[N; C]) -> Option<N> {
        let zero = N::from_u8(0);

        let mut stack = [zero; 99];
        let mut pointer = 0;

        for index in (0..self.field.len()).rev() {
            match Op::interpret_code(self.field[index]) {
                Nop           => {/* do nothing */},
                OpPivot(NOT)  => {stack[pointer-1]  =! stack[pointer-1];}
                OpPivot(MUL)  => {stack[pointer-2]  *= stack[pointer-1]; pointer -= 1;},
                OpPivot(DIV)  => {
                    if stack[pointer-1] == zero {return None}
                    if N::is_signed() && stack[pointer-1] == !zero && stack[pointer-2] == <N as Number>::min() {return None}
                    stack[pointer-2]  /= stack[pointer-1]; pointer -= 1;
                },
                OpPivot(MOD)  => {
                    if stack[pointer-1] == zero {return None}
                    if N::is_signed() && stack[pointer-1] == !zero && stack[pointer-2] == <N as Number>::min() {return None}
                    stack[pointer-2]  %= stack[pointer-1]; pointer -= 1;
                },
                OpPivot(ADD)  => {stack[pointer-2]  += stack[pointer-1]; pointer -= 1;},
                OpPivot(SUB)  => {stack[pointer-2]  -= stack[pointer-1]; pointer -= 1;},
                OpPivot(LSL)  => {stack[pointer-2] <<= stack[pointer-1]; pointer -= 1;},
                OpPivot(LSR)  => {stack[pointer-2] >>= stack[pointer-1]; pointer -= 1;},
                OpPivot(AND)  => {stack[pointer-2]  &= stack[pointer-1]; pointer -= 1;},
                OpPivot(XOR)  => {stack[pointer-2]  ^= stack[pointer-1]; pointer -= 1;},
                OpPivot(ORR)  => {stack[pointer-2]  |= stack[pointer-1]; pointer -= 1;},
                ConstPivot(p) => {stack[pointer] = N::from_u8(p); pointer += 1;},
                VarPivot(v)   => {stack[pointer] = inputs[v as usize]; pointer += 1;},
            }
        }

        Some(stack[0])
    }

    fn stringify(&self, start: usize) -> (String, usize, usize) {
        if start >= self.field.len() {
            for i in 0..self.field.len() {
                print!("{:?} ", Op::interpret_code(self.field[i]));
            }
            println!();
        }

        match Op::interpret_code(self.field[start]) {
            Nop           => {let (a, b, c) = self.stringify(start+1); (a, b, c+1)},
            ConstPivot(p) => (format!("{p}"),                  !0, 1),
            VarPivot(v)   => (format!("{}", (v + 97) as char), !0, 1),
            OpPivot(op)   => {
                if op.arity() == 1 {
                    let (right, right_prec, right_len) = self.stringify(start + 1);
                    let right_render = if right_prec >= op.prec() {right} else {format!("({right})")};

                    (format!("{}{}", op.render_face(), right_render), op.prec(), 1 + right_len)
                } else {
                    let (right, right_prec, right_len) = self.stringify(start + 1);
                    let (left,  left_prec,  left_len ) = self.stringify(start + 1 + right_len);

                    let left_render  = if left_prec  >= op.prec() {left } else {format!("({left})")};
                    let right_render = if right_prec >  op.prec() {right} else {format!("({right})")};

                    (format!("{}{}{}", left_render, op.render_face(), right_render), op.prec(), 1 + left_len + right_len)
                }
            }
        }
    }
}

impl<N: Number, const C: usize> std::fmt::Display for Expression<N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.stringify(0).0)
    }
}

