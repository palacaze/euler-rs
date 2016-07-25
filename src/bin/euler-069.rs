// Totient maximum
//
// Euler's Totient function, φ(n) [sometimes called the phi function], is used to determine the
// number of numbers less than n which are relatively prime to n. For example, as 1, 2, 4, 5, 7,
// and 8, are all less than nine and relatively prime to nine, φ(9)=6.
// n    Relatively Prime  φ(n)  n/φ(n)
// 2    1                 1     2
// 3    1,2               2     1.5
// 4    1,3               2     2
// 5    1,2,3,4           4     1.25
// 6    1,5               2     3
// 7    1,2,3,4,5,6       6     1.1666...
// 8    1,3,5,7           4     2
// 9    1,2,4,5,7,8       6     1.5
// 10   1,3,7,9           4     2.5
//
// It can be seen that n=6 produces a maximum n/φ(n) for n ≤ 10.
//
// Find the value of n ≤ 1,000,000 for which n/φ(n) is a maximum.

#![feature(test)]
extern crate test;
extern crate primal;

extern crate rayon;
use rayon::prelude::*;

extern crate itertools;
use itertools::Itertools;

extern crate euler;
use euler::int::{Sqrt, Parity};

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

struct MaxTotient;

// max over n / phi(n)
impl rayon::par_iter::reduce::ReduceOp<(usize, usize)> for MaxTotient {
    fn start_value(&self) -> (usize, usize) { (0, 1) }
    fn reduce(&self, v1: (usize, usize), v2: (usize, usize)) -> (usize, usize) {
        if v1.0 * v2.1 > v1.1 * v2.0 { v1 } else { v2 }
    }
}

// use rayon for parallel execution
pub fn solve_brute_par() -> usize {
    let nb = 1_000_001;
    let sieve = primal::Sieve::new(nb.sqrt()+1);
    let m = (3..nb)
        .into_par_iter()
        .map(|i| (i, count_coprimes(i, &sieve)))
        .reduce(&MaxTotient{});
    m.0
}

// sequential brute force
pub fn solve_brute() -> usize {
    let nb = 1_000_001;
    let sieve = primal::Sieve::new(nb.sqrt()+1);
    let m = (3..nb)
        .into_iter()
        .map(|i| (i, count_coprimes(i, &sieve)))
        .fold((0, 1), |v1, v2|  if v1.0 * v2.1 > v1.1 * v2.0 { v1 } else { v2 });
    m.0
}

// the number with the most prime factors is the one we want
pub fn solve() -> usize {
    let nb = 1_000_001;
    primal::Primes::all().scan(1, |prod, p| { *prod *= p; Some(*prod) }).take_while(|&x| x < nb).last().unwrap()
}

fn main() {
    let s = solve();
    println!("max totient quotient: {:?}", s);

    let s = solve_brute();
    println!("max totient quotient: {:?}", s);

    let s = solve_brute_par();
    println!("max totient quotient: {:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_069() {
        let s = solve();
        assert_eq!(510510, s);
    }

    #[test]
    fn test_brute_069() {
        let s = solve();
        assert_eq!(510510, s);
    }

    #[bench]
    fn bench_069(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

