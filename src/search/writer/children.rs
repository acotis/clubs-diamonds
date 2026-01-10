
use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op::*;

// Now let's factor out a struct that manages an array of children of fixed
// lengths (every time the lengths change, an fresh Children instance is
// created to manage the new set of children).

pub struct Children {
    children: Vec<(usize, FillerWriter)>, // just FillerWriter for now
    write_op_after_first_child: bool,
}

impl Children {
    fn new(sizes: &[usize], standard: bool) -> Self {
        let mut ret = Self {
            write_op_after_first_child: false,
            children: vec![]
        };

        let mut offset = 0;

        for size in sizes {
            ret.children.push((offset, FillerWriter::new(*size)));
            offset += size + if standard && offset == 0 {0} else {1};
        }

        ret
    }

    pub fn standard(sizes: &[usize]) -> Self {
        Self::new(sizes, true)
    }

    pub fn extender(sizes: &[usize]) -> Self {
        Self::new(sizes, false)
    }

    // Todo: account for the fact that even a Writer's first write can return
    // false (that is, it is already exhausted when it gets created because
    // there are no valid things it can write).

    pub fn do_first_write(&mut self, dest: &mut [u8]) {
        for (offset, child) in &mut self.children {
            dest[*offset + child.length] = OpPivot(ORR).encode(); // this gets overwritten in a standard Children and kept in an extender
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

        // The dummy filler is just "0*0*0..." (with one of the 0's being 90
        // instead if the total number of bytes to fill is even).

        for i in 0..self.length {
            field[i] = if i == 0 || i % 2 == 1 {
                ConstPivot(0).encode()
            } else {
                OpPivot(MUL).encode()
            };
        }

        if self.length % 2 == 0 {
            field[0] = ConstPivot(90).encode();
            field[self.length-1] = Nop.encode();
        }

        self.already_wrote = true;
        true
    }
}

