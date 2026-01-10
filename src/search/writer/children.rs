
// Now let's factor out a struct that manages an array of children of fixed
// lengths (every time the lengths change, an fresh Children instance is
// created to manage the new set of children).

pub struct Children {
    children: Vec<(usize, XorWriter)>, // just XorWriter for now
}

impl Children {
    pub fn new_from_sizes(sizes: &[usize]) -> Self {
        let mut ret = Self {
            children: vec![]
        };

        let mut offset = 0;

        for size in sizes {
            ret.children.push((offset, XorWriter::new(*size)));
            offset += size + 1;
        }

        ret
    }

    // Todo: account for the fact that even a Writer's first write can return
    // false (that is, it is already exhausted when it gets created because
    // there are no valid things it can write).

    pub fn do_first_write(&mut self, dest: &mut [u8]) {
        for (offset, child) in &mut self.children {
            if *offset > 0 {dest[*offset-1] = b'|';}
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

struct XorWriter {
    length: usize,
    already_wrote: bool,
}

impl XorWriter {
    fn new(length: usize) -> Self {
        Self {
            length,
            already_wrote: false,
        }
    }

    fn write(&mut self, field: &mut [u8]) -> bool {
        if !self.already_wrote {
            for i in 0..self.length {
                field[i] = b'x';
            }

            self.already_wrote = true;
            return true;
        }

        false
    }
}

