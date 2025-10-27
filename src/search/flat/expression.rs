
use std::marker::PhantomData;

use crate::search::number::Number;
use crate::search::flat::pivot::Pivot::*;
use crate::search::flat::pivot::Op::{self, *};

#[derive(Copy, Clone)]
pub struct ExpressionCore<'a, N: Number, const C: usize> {
    field: &'a [u8],
    nothing: std::marker::PhantomData<N>,
}

impl<'a, N: Number, const C: usize> ExpressionCore<'a, N, C> {
    pub fn new (field: &'a [u8]) -> Self {
        Self {
            field,
            nothing: PhantomData::default(),
        }
    }

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
}

pub struct Expression<N: Number, const C: usize> {
    field_for_core: Vec<u8>,
    nothing: std::marker::PhantomData<N>,
}

impl<N: Number, const C: usize> Expression<N, C> {
    pub fn from_core(core: ExpressionCore<'_, N, C>) -> Self {
        Self {
            field_for_core: core.field.to_vec(),
            nothing: PhantomData::default(),
        }
    }

    pub fn apply(&self, inputs: &[N; C]) -> Option<N> {
        self.core().apply(inputs)
    }

    pub fn core(&self) -> ExpressionCore<N, C> {
        ExpressionCore {
            field: &self.field_for_core,
            nothing: PhantomData::default(),
        }
    }

    fn stringify(&self, start: usize) -> (String, usize, usize) {
        if start >= self.field_for_core.len() {
            for i in 0..self.field_for_core.len() {
                print!("{:?} ", crate::search::flat::pivot::Op::interpret_code(self.field_for_core[i]));
            }
            println!();
        }

        match Op::interpret_code(self.field_for_core[start]) {
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

    pub fn count_variable_appearances(&self, variable_id: u8) -> usize {
        self.field_for_core.iter().map(|&i| if Op::interpret_code(i) == VarPivot(variable_id) {1} else {0}).sum()
    }
}

impl<N: Number, const C: usize> std::fmt::Display for Expression<N, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.stringify(0).0)
    }
}

