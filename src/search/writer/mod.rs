
mod partition;
mod children;
mod writers;

pub use partition::Partition; // temporary. todo: make not public
pub use children::Children;
pub use writers::*;

use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op;

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

enum Location {
    TOP,
    OR,
    XOR,
    AND,
    LEFT_OF_SHIFT,
    RIGHT_OF_SHIFT,
    ADD,
    LEFT_OF_MUL,
    RIGHT_OF_MUL,
    NEG,
}

struct WriterContext {
    location: Location,
}

enum WriterState {
    Init,
    Or(OrWriter),
    Add(AddWriter),
    Done,
}

struct Writer {
    length: usize,
    state: WriterState,
    context: WriterContext,
}

impl Writer {
    fn write(&mut self, dest: &mut [u8]) {
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
                    self.state = Done;
                    continue;
                }

                Done => {
                    return false;
                }
            }
        }
    }
}

