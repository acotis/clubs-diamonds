
mod partition;
mod children;
mod writers;

pub use partition::Partition; // temporary. todo: make not public
pub use children::Children;
pub use writers::*;

use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op;

static ADD: u8 = OpPivot(Op::ADD).encode();
static SUB: u8 = OpPivot(Op::SUB).encode();
static OR:  u8 = OpPivot(Op::ORR).encode();

