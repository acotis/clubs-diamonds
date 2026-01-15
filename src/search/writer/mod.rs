
mod partition;
mod children;
mod writers;

pub use partition::Partition; // temporary. todo: make not public
pub use children::Children;
pub use writers::*;

use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op;

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

#[derive(Debug)]
pub struct WriterContext {
    pub location: Location,
    pub const_allowed: bool,
}

#[derive(Debug)]
enum WriterState {
    Init,
    Or(OrWriter),
    Shift(ShiftWriter),
    Add(AddWriter),
    Mul(MulWriter),
    Neg(NegWriter),
    Const(ConstWriter),
    Var(VarWriter),
    Done,
}

#[derive(Debug)]
pub struct Writer {
    length: usize,
    state: WriterState,
    pub context: WriterContext,
}

impl Writer {
    pub fn new(length: usize, context: WriterContext) -> Self {
        Self {
            length,
            state: Init,
            context,
        }
    }

    pub fn reset(&mut self) {
        self.state = Init;
    }

    pub fn is_const(&self) -> bool {
        matches!(self.state, Const(_))
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        //println!("state is {:?}", self.state);

        loop {
            match self.state {
                Init => {
                    self.init_or_state(dest);
                    continue;
                }

                Or(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_shift_state(dest);
                    continue;
                }

                Shift(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_add_state(dest);
                    continue;
                }

                Add(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_mul_state(dest);
                    continue;
                }

                Mul(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_neg_state(dest);
                    continue;
                }

                Neg(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_const_state(dest);
                    continue;
                }

                Const(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_var_state(dest);
                    continue;
                }

                Var(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_done_state(dest);
                    continue;
                }

                Done => {
                    return false;
                }
            }
        }
    }

    fn init_or_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_OR {2} else {0};
        if self.length < wasted_space + 3 {self.init_shift_state(dest); return;}
        if self.context.location == CHILD_OF_OR {self.init_shift_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = Or(OrWriter::new(self.length - wasted_space));
    }

    fn init_shift_state(&mut self, dest: &mut [u8]) {
        let wasted_space = 1 + if self.context.location > LEFT_CHILD_OF_SHIFT {2} else {0};
        if self.length < wasted_space + 3 {self.init_add_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = Shift(ShiftWriter::new(self.length - wasted_space + 1));
    }

    fn init_add_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > CHILD_OF_ADD {2} else {0};
        if self.length < wasted_space + 3 {self.init_mul_state(dest); return;}
        if self.context.location == CHILD_OF_ADD {self.init_mul_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = Add(AddWriter::new(self.length - wasted_space));
    }

    fn init_mul_state(&mut self, dest: &mut [u8]) {
        let wasted_space = if self.context.location > LEFT_CHILD_OF_MUL {2} else {0};
        if self.length < wasted_space + 3 {self.init_neg_state(dest); return;}

        dest[self.length-1] = Nop.encode(); // in case there are parens
        dest[self.length-2] = Nop.encode();

        self.state = Mul(MulWriter::new(self.length - wasted_space));
    }

    fn init_neg_state(&mut self, dest: &mut [u8]) {
        if self.length < 2 {self.init_const_state(dest); return;}

        self.state = Neg(NegWriter::new(self.length));
    }

    fn init_const_state(&mut self, dest: &mut [u8]) {
        if !self.context.const_allowed {self.init_var_state(dest); return;}
        if self.length > 2 {self.init_var_state(dest); return;}
        self.state = Const(ConstWriter::new(self.length));
    }

    fn init_var_state(&mut self, dest: &mut [u8]) {
        if self.length > 1 {self.init_done_state(dest); return;}
        self.state = Var(VarWriter::new(self.length));
    }

    fn init_done_state(&mut self, dest: &mut [u8]) {
        self.state = Done;
    }
}

