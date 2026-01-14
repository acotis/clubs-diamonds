
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

static OR:  u8 = OpPivot(Op::ORR).encode();
static XOR: u8 = OpPivot(Op::XOR).encode();
static AND: u8 = OpPivot(Op::AND).encode();
static LSL: u8 = OpPivot(Op::LSL).encode();
static LSR: u8 = OpPivot(Op::LSR).encode();
static ADD: u8 = OpPivot(Op::ADD).encode();
static SUB: u8 = OpPivot(Op::SUB).encode();
static MUL: u8 = OpPivot(Op::MUL).encode();
static DIV: u8 = OpPivot(Op::DIV).encode();
static MOD: u8 = OpPivot(Op::MOD).encode();
static NEG: u8 = OpPivot(Op::NEG).encode();
static NOT: u8 = OpPivot(Op::NOT).encode();

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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

pub struct WriterContext {
    pub location: Location,
}

enum WriterState {
    Init,
    Or(OrWriter),
    Add(AddWriter),
    Const(ConstWriter),
    Var(VarWriter),
    Done,
}

pub struct Writer {
    length: usize,
    state: WriterState,
    context: WriterContext,
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

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            match self.state {
                Init => {
                    self.init_or_state();
                    continue;
                }

                Or(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_add_state();
                    continue;
                }

                Add(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_const_state();
                    continue;
                }

                Const(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_var_state();
                    continue;
                }

                Var(ref mut writer) => {
                    if writer.write(dest) {return true;}
                    self.init_done_state();
                    continue;
                }

                Done => {
                    return false;
                }
            }
        }
    }

    fn init_or_state(&mut self) {
        if self.length < 3 {self.init_const_state(); return;}
        if self.context.location == CHILD_OF_OR {self.init_add_state(); return;}
        self.state = Or(OrWriter::new(self.length));
    }

    fn init_add_state(&mut self) {
        if self.length < 3 {self.init_const_state(); return;}
        if self.context.location == CHILD_OF_ADD {self.init_const_state(); return;}
        self.state = Add(AddWriter::new(self.length));
    }

    fn init_const_state(&mut self) {
        if self.length > 2 {self.init_var_state(); return;}
        self.state = Const(ConstWriter::new(self.length));
    }

    fn init_var_state(&mut self) {
        if self.length > 1 {self.init_done_state(); return;}
        self.state = Var(VarWriter::new(self.length));
    }

    fn init_done_state(&mut self) {
        self.state = Done;
    }
}

