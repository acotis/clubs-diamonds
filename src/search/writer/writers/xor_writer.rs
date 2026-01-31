
use crate::search::writer::partition::Partition;
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct XorWriter<N: Number, const C: usize> {
    constant_cap: u8,
    partition: Partition,
    children: Children<N, C>,
}

impl<N: Number, const C: usize> XorWriter<N, C> {
    pub fn new(length: usize, constant_cap: u8) -> Self {
        let mut initial_partition = Partition::standard(length);
        initial_partition.next();

        Self {
            constant_cap,
            children: Children::standard(CHILD_OF_XOR, constant_cap, XOR, &initial_partition.state()),
            partition: initial_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            if self.children.write(dest) {
                return true;
            }

            if self.partition.next() {
                self.children = Children::standard(CHILD_OF_XOR, self.constant_cap, XOR, &self.partition.state());
                continue;
            }

            return false;
        }
    }
}

