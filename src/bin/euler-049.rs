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

#[macro_use]
extern crate itertools;
use itertools::Itertools;

// base struct for a prime numbers iterator, that stores encountered primes
// in order to speed-up discovery of subsequent primes.
#[derive(Debug)]
struct PrimeCounter {
    v : Vec<u64>,
}

impl PrimeCounter {
    fn new() -> PrimeCounter {
        PrimeCounter { v : Vec::new() }
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
fn count_digits(mut n: u64) -> u32 {
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
    // items are sorted then grouped by their tag. That way primes forming permutations
    // are grouped together since they share the same digits.
    let primes = PrimeCounter::new();
    let prime_groups = primes.take_while(|x| x < &nb)
                             .filter(|x| x > &1000)
                             .map(|x| (count_digits(x), x))
                             .sorted().into_iter()
                             .group_by(|&(t,_)| t);

    // iterate over permutation groups
    for (_, group) in prime_groups {
        if group.len() < 3 {
            continue;
        }

        for (i, p1) in group.iter().enumerate() {
            for p2 in group.iter().skip(i+1) {
                let p3v = 2 * p2.1 - p1.1;

                // no need to try further, p3 would be too big
                if p3v >= nb { break; }

                if let Some(p3) = group.iter().find(|&&x| x.1 == p3v) {
                    v.push((p1.1, p2.1, p3.1));
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
    fn test_49() {
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

