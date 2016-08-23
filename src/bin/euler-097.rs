// Large non-Mersenne prime
//
// The first known prime found to exceed one million digits was discovered in 1999, and is a
// Mersenne prime of the form 26972593−1; it contains exactly 2,098,960 digits. Subsequently other
// Mersenne primes, of the form 2p−1, have been found which contain more digits.
//
// However, in 2004 there was found a massive non-Mersenne prime which contains 2,357,207 digits:
// 28433 × 2^7830457+1.
//
// Find the last ten digits of this prime number.

#![feature(test)]
extern crate test;
extern crate time;
use time::PreciseTime;

const MOD: usize  = 100_000;
const MOD2: usize = 10_000_000_000;

fn mod_mul(a: usize, b: usize) -> usize {
    let ma = a % MOD;
    let mb = b % MOD;
    (b * ma + (a - ma) * mb) % MOD2
}

fn mod_pow(mut n: usize, mut p: usize) -> usize {
    if n == 1 { return 1; }

    let mut r = 1;

    loop {
        if p & 0x1 == 0x1 {
            r = mod_mul(r, n);
            if p == 1 { return r; }
        }
        p /= 2;
        n = mod_mul(n, n);
    }
}

pub fn solve() -> usize {
    mod_mul(28433, mod_pow(2, 7830457)) + 1
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("element of longest chain: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_97() {
        let s = solve();
        assert_eq!(8739992577, s);
    }

    #[bench]
    fn bench_97(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

