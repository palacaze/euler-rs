// Prime permutations
//
// The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330,
// is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit
// numbers are permutations of one another.
//
// There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes,
// exhibiting this property, but there is one other 4-digit increasing sequence.
//
// What 12-digit number do you form by concatenating the three terms in this sequence?

#![feature(test)]
extern crate test;

use std::collections::HashMap;

#[derive(Debug)]
struct PrimeCounter {
    v : Vec<u64>,
}

#[derive(Debug,Copy,Clone)]
struct PrimePerm {
    prime: u64,
    perm: u32
}

impl PrimeCounter {
    fn new() -> PrimeCounter {
        PrimeCounter { v :  Vec::new() }
    }

    fn is_prime(& self, n : u64) -> bool {
        let lim = (n as f32).sqrt() as u64 + 1;
        for x in &self.v {
            if *x > lim { return true; }
            if n % x == 0 { return false;
            }
        }
        true
    }
}

// prime numbers iterator
impl Iterator for PrimeCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n : Self::Item = match self.v.last() {
            Some(x) => x + 1,
            None => 2,
        };

        loop {
            if self.is_prime(n) {
                self.v.push(n);
                break;
            }
            n += 1;
        }
        Some(n)
    }
}

// with 4 digits, we need 3 bits per digit to account every digit,
// this is usable to compare permutations for numbers up to 9999999
fn account_digits(mut n: u64) -> u32 {
    let mut d: u32 = 0;
    while n != 0 {
        d += 1 << (3 * (n % 10));
        n /= 10;
    }
    d
}

pub fn solve() -> Vec<(u64, u64, u64)> {
    let nb = 10_000u64;
    let mut v = Vec::new();

    // prime numbers less than nb with a tag representing the digits of the number
    let primes = PrimeCounter::new();
    let primes_vec = primes.take_while(|x| x < &nb).filter(|x| x > &1000)
                           .map(|x| PrimePerm{prime: x, perm: account_digits(x)}).collect::<Vec<_>>();

    // a map for faster searching
    let primes_map = primes_vec.iter().map(|x| (x.prime, x.perm)).collect::<HashMap<_,_>>();

    for (i, p1) in primes_vec.iter().enumerate() {
        for p2 in primes_vec.iter().skip(i+1) {
            let p3v = 2 * p2.prime - p1.prime;
            
            // no need to try further, p3 would be too big
            if p3v >= nb { break; }

            if p1.perm == p2.perm {
                if let Some(a) = primes_map.get(&p3v) {
                    if *a == p1.perm {
                        v.push((p1.prime, p2.prime, p3v));
                    }
                }
            }
        }
    }

    v
}

fn main() {
    let s = solve();
    let b = s[1];
    println!("{}{}{}", b.0, b.1, b.2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_brute_49() {
        let s = solve();
        assert_eq!(2, s.len());
        assert_eq!(2969, s[1].0);
        assert_eq!(6299, s[1].1);
        assert_eq!(9629, s[1].2);
    }

    #[bench]
    fn bench_49(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

