
mod partition;
mod children;

pub use partition::Partition; // temporary. todo: make not public
pub use children::Children;

use crate::search::pivot::Pivot::*;
use crate::search::pivot::Op;
use crate::Number;

use std::marker::PhantomData;

// Rules to remember:
//    — These op levels are fully commutative and associative: | ^ &
//    — These op levels have commutativity/associativity rules: + -
//    — These op levels have commutative and associative ops: * / %
//    — These op levels have pretty much nothing: << >> ! -
//    — These op levels probably never care about multiple constants: | ^ & + -
//    — These op levels have ops that probably never care: * / %
//    — % does care about multiple constants (a%50%7)

// In all content below, an expression of length N is considered
// to honorarily comprise N+1 bytes, with the first byte being an
// unwritten additional '|' sign before the first component. Each
// component, including the first, therefore takes up exactly one
// more byte than its actual length. We'll consider an expression
// of actual length 12, and so virtual length 13.
//
// To avoid wasting computation, we only want to fiddle with the
// length distributions as many times as we need to. So for example,
// the first thing an OrWriter will try is to allocate all of its
// bytes to a single XorWrite, so the pattern is 13. Once this
// XorWriter is exhaused, the OrWriter will try another allocation
// of bytes. It peels back the 13 and has 13 bytes to spare. It can
// try writing an 12, leaving 1 byte to spare, but we are forbidden
// to write a 1, so that doesn't work. So it peels back the 12 and
// writes an 11, leaving 2 bytes to spare, and then it can write a
// 2, leaving 0 bytes to spare, for the pattern 11+2.
//
// Now it will increment the 11 (actually 10) and 2 (actually 1)
// in a cycle until they both collectively run out.
//
// Next, it will try another combination of lengths. It will peel
// back the 2, and since it's a 2, it can't write anything else
// there. So it peels back the 11 and has 13 bytes to spare. It will
// try writing a 10, leaving 3 bytes, and then it can write a 3,
// for the pattern 10+3.
//
// Once that's done, it peels back the 3. It can write a 2, for
// a 10+2, leaving 1 byte, but then it's stuck, so it peels back
// the 2, leaving 10, peels back the 10, tries a 9, and then writes
// a 4, for 9+4.
//
// Once that's done, it goes to 9+2+2, and so on.


// Let's try writing that AddSubtractWriter algorithm.
//
// This is another situation where an expression with N actual bytes can
// be considered to have N+1 virtual bytes, and every component can be
// considered to have a virtual operator previx (even the first, which
// really doesn't have one).
//
// An AddSubtract expression allocates zero or more (and not exactly one)
// virtual bytes to the subtracted terms, and two or more virtual bytes to
// the added terms. Each group is a separate non-decreasing partition. Just
// like with the purely regular levels, the AddSubtract level cycles all of
// its child expressions until they run out, and only then does it meddle
// with the byte allocations. First, it tries to cycle the byte allocations
// within the subtracted terms, if there are any. Then, only if we've run
// out of those (or there aren't any) we cycle the byte allocations of the
// added terms, and reset the allocations of the subtracted terms. Finally,
// only when we run out of allocations for the added terms, we change the
// allocation of overall bytes between the added-terms bucket and the
// subtracted-terms bucket. When all such bucket allocations have been
// exhausted, we are done.



// Now let's write the AddSubtract writer.

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
        let add_partition = Partition::new(length);
        let sub_partition = Partition::new(0);

        Self {
            length,
            nothing: PhantomData,

            bytes_add: length,
            add_children: Children::new_from_sizes(&add_partition.state()),
            sub_children: Children::extender(&sub_partition.state()),
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
                self.sub_children = Children::extender(&self.sub_partition.state());
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
            self.sub_children = Children::extender(&self.sub_partition.state());
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
            self.add_children = Children::new_from_sizes(&self.add_partition.state());
            self.add_children.do_first_write(dest);

            if self.bytes_add < self.length {
                self.sub_partition = Partition::new(self.length - self.bytes_add - 1);
                self.sub_children = Children::extender(&self.sub_partition.state());
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

            self.add_partition = Partition::new(self.bytes_add);
            self.add_children = Children::new_from_sizes(&self.add_partition.state());
            self.add_children.do_first_write(dest);

            self.sub_partition = Partition::new(self.length - self.bytes_add - 1);
            self.sub_children = Children::extender(&self.sub_partition.state());
            self.sub_children.do_first_write(&mut dest[self.bytes_add..]);

            return true;
        }

        // But if even that isn't possible, then we are truly done, and
        // we return false.

        false
    }
}


// This is implicitly an OrWriter.

pub struct Writer<N: Number> {
    // assume one input for now
    // assume no required vars for now
    // minimum precedence is meaningless here
    // assume constant_cap is 155 for now

    // intrinsic properties

    length: usize,
    nothing: PhantomData<N>,

    // state

    partition: Partition,
    children: Children,
}

impl<N: Number> Writer<N> {
    pub fn new(_: usize, length: usize, _: u8, _: Option<Option<Op>>) -> Self {
        let initial_partition = Partition::new(length);

        Self {
            length,
            nothing: PhantomData,
            children: Children::new_from_sizes(&initial_partition.state()),
            partition: initial_partition,
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {
        if self.children.write(dest) {return true}

        // If we fell through to here, we have exhausted all our writers
        // and need to re-distribute bytes.

        // Quick-exit conditions:
        //     1. 2 and 3 cannot be written as sums at all.
        //     2. 4 can only be split as 2+2.
        //     3. 5 can only be split as 3+2.
        // Todo: consider the condition "self.children.len() * 2 == vlen".
        // Todo: or how about "self.children[0].length == 2".

        // let vlen = self.length + 1;
        // if vlen <= 3 {return false}
        // if vlen <= 5 && self.children.children.len() == 2 {return false}

        // If we didn't exit, try to go to the next partition.

        if self.partition.next() {
            self.children = Children::new_from_sizes(&self.partition.state());
            self.children.do_first_write(dest);
            return true;
        }

        // If we got to this point, we have no more paritions to cycle
        // through, so we are done.

        false
    }
}

