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

extern crate euler;
use euler::int::{Sqrt, PermutTag};

// calculate the totient using euler's formula
fn totient(i: usize, sieve: &primal::Sieve) -> usize {
    sieve.factor(i).unwrap().iter().fold(1, |a, &(p, c)| a * (p-1) * p.pow(c as u32 - 1))
}

struct MinTotient;

// min over n / phi(n)
impl rayon::par_iter::reduce::ReduceOp<(usize, usize)> for MinTotient {
    fn start_value(&self) -> (usize, usize) { (1, 0) }
    fn reduce(&self, v1: (usize, usize), v2: (usize, usize)) -> (usize, usize) {
        if v1.0 * v2.1 < v1.1 * v2.0 { v1 } else { v2 }
    }
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

// φ(n) = n.Π(1 - 1/p) where p are the prime factors of n
// n/φ(n) = Π(p/(p-1)) is minimal when n is prime and as big as possible
// but in that case, φ(n) = n-1 can't be a permutation of n
// so the next best candidates are semi/primes (product of 2 primes).
// we search for the product of 2 primes, so that φ(n) = (p1 - 1)(p2 - 1)
// and n/φ(n) = p1*p2 /((p1-1)(p2-1)), which is smallest when p1 is near p2
// and p1*p2 as big as possible
pub fn solve_smart() -> usize {
    let nb = 10_000_001;
    let primes = primal::Primes::all().take_while(|&i| i < nb / 2).collect::<Vec<_>>();
    let mut max = (1, 0);

    for a in &primes {
        for b in &primes {
            let p = a * b;
            if p >= nb { break; }

            let f = (a-1)*(b-1);
            if f.permut_tag() != p.permut_tag() {
                continue;
            }

            if p * max.1 < f * max.0 {
                max = (p, f);
            }
        }
    }
    max.0
}

fn main() {
    let s = solve_smart();
    println!("min totient quotient: {:?}", s);

    let s = solve_totient();
    println!("min totient quotient: {:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_smart_070() {
        let s = solve_smart();
        assert_eq!(8319823, s);
    }

    #[test]
    fn test_totient_070() {
        let s = solve_totient();
        assert_eq!(8319823, s);
    }

    #[bench]
    fn bench_070(b: &mut Bencher) {
        b.iter(|| black_box(solve_smart()));
    }
}

