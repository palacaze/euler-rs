// Spiral primes
//
// Starting with 1 and spiralling anticlockwise in the following way, a square
// spiral with side length 7 is formed.
//
// 37 36 35 34 33 32 31
// 38 17 16 15 14 13 30
// 39 18  5  4  3 12 29
// 40 19  6  1  2 11 28
// 41 20  7  8  9 10 27
// 42 21 22 23 24 25 26
// 43 44 45 46 47 48 49
//
// It is interesting to note that the odd squares lie along the bottom right
// diagonal, but what is more interesting is that 8 out of the 13 numbers
// lying along both diagonals are prime; that is, a ratio of 8/13 ≈ 62%.
//
// If one complete new layer is wrapped around the spiral above, a square
// spiral with side length 9 will be formed. If this process is continued,
// what is the side length of the square spiral for which the ratio of primes
// along both diagonals first falls below 10%?

#![feature(test)]
extern crate test;

extern crate euler;
use euler::int::{Sqrt, Parity};

pub struct Diag {
    side: u64,
    count: u64,
}

impl Diag {
    pub fn new() -> Diag {
        Diag { side: 1, count: 0 }
    }
}

// diagonal iterator
impl Iterator for Diag {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let d = self.side * self.side - self.count * (self.side - 1);
        if self.count == 0 {
            self.side += 2;
            self.count = 3;
        }
        else {
            self.count -= 1;
        }
        Some(d)
    }
}

pub fn solve() -> u64 {
    let diag = Diag::new();
    let prime_lst = euler::primes::generate_primes(50_000);
    let r = diag.enumerate().skip(1)
                .scan(0, |primes, (i, d)| {
                       if euler::primes::is_prime_with_cache(d, &prime_lst) { *primes += 1; }
                       Some((*primes, i+2, d)) })
                  .skip_while(|&(p, t, _)| 10 * p >= t )
                  .nth(0).expect("should work");
    let s = r.2.sqrt();
    s + 1 + s.is_odd() as u64
}

fn main() {
    let s = solve();
    println!("side length: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_58() {
        let s = solve();
        assert_eq!(26241, s);
    }

    #[bench]
    fn bench_58(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

