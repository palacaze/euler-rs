// Diophantine reciprocals II
// In the following equation x, y, and n are positive integers.
//
// 1/x + 1/y = 1/n
//
// It can be verified that when n = 1260, there are 113 distinct solutions and this is the least
// value of n for which the total number of distinct solutions exceeds one hundred.
//
// What is the least value of n for which the number of distinct solutions exceeds four million?
//
// NOTE: This problem is a much more difficult version of Problem 108 and as it is well beyond
// the limitations of a brute force approach it requires a clever implementation.

#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;
extern crate primal;

use euler::int::Parity;
use time::PreciseTime;

fn next_odd(n: u32) -> u32 {
    if n.is_odd() { n } else { n + 1 }
}

// The max val of a factor when there are still c factors to add
// and the product r = part * val^count must be greater or equal to t, unless
// r would not be optimal (reducing one of the c factors would still
// lead to r >= t
fn max_factor(count: usize, part: u32, t: u32) -> u32 {
    let tar = (t as f64) / (part as f64);
    let val = next_odd(tar.powf(1.0 / (count as f64)).ceil() as u32);
    let pow = val.pow(count as u32 - 1);
    if val > 1 && part * pow * (val - 2) > t { val - 2 } else { val }
}

// Build a pyramid of 'count' odd numbers in increasing order, whose folded product must be
// greater than t. Those numbers form the number of proper divisors of the n², the square
// of the that we are trying to minimize.
// Proceed recursively, the numbers are stored in fac and their partial product is part_t.
// The best n up to now is stored in n_best, which is f64 because it may exceed u64 size.
// The partial product of n is part_n.
fn add_factor(count: usize, mut part_n: f64, part_t: u32, t: u32, fac: &[u32], primes: &[f64], mut n_best: &mut f64) {
    let len = fac.len();
    let min = *fac.last().unwrap_or(&3);
    let max = max_factor(count - len, part_t, t);

    // last element, we calculate the value of n
    if fac.len() == count - 1 {
        let n = part_n * 2.0f64.powi(((max-1)/2) as i32);
        if n <= *n_best {
            *n_best = n;
        }
    }
    else {
        let mut v = fac.to_vec();
        v.push(0);
        part_n *= primes[len].powi(((min-1)/2) as i32);

        for e in (min..max+1).step_by(2) {
            if part_n > *n_best { return; } // early exit if already more than best solution
            v[len] = e;
            add_factor(count, part_n, part_t * e, t, &v, &primes, &mut n_best);
            part_n *= primes[len];
        }
    }
}

// Find the best n number composed of c prime factors,
// And with at least t proper divisors for n²
fn find_best_decomp_n(c: usize, t: u32, n_best: f64, primes: &[usize]) -> f64 {
    let mut primes = primes.iter().take(c).map(|i| *i as f64).collect::<Vec<_>>();
    primes.reverse();
    let mut n = n_best;
    add_factor(c, 1.0, 1, t, &[], &primes, &mut n);
    return n;
}

pub fn solve() -> f64 {
    let nb = 4_000_000;
    // let nb = 1_000;
    let tar = 2 * nb;   // number of proper divisors of n² to exceed
    let mut n_best = 1.0e100;

    // max number of primes factors that n may be composed of
    let max_factors = ((tar as f64).log2() / 3f64.log2()) as usize + 1;
    let primes = primal::Primes::all().take(max_factors).collect::<Vec<_>>();

    // we try to compose the best n with fixed count of prime factors
    for i in (2..max_factors+1).rev() {
        let n = find_best_decomp_n(i, tar, n_best, &primes);
        if n < n_best {
            n_best = n;
        }
    }

    n_best
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("smallest n: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_110() {
        let s = solve();
        assert_eq!(9350130049860600f64, s);
    }

    #[bench]
    fn bench_110(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
