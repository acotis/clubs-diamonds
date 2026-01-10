
// First, let's factor out the Partition into its own struct. It
// will use the convention that every term in the sum is the actual
// length of the relevant component (not including the operator) and
// so the sum of all terms, plus the number of terms minus one, is
// the actual length of the overall expression.

pub struct Partition {
    state: Vec<usize>,
}

impl Partition {
    pub fn standard(total_len: usize) -> Self {
        Self {
            state: if total_len > 0 {vec![total_len]} else {vec![]},
        }
    }

    pub fn extender(total_len: usize) -> Self {
        Self {
            state: if total_len > 0 {vec![total_len - 1]} else {vec![]},
        }
    }

    pub fn state(&self) -> &[usize] {
        &self.state
    }

    pub fn next(&mut self) -> bool {
        
        // Step to the next byte allocation. You'd think this step would
        // be a huge pain, but it doesn't have to be. We can produce one
        // allocation from the previous by simply scanning through the
        // current allocation in reverse and finding the last chunk that
        // can be made smaller. A chunk can be made smaller if it is
        // greater than 1 and if by subtracting 1 or 2 from it it can be
        // left greater than 0 and the subtracted part, combined with the
        // total value of all the chunks after it, can be re-partitioned
        // into chunks not greater than the new value. This partitioning
        // is only impossible if the subtracted part is 1 and the chunks
        // following the decremented chunk had been a sequence of all
        // ones.

        //println!("  about to step to next allocation");

        let mut spare_bytes = 0;

        while let Some(pop) = self.state.pop() {
            spare_bytes += pop + 1;

            // If we just popped a 1, we can't go anywhere because
            // writing a 0 is forbidden, so continue.

            if pop == 1 {
                continue;
            }

            // If we just popped a 2, we can only push 1's, and that's
            // only possible if the number of spare bytes is even. If
            // we did, but it's not, continue.

            if pop == 2 && spare_bytes % 2 != 0 {
                continue;
            }

            // If we just popped a 3 or higher, we can decrement no
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

                let next_push = if last_push > spare_bytes - 1 {
                    spare_bytes - 1
                } else if last_push == spare_bytes - 1 || last_push + 3 <= spare_bytes {
                    last_push
                } else {
                    last_push - 1
                };

                self.state.push(next_push);
                spare_bytes -= next_push + 1;
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

#[test]
fn test_partition() {
    let mut partition = Partition::new(10);
    let mut printout = format!("");

    loop {
        printout +=
            &(partition
                .state()
                .iter()
                .map(|&len| "x".repeat(len))
                .collect::<Vec<_>>()
                .join("|")
                + "\n");

        if !partition.next() {
            break
        }
    }

    assert_eq!(
        printout,
        concat!(
            "xxxxxxxxxx\n",
            "xxxxxxxx|x\n",
            "xxxxxxx|xx\n",
            "xxxxxx|xxx\n",
            "xxxxxx|x|x\n",
            "xxxxx|xxxx\n",
            "xxxxx|xx|x\n",
            "xxxx|xxx|x\n",
            "xxxx|xx|xx\n",
            "xxxx|x|x|x\n",
            "xxx|xxx|xx\n",
            "xxx|xx|x|x\n",
            "xx|xx|xx|x\n",
            "xx|x|x|x|x\n",
        )
    );
}

