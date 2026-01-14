
use crate::search::pivot::Pivot::*;
use super::{Writer, WriterContext, Location};

// Now let's factor out a struct that manages an array of children of fixed
// lengths (every time the lengths change, an fresh CalSet instance is
// created to manage the new set of children).

// The CalSet struct manages the children of any operator which is
// Commutative, Associative, and Liquifying (merges constants). It can
// also handle the addition/subtraction layer (a pair of operators which
// honorarily have these three properties when considered in tandem)
// via its CalSet::dual() method.

pub struct CalSet {
    children: Vec<(usize, Writer)>, // just FillerWriter for now
    children_in_group_1: usize,
    op_byte_1: u8,
    op_byte_2: u8,
    first_write: bool,
}

impl CalSet {
    pub fn standard(location: Location, op_byte: u8, sizes: &[usize]) -> Self {
        Self::dual(location, op_byte, sizes, 0, &[])
    }

    pub fn dual(location: Location, op_byte_1: u8, sizes_1: &[usize], op_byte_2: u8, sizes_2: &[usize]) -> Self {
        let mut ret = Self {
            children: vec![],
            children_in_group_1: sizes_1.len(),
            op_byte_1,
            op_byte_2,
            first_write: true,
        };

        let mut offset = 0;

        for &size in sizes_1.iter().chain(sizes_2.iter()) {
            ret.children.push((offset, Writer::new(size, WriterContext {location})));
            offset += if offset == 0 {size} else {size + 1};
        }

        ret
    }

    // The .write() method 

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.first_write {
            self.first_write = false;
            self.first_write(dest)
        } else {
            self.write_helper(dest, self.children.len()-1)
        }
    }

    fn first_write(&mut self, dest: &mut [u8]) -> bool {
        for index in 0..self.children.len() {
            let (offset, child) = &mut self.children[index];

            if !child.write(&mut dest[*offset..]) {
                return false;
            }

            if index > 0 {
                dest[*offset + child.length] = if index < self.children_in_group_1 {
                    self.op_byte_1
                } else {
                    self.op_byte_2
                };
            }
        }

        true
    }

    fn write_helper(&mut self, dest: &mut [u8], index: usize) -> bool {
        let (offset, child) = &mut self.children[index];

        //println!("writing child at offset {offset} with length {}", child.length);

        if child.write(&mut dest[*offset..]) {
            return true;
        }

        if index == 0 {
            return false;
        }

        if self.write_helper(dest, index-1) {
            self.children[index].1.reset();
            return self.write_helper(dest, index);
        }

        return false;
    }
}

struct FillerWriter {
    length: usize,
    next_num: u8,
}

impl FillerWriter {
    fn new(length: usize) -> Self {
        Self {
            length,
            next_num: 1,
        }
    }

    fn write(&mut self, field: &mut [u8]) -> bool {
        if self.next_num > 3 {return false}
        field[self.length-1] = Filler(self.next_num, self.length as u8).encode();
        self.next_num += 1;
        true
    }

    fn reset(&mut self) {
        self.next_num = 1;
    }
}

