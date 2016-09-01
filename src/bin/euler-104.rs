// Pandigital Fibonacci ends
//
// The Fibonacci sequence is defined by the recurrence relation:
//
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//
// It turns out that F541, which contains 113 digits, is the first Fibonacci number for which the
// last nine digits are 1-9 pandigital (contain all the digits 1 to 9, but not necessarily in
// order). And F2749, which contains 575 digits, is the first Fibonacci number for which the first
// nine digits are 1-9 pandigital.
//
// Given that Fk is the first Fibonacci number for which the first nine digits AND the last nine
// digits are 1-9 pandigital, find k.

#![feature(conservative_impl_trait)]
#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;

use time::PreciseTime;
use euler::int::PermutTag;

// A Fibionacci iterator that keep first and last digits
struct FibCounter {
    af: usize,
    al: usize,
    bf: usize,
    bl: usize,
}

impl FibCounter {
    fn new() -> FibCounter {
        FibCounter { af : 1, al: 1, bf : 1, bl: 1 }
    }
}

impl Iterator for FibCounter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        // last digits, this is modulo
        let c = (self.al + self.bl) % 1_000_000_000;
        self.al = self.bl;
        self.bl = c;

        // first digits, we keep more digits than necessary to
        // account for carries
        let c = self.af + self.bf;
        self.af = self.bf;
        self.bf = c;
        if self.af > 100_000_000_000_000_000 {
            self.af /= 10;
            self.bf /= 10;
        }

        let mut f = self.af;
        while f >= 1_000_000_000 {
            f /= 10;
        }

        Some((self.al, f))
    }
}

// a functor that test digits presence
fn same_digit_tester(n: usize) -> impl Fn(usize) -> bool {
    let tag = n.permut_tag();
    move |x| tag == x.permut_tag()
}

pub fn solve() -> usize {
    let tester = same_digit_tester(123456789usize);
    FibCounter::new().take_while(|&(x, y)| !(tester(y) && tester(x))).count() + 2
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("optimum set: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_104() {
        let s = solve();
        assert_eq!(329468, s);
    }

    #[bench]
    fn bench_103(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

