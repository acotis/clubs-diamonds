
use crate::Number;
use super::{Writer, WriterContext, Location};

// The Children struct manages a set of children expressions of given fixed
// lengths. It can be configured to either forbid or allow multiple children
// to take constant values; it should be configured to forbid this for
// "liquifying" operators (ones which are commutative and associative and
// also which, when applied to any two constants, yield a value that would
// be shorter-or-equal to just write out literally, i.e. 7+9 = 16 and "16"
// is shorter than "7+9").
//
// The Children struct comes in a few variations to support different
// levels' use-cases. Here is the full table:
//
//     |        standard
//     ^        standard
//     &        standard
//     << >>    two-context
//     +-       dual
//     */%      two-context
//     -!       (not implemented yet)
//     const    (not implemented yet)

#[derive(Debug, Clone)]
pub struct Children<N: Number, const C: usize> {
    children: Vec<(usize, Writer<N, C>)>, // just FillerWriter for now
    children_in_group_1: usize,
    op_byte_1: u8,
    op_byte_2: u8,
    first_write: bool,
    forbid_multi_constants: bool,
    commutative: bool,
}

impl<N: Number, const C: usize> Children<N, C> {
    pub fn standard(location: Location, op_byte: u8, sizes: &[usize]) -> Self {
        Self::new(location, location, op_byte, 0, sizes, &[])
    }

    pub fn two_context(location_head: Location, location_tail: Location, op_byte: u8, sizes: &[usize]) -> Self {
        Self::new(location_head, location_tail, op_byte, 0, sizes, &[])
    }

    pub fn dual(location: Location, op_byte_1: u8, sizes_1: &[usize], op_byte_2: u8, sizes_2: &[usize]) -> Self {
        Self::new(location, location, op_byte_1, op_byte_2, sizes_1, sizes_2)
    }

    pub fn allow_multi_constants(self) -> Self {
        Self {
            forbid_multi_constants: false,
            ..self
        }
    }

    pub fn non_commutative(self) -> Self {
        Self {
            commutative: false,
            ..self
        }
    }

    // A generic constructor method which is invoked with various parameters
    // by the public constructors, which each target a specific use-case.

    fn new(
        location_head: Location,    // Location for first child.
        location_tail: Location,    // Location for all other children.
        op_byte_1: u8,              // Op head to write for first segment of children.
        op_byte_2: u8,              // Op head to write for second segment of children.
        sizes_1: &[usize],          // Sizes of children in first segment.
        sizes_2: &[usize],          // Sizes of children in second segment.
    ) -> Self {
        let mut ret = Self {
            children: vec![],
            children_in_group_1: sizes_1.len(),
            op_byte_1,
            op_byte_2,
            first_write: true,
            forbid_multi_constants: true,
            //commutative: true,
            commutative: false, // always non-commutative so I don't have to worry about it for now
        };

        let mut offset = 0;

        for &size in sizes_1.iter().chain(sizes_2.iter()) {
            ret.children.push((offset, Writer::new(size, WriterContext {location: if offset == 0 {location_head} else {location_tail}, const_allowed: true}, None)));
            offset += if offset == 0 {size} else {size + 1};
        }

        ret
    }

    // The .write() method.

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        let ret = self.write_helper(dest, self.children.len()-1, self.first_write);
        self.first_write = false;
        ret
    }

    fn write_helper(&mut self, dest: &mut [u8], index: usize, mut first_write: bool) -> bool {
        loop {
            let offset = self.children[index].0;
            let skip = first_write && index != 0;

            if !skip && self.children[index].1.write(&mut dest[offset..]) {
                if index > 0 {
                    dest[self.children[index].0 + self.children[index].1.length] = if index < self.children_in_group_1 {
                        self.op_byte_1
                    } else {
                        self.op_byte_2
                    };
                }

                return true;
            }

            //println!("{indent}writing this child was disallowed or failed (skip = {skip})");

            if index == 0 {
                return false;
            }

            if self.write_helper(dest, index-1, first_write) {
                //println!("{indent}recursion succeeded...");

                self.children[index].1.context.const_allowed =
                    !self.forbid_multi_constants ||
                    self.children[index-1].1.context.const_allowed &&
                   !self.children[index-1].1.is_const();

                // commutativity is disabled for now

                if self.commutative 
                && self.children[index].1.length == self.children[index-1].1.length
                && index != self.children_in_group_1
                {
                    self.children[index].1.state = self.children[index-1].1.state.clone();

                    if self.children[index].1.check_const_state(dest) {
                        first_write = false;
                        continue;
                    } else {
                        for i in 0..self.children[index].1.length {
                            dest[self.children[index].0 + i] = dest[self.children[index-1].0 + i];
                        }
                        return true;
                    }
                } else {
                    self.children[index].1.reset();
                    first_write = false;
                    continue;
                }
            }

            return false;
        }
    }
}

