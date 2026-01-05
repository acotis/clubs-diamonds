
use self::Op::*;
use self::Pivot::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pivot {
    Nop,
    OpPivot(Op),
    ConstPivot(u8),
    VarPivot(u8),
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Op {
    NOT, NEG,
    MUL, DIV, MOD,
    ADD, SUB,
    LSL, LSR,
    AND,
    XOR,
    ORR,
}

impl Op {
    pub fn first() -> Op {
        NOT
    }

    pub fn next(self) -> Option<Op> {
        match self {
            NEG => Some(NOT),
            NOT => Some(MUL),
            MUL => Some(DIV),
            DIV => Some(MOD),
            MOD => Some(ADD),
            ADD => Some(SUB),
            SUB => Some(LSL),
            LSL => Some(LSR),
            LSR => Some(AND),
            AND => Some(XOR),
            XOR => Some(ORR),
            ORR => None,
        }
    }

    pub fn len(self) -> usize {
        match self {
            NEG => 1,
            NOT => 1,
            MUL => 1,
            DIV => 1,
            MOD => 1,
            ADD => 1,
            SUB => 1,
            LSL => 2,
            LSR => 2,
            AND => 1,
            XOR => 1,
            ORR => 1,
        }
    }

    pub fn arity(self) -> usize {
        match self {
            NEG => 1,
            NOT => 1,
            MUL => 2,
            DIV => 2,
            MOD => 2,
            ADD => 2,
            SUB => 2,
            LSL => 2,
            LSR => 2,
            AND => 2,
            XOR => 2,
            ORR => 2,
        }
    }

    pub fn prec(self) -> usize {
        match self {
            NEG => 6,
            NOT => 6,
            MUL => 5,
            DIV => 5,
            MOD => 5,
            ADD => 4,
            SUB => 4,
            LSL => 3,
            LSR => 3,
            AND => 2,
            XOR => 1,
            ORR => 0,
        }
    }

    pub fn code(self) -> u8 {
        match self {
            NEG => 243,
            NOT => 254,
            MUL => 253,
            DIV => 252,
            MOD => 251,
            ADD => 250,
            SUB => 249,
            LSL => 248,
            LSR => 247,
            AND => 246,
            XOR => 245,
            ORR => 244,
        }
    }

    pub fn render_face(self) -> &'static str {
        match self {
            NEG => "-",
            NOT => "!",
            MUL => "*",
            DIV => "/",
            MOD => "%",
            ADD => "+",
            SUB => "-",
            LSL => "<<",
            LSR => ">>",
            AND => "&",
            XOR => "^",
            ORR => "|",
        }
    }

    pub fn interpret_code(code: u8) -> Pivot {
        match code {
            255      => Nop,
            243      => OpPivot(NEG),
            254      => OpPivot(NOT),
            253      => OpPivot(MUL),
            252      => OpPivot(DIV),
            251      => OpPivot(MOD),
            250      => OpPivot(ADD),
            249      => OpPivot(SUB),
            248      => OpPivot(LSL),
            247      => OpPivot(LSR),
            246      => OpPivot(AND),
            245      => OpPivot(XOR),
            244      => OpPivot(ORR),
            ..=155   => ConstPivot(code),
            230..243 => VarPivot(242 - code),
            x   => panic!("Unrecognized op {x}"),
        }
    }

    pub fn highest_unused_code() -> u8 {
        242
    }
}

