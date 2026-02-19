
mod display;
mod revar;
mod fromstr;

pub use revar::*;

use std::marker::PhantomData;

use crate::search::pivot::Pivot;
use crate::search::number::Number;
use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op::*;

/// Represents a syntactically-valid mathematical Rust expression. Can be applied to a set of input values to yield an output value. Can also be rendered as text using the `format!` macro or `.to_string()` method.
///
/// [`Expression`] implements [`FromStr`][std::str::FromStr], so you can parse expressions from a strings in addition to receiving them from the [`Searcher`][crate::Searcher] struct.

#[derive(Clone, Debug)]
pub struct Expression<N: Number, const C: usize> {
    pub (super) field: Vec<u8>,
    pub (super) nothing: PhantomData<N>,
}

impl<N: Number, const C: usize> Expression<N, C> {
    /// Apply this expression to an array of input values. Returns a result wrapped in an [`Option`]. The value `None` is returned if applying the expression to the given values would result in a runtime exception (for example, if it would end up dividing by zero).
    ///
    /// The length and entry type of the array must match the type parameters of the expression. If the expression came from a `Searcher::<u32, 3>`, then it is an `Expression::<u32, 3>` and the argument to this function is a `&[u32; 3]`.
    ///
    /// When applying the expression, the first value in the array is assigned to the variable `a`, the second to `b`, the third to `c`, and so on.
    ///
    /// Example: when the expression `a*c+53%b` is applied to the array `[2, 10, 4]`, it will yield `Some(11)`.
    ///
    /// Example #2: when the expression `4/a` is applied to the array `[0]`, it will yield `None`.

    pub fn apply(&self, inputs: &[N; C]) -> Option<N> {
        let mut stack = [N::from_u8(0); 99];
        let mut pointer = 0;

        for code in &self.field {
            match Pivot::decode(*code) {
                Nop => {},
                OpPivot(NEG)  => {stack[pointer-1] = N::from_u8(0).wrapping_sub(&stack[pointer-1])}
                OpPivot(NOT)  => {stack[pointer-1] = !stack[pointer-1];}
                OpPivot(MUL)  => {stack[pointer-2] = stack[pointer-2].wrapping_mul(&stack[pointer-1]);          pointer -= 1;}
                OpPivot(DIV)  => {stack[pointer-2] = stack[pointer-2] .checked_div(&stack[pointer-1])?;         pointer -= 1;}
                OpPivot(MOD)  => {stack[pointer-2] = stack[pointer-2] .checked_rem(&stack[pointer-1])?;         pointer -= 1;}
                OpPivot(ADD)  => {stack[pointer-2] = stack[pointer-2].wrapping_add(&stack[pointer-1]);          pointer -= 1;}
                OpPivot(SUB)  => {stack[pointer-2] = stack[pointer-2].wrapping_sub(&stack[pointer-1]);          pointer -= 1;}
                OpPivot(LSL)  => {stack[pointer-2] = stack[pointer-2].wrapping_shl( stack[pointer-1].as_u32()); pointer -= 1;}
                OpPivot(LSR)  => {stack[pointer-2] = stack[pointer-2].wrapping_shr( stack[pointer-1].as_u32()); pointer -= 1;}
                OpPivot(AND)  => {stack[pointer-2] = stack[pointer-2]             & stack[pointer-1];           pointer -= 1;}
                OpPivot(XOR)  => {stack[pointer-2] = stack[pointer-2]             ^ stack[pointer-1];           pointer -= 1;}
                OpPivot(ORR)  => {stack[pointer-2] = stack[pointer-2]             | stack[pointer-1];           pointer -= 1;}
                VarPivot(v)   => {stack[pointer  ] = inputs[v as usize];                                        pointer += 1;}
                FirstDigit(d) => {stack[pointer  ] = N::from_u8(d);                                             pointer += 1;}
                ContinuationDigit(d) => {stack[pointer-1] = stack[pointer-1].wrapping_shl(6) | N::from_u8(d);}
            }
        }

        Some(stack[0])
    }
}

