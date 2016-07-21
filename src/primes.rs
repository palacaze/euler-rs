extern crate test;

extern crate itertools;
use self::itertools::Itertools;

use std::collections::HashSet;
use super::int::Sqrt;

// base struct for a prime numbers iterator, that stores encountered primes
// in order to speed-up discovery of subsequent primes.
#[derive(Default)]
pub struct Primes {
    v: Vec<u64>,
}

impl Primes {
    pub fn new() -> Primes {
        Primes { v: Vec::new() }
    }
}

// prime numbers iterator
impl Iterator for Primes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n : Self::Item = match self.v.last() {
            Some(&2) => 3,
            Some(x) => x + 2,
            None => 2,
        };

        loop {
            if is_prime_with_cache(n, &self.v) {
                self.v.push(n);
                break;
            }
            n += 2;
        }
        Some(n)
    }
}

pub fn generate_primes(n: u64) -> Vec<u64> {
    let primes = Primes::new();
    primes.take_while(|x| x <= &n).collect::<Vec<_>>()
}

pub fn is_prime(n: u64) -> bool {
    if n == 1 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let lim = n.sqrt() + 1;
    for x in (5..lim).step(6) {
        if n % x == 0 || n % (x+2) == 0 {
            return false;
        }
    }
    true
}

// Use a list of precalculated primes to speedup primality computation
// NOTE: this is not a mere lookup, the prerequisite is that every prime
// under sqrt(n) is un the cache
// for a lookup use binary_search()
pub fn is_prime_with_cache(n: u64, lst: &[u64]) -> bool {
    if n == 1 { return false; }
    if n == 2 { return true; }
    let lim = n.sqrt() + 1;
    for x in lst {
        if *x > lim { return true; }
        if n % x == 0 { return false; }
    }
    true
}

// Lookup the prime from a set of precalculated primes
pub fn is_prime_with_set(n: u64, set: &HashSet<u64>) -> bool {
    set.contains(&n)
}

pub fn divisors(n: u64) -> Vec<u64> {
    if n == 1 { return vec![1]; }

    let mut s = n.sqrt();
    let square = s * s == n;
    if !square { s += 1 }

    // one is an obvious divisor
    let mut v: Vec<u64> = vec![1];

    // add every divisor below s
    v.extend((2..s).filter(|x| n % x == 0));
    if square { v.push(s); }

    // for every prime factor f below s, there also exists one
    // equal to n / f
    let len = v.len() - (square as usize);
    for i in (0..len).rev() {
        let r = n / v[i];
        v.push(r);
    }

    v
}

fn next_prime_factor(n: u64) -> u64 {
    if n == 1 { return 0; }
    if n % 2 == 0 { return 2; }

    let lim = n.sqrt() + 1;
    for x in (3..lim).step(2) {
        if n % x == 0 {
            return x;
        }
    }

    n
}

// a prime number has 1 prime factor, itself
pub fn prime_factors(mut n: u64) -> Vec<(u64, u64)> {
    let mut lst: Vec<(u64, u64)> = Vec::new();
    while n != 1 {
        let p = next_prime_factor(n);
        if lst.is_empty() {
            lst.push((p, 1));
        }
        else {
            let idx = lst.len() -1;
            if lst[idx].0 == p {
                lst[idx].1 += 1;
            }
            else {
                lst.push((p, 1));
            }
        }

        n /= p;
    }
    lst
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use super::*;
    use super::test::{Bencher, black_box};

    #[test]
    fn test_primes_iter() {
        let s = generate_primes(20);
        assert_eq!(&s, &[2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1u64), false);
        assert_eq!(is_prime(2u64), true);
        assert_eq!(is_prime(3u64), true);
        assert_eq!(is_prime(4u64), false);
        assert_eq!(is_prime(31u64), true);
        assert_eq!(is_prime(997651u64), true);
        assert_eq!(is_prime(51u64), false);
        assert_eq!(is_prime(997653u64), false);
    }

    #[test]
    fn test_is_prime_with_cache() {
        let cache = generate_primes(1_000u64);
        assert_eq!(is_prime_with_cache(1u64, &cache), false);
        assert_eq!(is_prime_with_cache(2u64, &cache), true);
        assert_eq!(is_prime_with_cache(3u64, &cache), true);
        assert_eq!(is_prime_with_cache(31u64, &cache), true);
        assert_eq!(is_prime_with_cache(997651u64, &cache), true);
        assert_eq!(is_prime_with_cache(51u64, &cache), false);
        assert_eq!(is_prime_with_cache(997653u64, &cache), false);
    }

    #[test]
    fn test_is_prime_with_set() {
        let set = HashSet::from_iter(generate_primes(1_000_000u64).into_iter());
        assert_eq!(is_prime_with_set(1u64, &set), false);
        assert_eq!(is_prime_with_set(2u64, &set), true);
        assert_eq!(is_prime_with_set(3u64, &set), true);
        assert_eq!(is_prime_with_set(31u64, &set), true);
        assert_eq!(is_prime_with_set(997651u64, &set), true);
        assert_eq!(is_prime_with_set(51u64, &set), false);
        assert_eq!(is_prime_with_set(997653u64, &set), false);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(&divisors(1u64), &[1]);
        assert_eq!(&divisors(2u64), &[1, 2]);
        assert_eq!(&divisors(3u64), &[1, 3]);
        assert_eq!(&divisors(4u64), &[1, 2, 4]);
        assert_eq!(&divisors(5u64), &[1, 5]);
        assert_eq!(&divisors(6u64), &[1, 2, 3, 6]);
        assert_eq!(&divisors(223u64), &[1, 223]);
        assert_eq!(&divisors(225u64), &[1, 3, 5, 9, 15, 25, 45, 75, 225]);
        assert_eq!(&divisors(224u64), &[1, 2, 4, 7, 8, 14, 16, 28, 32, 56, 112, 224]);
    }

    #[test]
    fn test_primes_factors() {
        assert_eq!(&prime_factors(1u64), &[]);
        assert_eq!(&prime_factors(2u64), &[(2, 1)]);
        assert_eq!(&prime_factors(3u64), &[(3, 1)]);
        assert_eq!(&prime_factors(4u64), &[(2, 2)]);
        assert_eq!(&prime_factors(5u64), &[(5, 1)]);
        assert_eq!(&prime_factors(6u64), &[(2, 1), (3, 1)]);
        assert_eq!(&prime_factors(223u64), &[(223, 1)]);
        assert_eq!(&prime_factors(225u64), &[(3, 2), (5, 2)]);
        assert_eq!(&prime_factors(224u64), &[(2, 5), (7, 1)]);
    }

    #[bench]
    fn bench_generate_primes_100(b: &mut Bencher) {
        b.iter(|| black_box(generate_primes(100)));
    }

    #[bench]
    fn bench_generate_primes_10_000(b: &mut Bencher) {
        b.iter(|| black_box(generate_primes(10_000)));
    }
}

