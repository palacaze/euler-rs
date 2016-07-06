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

#![feature(step_by)]

#![feature(test)]
extern crate test;

const NUM: usize = 4;

fn next_prime_factor(n: usize) -> usize {
    if n == 1 { return 0; }
    if n % 2 == 0 { return 2;  }

    let lim = (n as f32).sqrt() as usize + 1;
    for x in (3..lim).step_by(2) {
        if n % x == 0 {
            return x;
        }
    }

    n
}

// a prime number has 0 prime factors
fn prime_factors(n: usize) -> Vec<usize> {
    let mut lst = Vec::new();
    let mut r = n;
    while r != 1 {
        let p = next_prime_factor(r);
        lst.push(p);
        r /= p;
    }
    lst
}

// a prime number has 0 prime factors
fn count_prime_factors(n: usize) -> usize {
    let mut lst = prime_factors(n);
    lst.sort();
    lst.dedup();
    lst.len()
}

pub fn solve() -> usize {
    let mut count = 0;
    for i in 644.. {
        if count_prime_factors(i) >= NUM {
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

