
use crate::search::writer::Partition;
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct AndWriter<N: Number> {
    partition: Partition,
    children: Children<N>,
}

impl<N: Number> AndWriter<N> {
    pub fn new(length: usize) -> Self {
        let mut initial_partition = Partition::standard(length);
        initial_partition.next();

        Self {
            children: Children::standard(CHILD_OF_AND, AND, &initial_partition.state()),
            partition: initial_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            if self.children.write(dest) {
                return true;
            }

            // todo: re-instate quick-exit conditions for partitions.

            if self.partition.next() {
                self.children = Children::standard(CHILD_OF_AND, AND, &self.partition.state());
                continue;
            }

            return false;
        }
    }
}

