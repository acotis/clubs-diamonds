
use crate::Number;
use crate::search::writer::Partition;
use crate::search::writer::Children;

use super::super::*;

use std::marker::PhantomData;

pub struct AddSubtractWriter<N: Number> {
    length: usize,
    nothing: PhantomData<N>,

    bytes_add: usize, // virtual bytes (includes the unwritten + sign at the start of the expression)
    add_partition: Partition,
    sub_partition: Partition,
    add_children: Children,
    sub_children: Children,
}

impl<N: Number> AddSubtractWriter<N> {
    pub fn new(_: usize, length: usize, _: u8, _: Option<Option<Op>>) -> Self {
        let add_partition = Partition::standard(length);
        let sub_partition = Partition::extender(0);

        Self {
            length,
            nothing: PhantomData,

            bytes_add: length,
            add_children: Children::standard(ADD, &add_partition.state()),
            sub_children: Children::extender(SUB, &sub_partition.state()),
            add_partition,
            sub_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {

        //println!("  going to try writing subtracted chilren");

        if self.bytes_add < self.length && self.sub_children.write(&mut dest[self.bytes_add..]) {
            return true;
        }

        //println!("  going to try writing added children");

        if self.add_children.write(dest) {
            if self.bytes_add < self.length {
                self.sub_children = Children::extender(SUB, &self.sub_partition.state());
                self.sub_children.do_first_write(&mut dest[self.bytes_add..]);
            }
        
            return true;
        }

        // If we fell through to here, we have exhausted all our writers
        // and need to re-distribute bytes. The first thing to try is to
        // redistribute bytes among the subtracted components only. We
        // only try this if there are any bytes allocated to the subtracted
        // components at all.
        //
        // To rephrase, if there are subtracted components, and we
        // successfully increment the partitioning of their bytes, then
        // that incrementation is the "solution" to this Writer's overall
        // incrementation, and we return.

        //println!("  going to try incrementing subtracted partition");

        if self.bytes_add < self.length && self.sub_partition.next() {
            self.sub_children = Children::extender(SUB, &self.sub_partition.state());
            self.sub_children.do_first_write(&mut dest[self.bytes_add..]);

            return true;
        }

        // If that didn't work (either we don't have subtracted components
        // or we couldn't increment the partitioning of their bytes) then
        // the next thing to try is to increment the partitioning of the
        // added components' bytes. If this occurs, we also reset the
        // partitioning of the subtracted components.

        //println!("  going to try incrementing added partition");
        
        if self.add_partition.next() {
            self.add_children = Children::standard(ADD, &self.add_partition.state());
            self.add_children.do_first_write(dest);

            if self.bytes_add < self.length {
                self.sub_partition = Partition::extender(self.length - self.bytes_add);
                self.sub_children = Children::extender(SUB, &self.sub_partition.state());
                self.sub_children.do_first_write(&mut dest[self.bytes_add..]);
            };

            return true;
        }

        // Finally, if we couldn't even increment the partitioning of the
        // added components' bytes, the last resort is to change the
        // allocation of overall bytes between the added components and
        // the subtracted components. If this occurs, we also reset the
        // partitioning of added and subtracted components.

        //println!("  going to try shifting allocation");

        if self.bytes_add > 1 {
            self.bytes_add -= if self.bytes_add == self.length {2} else {1};

            self.add_partition = Partition::standard(self.bytes_add);
            self.add_children = Children::standard(ADD, &self.add_partition.state());
            self.add_children.do_first_write(dest);

            self.sub_partition = Partition::extender(self.length - self.bytes_add);
            self.sub_children = Children::extender(SUB, &self.sub_partition.state());
            self.sub_children.do_first_write(&mut dest[self.bytes_add..]);

            return true;
        }

        // But if even that isn't possible, then we are truly done, and
        // we return false.

        false
    }
}

