
use crate::search::writer::Partition;
use crate::search::writer::Children;
use super::super::*;

pub struct XorWriter {
    length: usize,

    partition: Partition,
    children: Children,
}

impl XorWriter {
    pub fn new(length: usize) -> Self {
        let initial_partition = Partition::standard(length);

        Self {
            length,
            children: Children::standard(XOR, &initial_partition.state()),
            partition: initial_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.children.write(dest) {return true}

        // todo: re-instate quick-exit conditions.

        if self.partition.next() {
            self.children = Children::standard(XOR, &self.partition.state());
            self.children.do_first_write(dest);
            return true;
        }

        false
    }
}

