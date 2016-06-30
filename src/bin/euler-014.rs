// Longest Collatz sequence
//
// The following iterative sequence is defined for the set of positive integers:
//
// n → n/2 (n is even)
// n → 3n + 1 (n is odd)
//
// Using the rule above and starting with 13, we generate the following sequence:
// 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//
// It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
// Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//
// Which starting number, under one million, produces the longest chain?
//
// NOTE: Once the chain starts the terms are allowed to go above one million.

#[inline]
fn is_odd(n : &usize) -> bool {
    n & 0x1 != 0
}

#[inline]
fn collatz(n : &usize) -> usize {
    if n < &2 { return 0; }
    if is_odd(&n) { 3 * n + 1 } else { n / 2 }
}

#[derive(Debug,Clone)]
struct CollatzSequence {
    cur : usize,
}

impl CollatzSequence {
    fn new(b : usize) -> CollatzSequence {
        CollatzSequence { cur : b }
    }
}

// A collatz sequence iterator
impl Iterator for CollatzSequence {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;

        if cur == 0 {
            return None;
        }
        if cur == 1 {
            self.cur = 0;
            return Some(1);
        }

        self.cur = collatz(&cur);
        Some(cur)
    }
}

// Producing all the collatz sequences is a lot of work, mostly riddled with
// duplicates. We want to minimize work here, so caching results for already
// encountered sub-sequences is a good idea.
// We don't wan't to cache arbitrarly high numbers, so as a tradeoff, we will
// only cache sequences lengthes up to one million.

fn main() {
    let nb = 1_000_000;

    // our cache, a sequence length for numbers from 0 to nb
    // a value of 0 means that the result has not been cached yet
    let mut cache = vec![0; nb];

    for i in 1..nb {
        let coll = CollatzSequence::new(i);
        let seq : Vec<usize> = coll.take_while(|&x| x >= nb || cache[x] == 0).collect();

        // cached length of already visited element, this would have been the next element
        // to seq, but it has been omitted by take_while, so we need to calculate it
        let len = if seq.is_empty() { 0 } else { cache[collatz(seq.last().unwrap())] };

        // store lengh of newly visited sub-sequences
        for (j, x) in seq.iter().rev().enumerate() {
            if *x < nb {
                // sequence length = newly traversed sequence length + known and cache end
                cache[*x] = len + j + 1;
            }
        }
    }
    
    let longest = cache.iter().enumerate().map(|(x, y)| (y, x)).max().unwrap();
    println!("longest at {}, length {}", longest.1, longest.0);
}
