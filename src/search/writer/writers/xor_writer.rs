
use crate::search::writer::partition::Partition;
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct XorWriter<N: Number, const C: usize> {
    max_constant: Option<u128>,
    partition: Partition,
    children: Children<N, C>,
}

impl<N: Number, const C: usize> XorWriter<N, C> {
    pub fn new(length: usize, max_constant: Option<u128>) -> Self {
        let mut initial_partition = Partition::standard(length);
        initial_partition.next();

        Self {
            max_constant,
            children: Children::standard(CHILD_OF_XOR, max_constant, XOR, &initial_partition.state()),
            partition: initial_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            if self.children.write(dest) {
                return true;
            }

            if self.partition.next() {
                self.children = Children::standard(CHILD_OF_XOR, self.max_constant, XOR, &self.partition.state());
                continue;
            }

            return false;
        }
    }
}

