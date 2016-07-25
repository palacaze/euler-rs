// Totient permutation
//
// Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
// number of positive numbers less than or equal to n which are relatively prime to n. For example,
// as 1, 2, 4, 5, 7, and 8, are all less than nine and relatively prime to nine, φ(9)=6.  The
// number 1 is considered to be relatively prime to every positive number, so φ(1)=1.
//
// Interestingly, φ(87109)=79180, and it can be seen that 87109 is a permutation of 79180.
//
// Find the value of n, 1 < n < 107, for which φ(n) is a permutation of n and the ratio n/φ(n)
// produces a minimum.

#![feature(test)]
extern crate test;
extern crate primal;

extern crate rayon;
use rayon::prelude::*;

extern crate itertools;
use itertools::Itertools;

extern crate euler;
use euler::int::{Sqrt, Parity, PermutTag};

// calculate the totient using euler's formula
fn totient(i: usize, sieve: &primal::Sieve) -> usize {
    sieve.factor(i).unwrap().iter().fold(1, |a, &(p, c)| a * (p-1) * p.pow(c as u32 - 1))
}

// calculate the totient of i, the brute-force way (before I searched on the internet)
fn count_coprimes(i: usize, sieve: &primal::Sieve) -> usize {
    // prime factors of i
    let divs = sieve.factor(i).unwrap().iter().map(|&(p,_)| p).collect::<Vec<_>>();
    let mut count = 0;

    // count co-primes encountered below i
    for &j in &divs {
        count += (i-1) / j;
    }

    // When a number is multiple of 2 prime factors, it has been counted twice, so
    // we remove duplicates. However, if it was multiples of 3 prime factors, we
    // just removed it 3 times so we must add it again...
    for c in 2..(divs.len()+1) {
        for m in divs.iter().combinations_n(c).map(|x| x.iter().fold(1, |a, &x| a*x)) {
            let num = (i-1) / m;
            if c.is_even() { count -= num; } else { count += num; }
        }
    }

    i - 1 - count
}

struct MinTotient;

// min over n / phi(n)
impl rayon::par_iter::reduce::ReduceOp<(usize, usize)> for MinTotient {
    fn start_value(&self) -> (usize, usize) { (1, 0) }
    fn reduce(&self, v1: (usize, usize), v2: (usize, usize)) -> (usize, usize) {
        if v1.0 * v2.1 < v1.1 * v2.0 { v1 } else { v2 }
    }
}

// use rayon for parallel execution
pub fn solve_brute() -> usize {
    let nb = 10_000_001;
    let sieve = primal::Sieve::new(nb.sqrt()+1);
    let m = (3..nb)
        .into_par_iter()
        .map(|i| (i, count_coprimes(i, &sieve)))
        .filter(|v| v.0.permut_tag() == v.1.permut_tag())
        .reduce(&MinTotient{});
    m.0
}

pub fn solve_totient() -> usize {
    let nb = 10_000_001;
    let sieve = primal::Sieve::new(nb.sqrt()+1);
    let m = (3..nb)
        .into_par_iter()
        .map(|i| (i, totient(i, &sieve)))
        .filter(|v| v.0.permut_tag() == v.1.permut_tag())
        .reduce(&MinTotient{});
    m.0
}

fn main() {
    let s = solve_brute();
    println!("min totient quotient: {:?}", s);

    let s = solve_totient();
    println!("min totient quotient: {:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_brute_070() {
        let s = solve_brute();
        assert_eq!(8319823, s);
    }

    #[test]
    fn test_totient_070() {
        let s = solve_totient();
        assert_eq!(8319823, s);
    }

    #[bench]
    #[ignore] // too long
    fn bench_070(b: &mut Bencher) {
        b.iter(|| black_box(solve_totient()));
    }
}

