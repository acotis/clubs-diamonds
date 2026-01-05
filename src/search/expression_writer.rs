
use crate::search::pivot::Op;
use self::EWState::*;

// Note to future self: because the final array representation of the expression
// is read backwards, the "left" subexpression of a binary operator appears to
// the right of the "left" subexpression in that array; the expression "a/2"
// would be represented as [DIV 2 a].

enum EWState {
    Dummy,
    Init,
    Variable  {next: u8, max: u8},
    Constant  {next: u8, max: u8},
    PrepareOp {op: Op},
    OpState   {op: Op, left: Box<ExpressionWriter>, right: Box<ExpressionWriter>},
}

pub struct ExpressionWriter {
    input_count: u8,
    required_vars: usize,
    length: usize,
    min_prec: usize,
    constant_cap: u8,
    op_requirement: Option<Option<Op>>,

    state: EWState,
    vum_of_last_write: usize,
}

impl ExpressionWriter {
    pub fn new(input_count: usize, length: usize, constant_cap: u8, op_requirement: Option<Option<Op>>) -> Self {
        Self {
            input_count: input_count as u8,
            required_vars: (1 << input_count) - 1,
            //required_vars: 0,
            length,
            min_prec: 0,
            constant_cap,
            op_requirement,
            state: Init,
            vum_of_last_write: 0,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        match self.state {
            Dummy => {
                return false;
            },

            Init => {
                if self.length + 1 < self.required_vars.count_ones() as usize * 2 {
                    return false;
                }

                self.state = if let Some(req) = self.op_requirement {
                    if let Some(op) = req {
                        match self.length {
                            0 => panic!(),
                            1 => return false,
                            2.. => PrepareOp {op},
                        }
                    } else {
                        match self.length {
                            0 => panic!(),
                            1 => if self.required_vars == 0 {
                                Variable {next: 0, max: self.input_count-1}
                            } else {
                                Variable {next: self.required_vars.trailing_zeros() as u8, max: self.required_vars.trailing_zeros() as u8}
                            },
                            2 => if self.required_vars == 0 {
                                Constant {next: 10, max: 100.min(self.constant_cap)}
                            } else {
                                return false
                            },
                            3 => if self.required_vars == 0 {
                                Constant {next: 100, max: 156.min(self.constant_cap)}
                            } else {
                                return false
                            },
                            4.. => return false,
                            //3.. => return false,
                        }
                    }
                } else {
                    match self.length {
                        0 => panic!(),
                        1 => if self.required_vars == 0 {
                            Variable {next: 0, max: self.input_count-1}
                        } else {
                            Variable {next: self.required_vars.trailing_zeros() as u8, max: self.required_vars.trailing_zeros() as u8}
                        },
                        2 => if self.required_vars == 0 {
                            Constant {next: 10, max: 100.min(self.constant_cap)}
                        } else {
                            PrepareOp {op: Op::first()}
                        },
                        3 => if self.required_vars == 0 {
                            Constant {next: 100, max: 156.min(self.constant_cap)}
                        } else {
                            PrepareOp {op: Op::first()}
                        },
                        4.. => PrepareOp {op: Op::first()},
                        //3.. => PrepareOp {op: Op::first()},
                    }
                };

                dest.fill(255);
                return self.write(dest);
            },

            Variable {next, max} => {
                if next <= max {
                    self.state = Variable {next: next + 1, max};
                    dest[self.length-1] = Op::highest_unused_code() - next;
                    self.vum_of_last_write = 1 << next;
                    return true;
                }

                // If we are looping through variables, then the
                // length is 1, so we know where we're going next
                // based only on the required_vars thing.

                if self.required_vars > 0 {
                    return false;
                }

                self.state = Constant {next: 0, max: 10.min(self.constant_cap)};
                return self.write(dest);
            },

            Constant {next, max} => {
                if next < max {
                    self.state = Constant {next: next + 1, max};
                    dest[self.length-1] = next;
                    self.vum_of_last_write = 0;
                    return true;
                }

                if self.op_requirement.is_some() { // must have been Some(None)
                    return false;
                }

                self.state = PrepareOp {op: Op::first()};
                return self.write(dest);
            }

            PrepareOp {op} => {
                let wasted_space = 
                    op.len() + 
                    if op.prec() < self.min_prec {2} else {0};

                // If this op wastes too much space, move on to
                // the next one.

                if wasted_space + op.arity() > self.length {
                    if let Some(next) = op.next() {
                        if self.op_requirement.is_some() {return false;}
                        self.state = PrepareOp {op: next};
                        return self.write(dest);
                    } else {
                        return false;
                    }
                }

                // Else, set it up.

                dest.fill(255);
                dest[self.length-1] = op.code();

                if op.arity() == 1 {
                    self.state = OpState {
                        op,
                        left: Box::new(Self {
                            input_count: 0,
                            length: 0,
                            min_prec: 0,
                            constant_cap: self.constant_cap,
                            required_vars: 0,
                            op_requirement: None,
                            state: Dummy,
                            vum_of_last_write: 0,
                        }),
                        right: Box::new(Self {
                            input_count: self.input_count,
                            length: self.length - wasted_space,
                            min_prec: op.prec(),
                            constant_cap: self.constant_cap,
                            required_vars: self.required_vars,
                            op_requirement: None,
                            state: Init,
                            vum_of_last_write: 0,
                        }),
                    };
                } else {
                    self.state = OpState {
                        op,
                        left: Box::new(Self {
                            input_count: self.input_count,
                            length: 1,
                            min_prec: op.prec(),
                            constant_cap: self.constant_cap,
                            required_vars: 0,
                            op_requirement: None,
                            state: Init,
                            vum_of_last_write: 0,
                        }),
                        right: Box::new(Self {
                            input_count: self.input_count,
                            length: self.length - wasted_space - 1,
                            min_prec: op.prec() + 1,
                            constant_cap: self.constant_cap,
                            required_vars: 0, // filled in later
                            op_requirement: None,
                            state: Init,
                            vum_of_last_write: 0,
                        }),
                    };
                }

                return self.write(dest);
            },
            
            OpState {op, ref mut left, ref mut right} => {
                if matches!(left.state, Init) {
                    if !left.write(&mut dest[..self.length-1-right.length]) {
                        if let Some(next) = op.next() {
                            if self.op_requirement.is_some() {return false;}
                            self.state = PrepareOp {op: next};
                            return self.write(dest);
                        } else {
                            return false;
                        }
                    }

                    right.required_vars = self.required_vars & !left.vum_of_last_write;
                }

                if right.write(&mut dest[self.length-1-right.length..self.length-1]) {
                    self.vum_of_last_write = left.vum_of_last_write | right.vum_of_last_write;
                    return true;
                }

                if left.write(&mut dest[..self.length-1-right.length]) {
                    right.state = Init;
                    right.required_vars = self.required_vars & !left.vum_of_last_write;
                    return self.write(dest);
                }

                if right.length > 1 && !matches!(left.state, Dummy) {
                    left.length  += 1; left.state  = Init;
                    right.length -= 1; right.state = Init;
                    return self.write(dest);
                }

                if let Some(next) = op.next() {
                    if self.op_requirement.is_some() {return false;}
                    self.state = PrepareOp {op: next};
                    return self.write(dest);
                }

                return false;
            },
        }
    }
}

