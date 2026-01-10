
use std::marker::PhantomData;

use crate::search::pivot::Pivot;
use crate::search::number::Number;
use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op::*;

/// Represents a syntactically-valid mathematical Rust expression. Can be applied to a set of input values to yield a result value. Can also be rendered as text using the `format!` macro or `.to_string()` method.
///
/// Currently, the only way to get your hands on an `Expression` is to be given it by a `Searcher`.

// Non-doc comment for devs: here is a list of common traits and why Expression
// doesn't implement them:
//     — Copy: uses a Vec internally
//     — PartialEq + Eq: I'd want this to be semantic and the default impl
//       would by symbolic against the vec's contents
//     — PartialOrd + Ord: meaningless
//     — Hash: same basic reason as PartialEq + Eq
//     — Default: no sensible default
//     — Serialize + Deserialize: would expose implementation details that I'm
//       not ready to stabilize

#[derive(Clone, Debug)] // impls Display below
pub struct Expression<N: Number, const C: usize> {
    pub /*(super)*/ field: Vec<u8>,
    pub /*(super)*/ nothing: PhantomData<N>,
    // todo: make these not public to the world anymore
}

impl<N: Number, const C: usize> Expression<N, C> {
    /// Apply this expression to an array of input values. Returns a result wrapped in an `Option`. The value `None` is returned if applying the expression to the given values would result in a runtime exception (for example, if it would end up dividing by zero).
    ///
    /// The length and entry type of the array must match the type parameters of the Expression. If the expression came from a `Searcher::<u32, 3>`, then it is an `Expression::<u32, 3>` and the argument to this function is a `[u32; 3]`.
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
                Nop           => {},
                OpPivot(NEG)  => {stack[pointer-1] = N::from_u8(0) - stack[pointer-1]}
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
                ConstPivot(p) => {stack[pointer  ] = N::from_u8(p);                                             pointer += 1;}
                VarPivot(v)   => {stack[pointer  ] = inputs[v as usize];                                        pointer += 1;}
            }
        }

        Some(stack[0])
    }

    /// Render this expression as text. Same as calling `format!("{expr}")`.

    pub fn render(&self) -> String {
        self.stringify(self.field.len()-1, &['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ]).0
    }

    /// Render this expression as text, using the provided array of characters as the variable names.

    pub fn render_with_var_names(&self, var_names: [char; C]) -> String {
        self.stringify(self.field.len()-1, &var_names).0
    }
}

impl<N: Number, const C: usize> Expression<N, C> {
    fn stringify(&self, start: usize, var_names: &[char]) -> (String, usize, usize) {
        if start >= self.field.len() {
            for i in 0..self.field.len() {
                print!("{:?} ", Pivot::decode(self.field[i]));
            }
            println!();
        }

        match Pivot::decode(self.field[start]) {
            Nop           => {let (a, b, c) = self.stringify(start-1, var_names); (a, b, c+1)},
            ConstPivot(p) => (format!("{p}"),                  !0, 1),
            VarPivot(v)   => (format!("{}", var_names[v as usize]), !0, 1),
            OpPivot(op)   => {
                if op.arity() == 1 {
                    let (right, right_prec, right_len) = self.stringify(start - 1, var_names);
                    let right_render = if right_prec >= op.prec() {right} else {format!("({right})")};

                    (format!("{}{}", op.render_face(), right_render), op.prec(), 1 + right_len)
                } else {
                    let (right, right_prec, right_len) = self.stringify(start - 1, var_names);
                    let (left,  left_prec,  left_len ) = self.stringify(start - 1 - right_len, var_names);

                    let left_render  = if left_prec  >= op.prec() {left } else {format!("({left})")};
                    let right_render = if right_prec >  op.prec() {right} else {format!("({right})")};

                    (format!("{}{}{}", left_render, op.render_face(), right_render), op.prec(), 1 + left_len + right_len)
                }
            }
        }
    }

    pub(super) fn render_with_optional_var_names(&self, var_names_option: Option<[char; C]>) -> String {
        if let Some(var_names) = var_names_option {
            self.render_with_var_names(var_names)
        } else {
            self.render()
        }
    }
}

impl<N: Number, const C: usize> std::fmt::Display for Expression<N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

