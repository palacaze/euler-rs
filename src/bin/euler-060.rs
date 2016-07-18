// Prime pair sets
//
// The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and concatenating
// them in any order the result will always be prime. For example, taking 7 and 109, both 7109 and
// 1097 are prime. The sum of these four primes, 792, represents the lowest sum for a set of four
// primes with this property.
//
// Find the lowest sum for a set of five primes for which any two primes concatenate to produce
// another prime.

#![feature(test)]
extern crate test;

extern crate primal;

extern crate euler;
use euler::int::{Digits};

#[derive(Debug, Clone)]
struct DPrime {
    n: usize,
    d: Vec<u8>,
}

impl DPrime {
    fn new(p: usize) -> Self {
        DPrime { n: p, d: p.to_digits() }
    }
}

fn are_concat_primes(a: &DPrime, b: &DPrime, sieve: &primal::Sieve) -> bool {
    let n = a.d.iter().rev().chain(b.d.iter().rev()).fold(0, |a, &d| 10 * a + d as usize);
    if !sieve.is_prime(n) {
        return false;
    }
    let n = b.d.iter().rev().chain(a.d.iter().rev()).fold(0, |a, &d| 10 * a + d as usize);
    sieve.is_prime(n)
}

pub fn solve() -> (usize, [usize; 5]) {
    // primes under 10_000 except for 2 and 5 that can't work
    let mut primes = vec![DPrime::new(3)];
    primes.extend(primal::Primes::all().skip(3).take_while(|&n| n < 10_000).map(DPrime::new));

    // a cache of primes for faster lookup
    let sieve = primal::Sieve::new(100_000_000);

    let mut best_sum = 1_000_000_000_000;
    let mut best_tup = [0; 5];

    for p1 in &primes {
        if p1.n >= best_sum { break; }
        let p2_list = primes.iter().filter(|p2| are_concat_primes(&p1, p2, &sieve)).collect::<Vec<_>>();
        for p2 in &p2_list {
            if p1.n + p2.n >= best_sum { break; }
            let p3_list = p2_list.iter().filter(|p3| are_concat_primes(p2, p3, &sieve)).collect::<Vec<_>>();
            for p3 in &p3_list {
                if p1.n + p2.n + p3.n >= best_sum { break; }
                let p4_list = p3_list.iter().filter(|p4| are_concat_primes(p3, p4, &sieve)).collect::<Vec<_>>();
                for p4 in &p4_list {
                    if p1.n + p2.n + p3.n + p4.n >= best_sum { break; }
                    if let Some(p5) = p4_list.iter().filter(|p5| are_concat_primes(p4, p5, &sieve)).nth(0) {
                        let sum = p1.n + p2.n + p3.n + p4.n + p5.n;
                        if sum < best_sum {
                            best_sum = sum;
                            best_tup = [p1.n, p2.n, p3.n, p4.n, p5.n];
                        }
                    }
                }
            }
        }
    }

    (best_sum, best_tup)
}

fn main() {
    let s = solve();
    println!("lowest sum: {}, primes: {:?}", s.0, s.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_60() {
        let s = solve();
        assert_eq!(26033, s.0);
    }

    #[bench]
    fn bench_60(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

