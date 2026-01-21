
use crate::Expression;

use crate::search::pivot::Pivot;
use crate::search::number::Number;
use crate::search::pivot::Pivot::*;

impl<N: Number, const C: usize> Expression<N, C> {
    fn stringify(&self, start: usize) -> (String, usize, usize) {
        if start >= self.field.len() {
            for i in 0..self.field.len() {
                print!("{:?} ", Pivot::decode(self.field[i]));
            }
            println!();
        }

        match Pivot::decode(self.field[start]) {
            Nop           => {let (a, b, c) = self.stringify(start-1); (a, b, c+1)},
            Filler(c, l)  => (["_", "x", "█"][c as usize - 1].repeat(l as usize), !0, l as usize),
            ConstPivot(p) => (format!("{p}"), !0, 1),
            VarPivot(v)   => (format!("{}", (v + b'a') as char), !0, 1),
            OpPivot(op)   => {
                if op.arity() == 1 {
                    let (right, right_prec, right_len) = self.stringify(start - 1);
                    let right_render = if right_prec >= op.prec() {right} else {format!("({right})")};

                    (format!("{}{}", op.render_face(), right_render), op.prec(), 1 + right_len)
                } else {
                    let (right, right_prec, right_len) = self.stringify(start - 1);
                    let (left,  left_prec,  left_len ) = self.stringify(start - 1 - right_len);

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
        write!(f, "{}", self.stringify(self.field.len()-1).0)
    }
}

