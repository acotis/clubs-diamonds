
mod partition;
mod children;
mod writers;

pub use children::Children;
pub use writers::*;

use std::marker::PhantomData;

use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op;
use crate::Number;

use WriterState::*;
use Location::*;

const OR:  u8 = OpPivot(Op::ORR).encode();
const XOR: u8 = OpPivot(Op::XOR).encode();
const AND: u8 = OpPivot(Op::AND).encode();
const LSL: u8 = OpPivot(Op::LSL).encode();
const LSR: u8 = OpPivot(Op::LSR).encode();
const ADD: u8 = OpPivot(Op::ADD).encode();
const SUB: u8 = OpPivot(Op::SUB).encode();
const MUL: u8 = OpPivot(Op::MUL).encode();
const DIV: u8 = OpPivot(Op::DIV).encode();
const MOD: u8 = OpPivot(Op::MOD).encode();
const NEG: u8 = OpPivot(Op::NEG).encode();
const NOT: u8 = OpPivot(Op::NOT).encode();

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum Location {
    TOP,
    CHILD_OF_OR,
    CHILD_OF_XOR,
    CHILD_OF_AND,
    LEFT_CHILD_OF_SHIFT,
    RIGHT_CHILD_OF_SHIFT,
    CHILD_OF_ADD,
    LEFT_CHILD_OF_MUL,
    RIGHT_CHILD_OF_MUL,
    CHILD_OF_NEG,
}

#[derive(Debug, Clone)]
pub struct WriterContext {
    pub location: Location,
    pub const_allowed: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WriterType {
    ConstVar, Neg, Mul, Add, Shift, And, Xor, Or 
}

impl WriterType {
    pub fn all() -> Vec<WriterType> {
        [
            WriterType::ConstVar,
            WriterType::Neg, 
            WriterType::Mul, 
            WriterType::Add, 
            WriterType::Shift, 
            WriterType::And, 
            WriterType::Xor, 
            WriterType::Or, 
        ].into()
    }
}

#[derive(Debug, Clone)]
enum WriterState<N: Number, const C: usize> {
    Init,
    Or(OrWriter<N, C>),
    Xor(XorWriter<N, C>),
    And(AndWriter<N, C>),
    Shift(ShiftWriter<N, C>),
    Add(AddWriter<N, C>),
    Mul(MulWriter<N, C>),
    Neg(NegWriter<N, C>),
    Const(ConstWriter),
    Var(VarWriter<C>),
    Done,
}

#[derive(Debug, Clone)]
pub struct Writer<N: Number, const C: usize> {
    length: usize,
    state: WriterState<N, C>,
    pub context: WriterContext,
    writer_type: Option<WriterType>,
    constant_cap: u128,
    nothing: PhantomData<N>,
}

impl<N: Number, const C: usize> Writer<N, C> {
    pub fn new(length: usize, context: WriterContext, writer_type: Option<WriterType>, constant_cap: u128) -> Self {
        Self {
            length,
            state: Init,
            context,
            writer_type,
            constant_cap,
            nothing: PhantomData,
        }
    }

    pub fn reset(&mut self) {
        self.state = Init;
    }

    pub fn is_const(&self) -> bool {
        matches!(self.state, Const(_))
    }

    pub fn check_const_state(&mut self, dest: &mut [u8]) -> bool {
        if self.is_const() && !self.context.const_allowed {
            self.init_var_state(dest);
            true
        } else {
            false
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        //println!("state is {:?}", self.state);

        loop {
            match self.state {
                Init => {
                    match self.writer_type {
                        None                        => {self.init_or_state(dest)}
                        Some(WriterType::Or)        => {self.init_or_state(dest);    if !matches!(self.state, WriterState::Or(_))    {return false}}
                        Some(WriterType::Xor)       => {self.init_xor_state(dest);   if !matches!(self.state, WriterState::Xor(_))   {return false}}
                        Some(WriterType::And)       => {self.init_and_state(dest);   if !matches!(self.state, WriterState::And(_))   {return false}}
                        Some(WriterType::Shift)     => {self.init_shift_state(dest); if !matches!(self.state, WriterState::Shift(_)) {return false}}
                        Some(WriterType::Add)       => {self.init_add_state(dest);   if !matches!(self.state, WriterState::Add(_))   {return false}}
                        Some(WriterType::Mul)       => {self.init_mul_state(dest);   if !matches!(self.state, WriterState::Mul(_))   {return false}}
                        Some(WriterType::Neg)       => {self.init_neg_state(dest);   if !matches!(self.state, WriterState::Neg(_))   {return false}}
                        Some(WriterType::ConstVar)  => {self.init_const_state(dest)}
                    }
                    continue;
                }

                WriterState::Or(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_xor_state(dest);
                    continue;
                }

                WriterState::Xor(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_and_state(dest);
                    continue;
                }

                WriterState::And(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_shift_state(dest);
                    continue;
                }

                WriterState::Shift(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_add_state(dest);
                    continue;
                }

                WriterState::Add(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_mul_state(dest);
                    continue;
                }

                WriterState::Mul(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_neg_state(dest);
                    continue;
                }

                WriterState::Neg(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_const_state(dest);
                    continue;
                }

                WriterState::Const(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_var_state(dest);
                    continue;
                }

                WriterState::Var(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    if self.writer_type != None {return false;}
                    self.init_done_state(dest);
                    continue;
                }

                WriterState::Done => {
                    return false;
                }
            }
        }
    }

    fn init_or_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_OR {2} else {0};
        if self.length < wasted_space + 3 {self.init_xor_state(dest); return;}
        if self.context.location == CHILD_OF_OR {self.init_xor_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::Or(OrWriter::new(self.length - wasted_space, self.constant_cap));
    }

    fn init_xor_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_XOR {2} else {0};
        if self.length < wasted_space + 3 {self.init_and_state(dest); return;}
        if self.context.location == CHILD_OF_XOR {self.init_and_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::Xor(XorWriter::new(self.length - wasted_space, self.constant_cap));
    }

    fn init_and_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_AND {2} else {0};
        if self.length < wasted_space + 3 {self.init_shift_state(dest); return;}
        if self.context.location == CHILD_OF_AND {self.init_shift_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::And(AndWriter::new(self.length - wasted_space, self.constant_cap));
    }

    fn init_shift_state(&mut self, dest: &mut [u8]) {
        let wasted_space = 1 + if self.context.location > LEFT_CHILD_OF_SHIFT {2} else {0};
        if self.length < wasted_space + 3 {self.init_add_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::Shift(ShiftWriter::new(self.length - wasted_space + 1, self.constant_cap));
    }

    fn init_add_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_ADD {2} else {0};
        if self.length < wasted_space + 3 {self.init_mul_state(dest); return;}
        if self.context.location == CHILD_OF_ADD {self.init_mul_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::Add(AddWriter::new(self.length - wasted_space, self.constant_cap));
    }

    fn init_mul_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > LEFT_CHILD_OF_MUL {2} else {0};
        if self.length < wasted_space + 3 {self.init_neg_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = WriterState::Mul(MulWriter::new(self.length - wasted_space, self.constant_cap));
    }

    fn init_neg_state(&mut self, dest: &mut [u8]) {
        if self.length < 2 {self.init_const_state(dest); return;}

        self.state = WriterState::Neg(NegWriter::new(self.length, self.constant_cap));
    }

    fn init_const_state(&mut self, dest: &mut [u8]) {
        if !self.context.const_allowed {self.init_var_state(dest); return;}
        if self.length > 4 {self.init_var_state(dest); return;}
        self.state = WriterState::Const(ConstWriter::new(self.length, self.constant_cap));
    }

    fn init_var_state(&mut self, dest: &mut [u8]) {
        if self.length > 1 {self.init_done_state(dest); return;}
        self.state = WriterState::Var(VarWriter::new(self.length, self.constant_cap));
    }

    fn init_done_state(&mut self, _dest: &mut [u8]) {
        self.state = Done;
    }
}

