
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

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        let mut next_to_write = self.children.len()-1;

        loop {
            let (offset, child) = &mut self.children[next_to_write];
            if child.write(&mut dest[*offset..]) {
                // todo: must also reset all children following this one
                return true;
            }
            if next_to_write == 0 {return false}
            next_to_write -= 1;
        }
    }
}

struct FillerWriter {
    length: usize,
    already_wrote: bool,
}

impl FillerWriter {
    fn new(length: usize) -> Self {
        Self {
            length,
            already_wrote: false,
        }
    }

    fn write(&mut self, field: &mut [u8]) -> bool {
        if self.already_wrote {return false}
        field[self.length-1] = Filler(self.length as u8).encode();
        self.already_wrote = true;
        true
    }
}

