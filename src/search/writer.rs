
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
//
// First, let's factor out the OnelessParititon into its own struct. It
// will use the convention that every term in the sum is the virtual length
// of the relevant component, and that the sum of all terms is the virtual
// length of the whole expression.

struct OnelessPartition {
    state: Vec<usize>,
}

impl OnelessPartition {
    fn new(virtual_len: usize) -> Self {
        Self {
            state: vec![virtual_len],
        }
    }

    fn next(&mut self) -> bool {
        
        // Step to the next byte allocation. You'd think this step would
        // be a huge pain, but it doesn't have to be. We can produce one
        // allocation from the previous by simply scanning through the
        // current allocation in reverse and finding the last chunk that
        // can be made smaller. A chunk can be made smaller if it is
        // greater than 2 and if by subtracting 1 or 2 from it it can be
        // left greater than 1 and the subtracted part, combined with the
        // total value of all the chunks after it, can be re-partitioned
        // into chunks not greater than the new value. This partitioning
        // is only impossible if the subtracted part is 1 and the chunks
        // following the decremented chunk had been a sequence of all
        // twos.

        //println!("  about to step to next allocation");

        let mut spare_bytes = 0;

        while let Some(pop) = self.state.pop() {
            spare_bytes += pop;

            // If we just popped a 2, we can't go anywhere because
            // writing a 1 is forbidden, so continue.

            if pop == 2 {
                continue;
            }

            // If we just popped a 3, we can only push 2's, and that's
            // only possible if the number of spare bytes is even. If
            // we did, but it's not, continue.

            if pop == 3 && spare_bytes % 2 != 0 {
                continue;
            }

            // If we just popped a 4 or higher, we can decrement no
            // matter what followed it, so if we got to this point, we
            // know we can decrement.
            //
            // The refilling algorithm is the same no matter what we just
            // popped: push N-1, and then repeatedly push the largest
            // number you can push, up to a limit of the previous pushed
            // number, without getting stuck. You are only stuck if either
            // of these two conditions is met:
            //
            //   1. You have exactly one byte remaining.
            //   2. Your next push is forced to be a 1 and the number
            //      of remaining bytes is odd.

            let mut last_push = pop - 1;

            while spare_bytes > 0 {
                //println!("  spare bytes: {spare_bytes}");

                let next_push = if last_push > spare_bytes {
                    spare_bytes
                } else if last_push == spare_bytes || last_push + 2 <= spare_bytes {
                    last_push
                } else {
                    last_push - 1
                };

                self.state.push(next_push);
                spare_bytes -= next_push;
                last_push = next_push;
            }

            return true;
        }

        // If we got outside the loop, we unwound all of the terms and never
        // found one we could decrement, so we have run out of partitions
        // and should return false.

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

    // state

    partition: OnelessPartition,
    children: Vec<(usize, XorWriter<N>)>, // offsets and writers
}

impl<N: Number> Writer<N> {
    pub fn new(_: usize, length: usize, _: u8, _: Option<Option<Op>>) -> Self {
        Self {
            length,
            partition: OnelessPartition::new(length + 1),
            children: vec![(0, XorWriter::<N>::new(length))],
        }
    }

    pub fn write(&mut self, dest: &mut [u8]) -> bool {

        // In all content below, an expression of length N is considered
        // to honorarily comprise N+1 bytes, with the first byte being an
        // unwritten additional '|' sign before the first component. Each
        // component, including the first, therefore takes up exactly one
        // more byte than its actual length. We'll consider an expression
        // of actual length 12, and so virtual length 13.

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

        // Now it will increment the 11 (actually 10) and 2 (actually 1)
        // in a cycle until they both collectively run out.

        // Next, it will try another combination of lengths. It will peel
        // back the 2, and since it's a 2, it can't write anything else
        // there. So it peels back the 11 and has 13 bytes to spare. It will
        // try writing a 10, leaving 3 bytes, and then it can write a 3,
        // for the pattern 10+3.

        // Once that's done, it peels back the 3. It can write a 2, for
        // a 10+2, leaving 1 byte, but then it's stuck, so it peels back
        // the 2, leaving 10, peels back the 10, tries a 9, and then writes
        // a 4, for 9+4.

        // Once that's done, it goes to 9+2+2, and so on.

        let vlen = self.length + 1;
        let mut next_to_write = self.children.len()-1;

        loop {
            let (offset, child) = &mut self.children[next_to_write];
            if child.write(&mut dest[*offset..]) {return true}
            if next_to_write == 0 {break}
            next_to_write -= 1;
        }

        // If we fell through to here, we have exhausted all our writers
        // and need to re-distribute bytes.

        // Quick-exit conditions:
        //     1. 2 and 3 cannot be written as sums at all.
        //     2. 4 can only be split as 2+2.
        //     3. 5 can only be split as 3+2.
        // Todo: consider the condition "self.children.len() * 2 == vlen".
        // Todo: or how about "self.children[0].length == 2".

        if vlen <= 3 {return false}
        if vlen <= 5 && self.children.len() == 2 {return false}

        // If we didn't exit, go to the next partition. If that fails,
        // we're done, so return false.

        if !self.partition.next() {return false}

        // If we made it to this point, we have a new partition and should
        // set up our new children.

        let mut offset = 0;
        self.children.clear();

        for size in &self.partition.state {
            if offset > 0 {dest[offset-1] = b'|';}

            self.children.push((offset, XorWriter::<N>::new(size - 1)));
            self.children.last_mut().unwrap().1.write(&mut dest[offset..]);

            offset += size;
        }

        true
    }
}

struct XorWriter<N: Number> {
    length: usize,
    already_wrote: bool,
    nothing: PhantomData<N>,
}

impl<N: Number> XorWriter<N> {
    fn new(length: usize) -> Self {
        Self {
            length,
            already_wrote: false,
            nothing: PhantomData,
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

