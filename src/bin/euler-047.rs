// Distinct primes factors
//
// The first two consecutive numbers to have two distinct prime factors are:
//
// 14 = 2 × 7
// 15 = 3 × 5
//
// The first three consecutive numbers to have three distinct prime factors are:
//
// 644 = 2² × 7 × 23
// 645 = 3 × 5 × 43
// 646 = 2 × 17 × 19.
//
// Find the first four consecutive integers to have four distinct prime factors.
// What is the first of these numbers?

#![feature(test)]
extern crate test;

extern crate euler;

const NUM: usize = 4;

// a prime number has 1 prime factors
fn count_prime_factors(n: u64) -> usize {
    euler::primes::prime_factors(n).len()
}

pub fn solve() -> usize {
    let mut count = 0;
    for i in 644.. {
        if count_prime_factors(i as u64) >= NUM {
            count += 1;
        }
        else {
            count = 0;
        }
        if count == NUM {
            return i - NUM + 1;
        }
    }

    0
}

fn main() {
    let val = solve();
    println!("first {}-consecutive prime factors = {:?}", NUM, val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_pb() {
        assert_eq!(134043, solve());
    }

    #[bench]
    fn bench_pb(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

