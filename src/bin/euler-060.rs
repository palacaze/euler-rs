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

#[derive(Debug, Clone)]
struct DPrime {
    n: usize,
    fac: usize,
}

impl DPrime {
    fn new(p: usize) -> Self {
        DPrime { n: p, fac: next_pow10(p) }
    }
}

fn next_pow10(n: usize) -> usize {
    let mut p = 10;
    while n > p { p *= 10; }
    p
}

pub fn solve() -> (usize, [usize; 5]) {
    let lim = 10_000;

    // primes under 10_000 except for 2 and 5 that can't work
    let mut primes = vec![DPrime::new(3)];
    primes.extend(primal::Primes::all().skip(3).take_while(|&n| n < lim).map(DPrime::new));

    // a cache of primes for faster lookup and function to test
    // concatenated primes using it
    let sieve = primal::Sieve::new(lim * lim);
    let are_coprimes = |a: &DPrime, b: &DPrime| {
        b.n > a.n && sieve.is_prime(a.n + a.fac * b.n) &&
                     sieve.is_prime(b.n + b.fac * a.n)
    };

    let mut best_sum = 1_000_000_000_000;
    let mut best_tup = [0; 5];

    for p1 in &primes {
        if p1.n >= best_sum { break; }
        let p2l = primes.iter().filter(|p2| are_coprimes(&p1, p2)).collect::<Vec<_>>();
        for p2 in &p2l {
            if p1.n + p2.n >= best_sum { break; }
            let p3l = p2l.iter().filter(|p3| are_coprimes(p2, p3)).collect::<Vec<_>>();
            for p3 in &p3l {
                if p1.n + p2.n + p3.n >= best_sum { break; }
                let p4l = p3l.iter().filter(|p4| are_coprimes(p3, p4)).collect::<Vec<_>>();
                for p4 in &p4l {
                    if p1.n + p2.n + p3.n + p4.n >= best_sum { break; }
                    if let Some(p5) = p4l.iter().filter(|p5| are_coprimes(p4, p5)).nth(0) {
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

