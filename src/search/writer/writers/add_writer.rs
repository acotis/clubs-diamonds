
use crate::search::writer::partition::Partition;
use crate::search::writer::Children;
use crate::Number;

use super::super::*;

#[derive(Debug, Clone)]
pub struct AddWriter<N: Number, const C: usize> {
    length: usize,
    max_constant: Option<u128>,
    bytes_add: usize, // virtual bytes (includes the unwritten + sign at the start of the expression)
    add_partition: Partition,
    sub_partition: Partition,
    children: Children<N, C>,
}

impl<N: Number, const C: usize> AddWriter<N, C> {
    pub fn new(length: usize, max_constant: Option<u128>) -> Self {
        let mut add_partition = Partition::standard(length);
        let sub_partition = Partition::extender(0);
        add_partition.next();

        Self {
            length,
            max_constant,
            bytes_add: length,
            children: Children::dual(
                CHILD_OF_ADD,
                max_constant,
                ADD, &add_partition.state(),
                SUB, &sub_partition.state(),
            ),
            add_partition,
            sub_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        loop {
            if self.children.write(dest) {
                return true;
            }

            // If that failed, we need to increment the partitioning. Since
            // it is a dual parititon, this is slightly involved.

            if self.sub_partition.next() {
                self.children = Children::dual(
                    CHILD_OF_ADD,
                    self.max_constant,
                    ADD, &self.add_partition.state(),
                    SUB, &self.sub_partition.state(),
                );

                continue;
            }

            if self.add_partition.next() {
                self.sub_partition = Partition::extender(self.length - self.bytes_add);
                self.children = Children::dual(
                    CHILD_OF_ADD,
                    self.max_constant,
                    ADD, &self.add_partition.state(),
                    SUB, &self.sub_partition.state(),
                );

                continue;
            }

            if self.bytes_add > 1 {
                self.bytes_add -= if self.bytes_add == self.length {2} else {1};
                self.add_partition = Partition::standard(self.bytes_add);
                self.sub_partition = Partition::extender(self.length - self.bytes_add);
                self.children = Children::dual(
                    CHILD_OF_ADD,
                    self.max_constant,
                    ADD, &self.add_partition.state(),
                    SUB, &self.sub_partition.state(),
                );

                continue;
            }

            return false;
        }
    }
}

