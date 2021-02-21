// Diophantine reciprocals I
//
// In the following equation x, y, and n are positive integers.
// 1/x + 1/y = 1/n
//
// For n = 4 there are exactly three distinct solutions:
// 1/5 + 1/20 = 1/4
// 1/6 + 1/12 = 1/4
// 1/8 + 1/8  = 1/4
//
// What is the least value of n for which the number of distinct solutions exceeds one-thousand?
//
// NOTE: This problem is an easier version of Problem 110; it is strongly advised that you solve
// this one first.

// Using a = x + y and b = x - n, and substitinq in the equation leads to:
// a = b + 2n + n² / b, which means that b divides n².
// If we assume n² = c.d, and choose b = c, so x = n + c we get:
// a = c + d + 2n  ->  y = n + d
// Now choosing b = d, we find x = n + d and y = n + c
//
// As the original equation is symmetric with respect to x and y,
// The number of proper divisors of n² gives the double of proper solutions to the problem.
// There is one exception though, when the c = d = n, which is counted only once.
// So given nd the number of divisors of n², the number of solutions to the equation is:
// (nd + 1) / 2.

// The number of proper divisors is easy to calculate.
// given a number n whose prime decomposition is (p0, a0), (p1, a1), ... (pq, aq)
// the number of divisors is (a0+1)(a1+1)...(aq+1).
// For n² this is (2.a0+1)(2.a1+1)...(2.aq+1) = 2 * nd - 1
//
// We are looking for nd > 1000 and (a0, a1,..., aq) such that p0,...,pq the first
// q prime numbers such that n = p0^a0 * p1^a1 * ... * pq^aq is minimal
//
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

// Build a pyramid count odd number in increasing order, whose folded product must be
// greater than t. Those numbers form the number of proper divisors of the n², the square
// of the that we are trying to minimize.
// Proceed recursively, the numbers are stored in fac and their partial product is part.
// The best n up to now is stored in n_best, which is f64 because it may exceed u64 size.
fn add_factor(count: usize, part: u32, t: u32, fac: &[u32], primes: &[usize], mut n_best: &mut f64) {
    let len = fac.len();
    let min = *fac.last().unwrap_or(&3);
    let max = max_factor(count - len, part, t);

    // last element, we calculate the value of n
    if fac.len() == count - 1 {
        let n = fac.iter().chain(&[max]).zip(primes.iter()).fold(1.0, |a, (&c, p)| a * (*p as f64).powi(((c-1)/2) as i32));
        if n <= *n_best {
            *n_best = n;
        }
    }
    else {
        let mut v = fac.to_vec();
        v.push(0);
        for e in (min..max+1).step_by(2) {
            v[len] = e;
            add_factor(count, part * e, t, &v, &primes, &mut n_best);
        }
    }
}

// Find the best n number composed of c prime factors,
// And with at least t proper divisors for n²
fn find_best_decomp_n(c: usize, t: u32, n_best: f64) -> f64 {
    let mut primes = primal::Primes::all().take(c).collect::<Vec<_>>();
    primes.reverse();
    let mut n = n_best;
    add_factor(c, 1, t, &[], &primes, &mut n);
    return n;
}

pub fn solve() -> f64 {
    let nb = 1_000;
    let tar = 2 * nb;   // number of proper divisors of n² to exceed
    let mut n_best = 1.0e100;

    // max number of primes factors that n may be composed of
    let max_factors = ((tar as f64).log2() / 3f64.log2()) as usize + 1;

    // we try to compose the best n with fixed count of prime factors
    for i in 3..max_factors+1 {
        let n = find_best_decomp_n(i, tar, n_best);
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
    fn test_108() {
        let s = solve();
        assert_eq!(180180f64, s);
    }

    #[bench]
    fn bench_108(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
