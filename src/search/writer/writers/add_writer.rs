
use crate::Number;
use crate::search::writer::Partition;
use crate::search::writer::Children;

use super::super::*;

use std::marker::PhantomData;

pub struct AddWriter {
    length: usize,

    bytes_add: usize, // virtual bytes (includes the unwritten + sign at the start of the expression)
    add_partition: Partition,
    sub_partition: Partition,
    children: Children,
}

impl AddWriter {
    pub fn new(length: usize) -> Self {
        let mut add_partition = Partition::standard(length);
        let sub_partition = Partition::extender(0);
        add_partition.next();

        Self {
            length,
            bytes_add: length,
            children: Children::dual(
                CHILD_OF_ADD,
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
                    ADD, &self.add_partition.state(),
                    SUB, &self.sub_partition.state(),
                );

                continue;
            }

            if self.add_partition.next() {
                self.sub_partition = Partition::extender(self.length - self.bytes_add);
                self.children = Children::dual(
                    CHILD_OF_ADD,
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
                    ADD, &self.add_partition.state(),
                    SUB, &self.sub_partition.state(),
                );

                continue;
            }

            return false;
        }
    }
}

