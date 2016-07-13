// The Fibonacci sequence is defined by the recurrence relation:
//
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//
// Hence the first 12 terms will be:
//
//     F1 = 1
//     F2 = 1
//     F3 = 2
//     F4 = 3
//     F5 = 5
//     F6 = 8
//     F7 = 13
//     F8 = 21
//     F9 = 34
//     F10 = 55
//     F11 = 89
//     F12 = 144
//
// The 12th term, F12, is the first term to contain three digits.
//
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

#![feature(test)]
extern crate test;

extern crate gmp;
use gmp::mpz::Mpz;

use std::mem;

#[derive(Debug)]
struct FibCounter {
    a : Mpz,
    b : Mpz,
}

impl FibCounter {
    fn new() -> FibCounter {
        FibCounter { a : Mpz::one(), b : Mpz::one() }
    }
}

impl Iterator for FibCounter {
    type Item = Mpz;
    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.a + &self.b;
        self.a = mem::replace(&mut self.b, c);
        Some(self.a.clone())
    }
}

pub fn solve() -> usize {
    let fib = FibCounter::new();
    let lim = Mpz::from(10).pow(999);
    let iter = fib.take_while(|x| x < &lim).count();

    // + 2 because we didn't step over F(0), and iter stop 1 before our goal
    iter + 2
}

fn main() {
    let val = solve();
    println!("iters = {:?}", val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_25() {
        assert_eq!(4782, solve());
    }

    #[bench]
    fn bench_25(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

