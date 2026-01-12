
use crate::search::pivot::Pivot::*;

// Now let's factor out a struct that manages an array of children of fixed
// lengths (every time the lengths change, an fresh Children instance is
// created to manage the new set of children).

pub struct Children {
    children: Vec<(usize, FillerWriter)>, // just FillerWriter for now
    op_byte: u8,
}

impl Children {
    fn new(op_byte: u8, sizes: &[usize], standard: bool) -> Self {
        let mut ret = Self {
            op_byte,
            children: vec![]
        };

        let mut offset = 0;

        for size in sizes {
            ret.children.push((offset, FillerWriter::new(*size)));
            offset += size + if standard && offset == 0 {0} else {1};
        }

        ret
    }

    pub fn standard(op_byte: u8, sizes: &[usize]) -> Self {
        Self::new(op_byte, sizes, true)
    }

    pub fn extender(op_byte: u8, sizes: &[usize]) -> Self {
        Self::new(op_byte, sizes, false)
    }

    // Todo: account for the fact that even a Writer's first write can return
    // false (that is, it is already exhausted when it gets created because
    // there are no valid things it can write).

    pub fn do_first_write(&mut self, dest: &mut [u8]) {
        for (offset, child) in &mut self.children {
            dest[*offset + child.length] = self.op_byte; // this gets overwritten in a standard Children and kept in an extender
            child.write(&mut dest[*offset..]);
        }
    }

    // The .write() method 

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        self.write_helper(dest, self.children.len()-1)
    }

    fn write_helper(&mut self, dest: &mut [u8], index: usize) -> bool {
        let (offset, child) = &mut self.children[index];

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

