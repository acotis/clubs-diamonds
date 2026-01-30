
use crate::search::writer::Partition;
use crate::search::writer::Children;
use crate::Number;
use super::super::*;

#[derive(Debug, Clone)]
pub struct OrWriter<N: Number, const C: usize> {
    constant_cap: u8,
    partition: Partition,
    children: Children<N, C>,
}

impl<N: Number, const C: usize> OrWriter<N, C> {
    pub fn new(length: usize, constant_cap: u8) -> Self {
        let mut initial_partition = Partition::standard(length);
        initial_partition.next();

        Self {
            constant_cap,
            children: Children::standard(CHILD_OF_OR, constant_cap, OR, &initial_partition.state()),
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
                self.children = Children::standard(CHILD_OF_OR, self.constant_cap, OR, &self.partition.state());
                continue;
            }

            return false;
        }
    }
}

