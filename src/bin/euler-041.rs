// Pandigital prime
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits
// 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
//
// What is the largest n-digit pandigital prime that exists?

// The best seems to be going backward from 987654321, we will test all digits permutations
// for prime.
// In order to speed-up the process we will first generate all the primes < sqrt(987654321).

#[macro_use]
extern crate itertools;

use std::collections::HashSet;
use itertools::Itertools;


fn is_prime(n : &usize) -> bool {
    if *n == 1 { return false; }
    if *n == 2 { return true;  }

    let lim = (*n as f64).sqrt() as usize + 1;
    for x in 2..lim {
        if *n % x == 0 {
            return false;
        }
    }
    true
}

// determine if n is prime using a cache to speed up the computation
// here we assume that cache contains all the primes up to sqrt(n)
fn is_prime_with_cache(n : &usize, cache: &HashSet<usize>) -> bool {
    for x in cache {
        if *n % x == 0 {
            return false;
        }
    }
    true
}

// create a cache of prime numbers that can be used to test up to max_n
fn primes_cache(max_n: usize) -> HashSet<usize> {
    let lim = (max_n as f64).sqrt() as usize + 1;
    let mut cache = (3..lim).step(2).filter(is_prime).collect::<HashSet<_>>();
    cache.insert(2);
    cache
}

// Generate all the numbers through digits permutation from supplied list of digits.
// The digit_set order is respected, ie if the digits in digit_set are sorted,
// the output vector will also be sorted
fn digit_permutations(digit_set: &[usize]) -> Vec<usize> {
    let len = digit_set.len();
    if len == 1 { return digit_set.to_vec(); }

    let mut v = Vec::new();

    for i in 0..len {
        let mut s = digit_set.to_vec();
        let f = digit_set[i] * 10usize.pow(len as u32 -1);
        s.remove(i);
        let mut p = digit_permutations(&s);
        for e in p.iter_mut() {
            *e += f;
        }

        v.extend(p)
    }
    v
}

fn main() {
    // we test from biggest number of digits down
    for i in (2..10).rev() {
        let cache = primes_cache(10usize.pow(i as u32));
        // we create a reversed ordered list of digits in order to iterate
        // permutations also from the biggest down.
        // That way we know that the first encountered prime will be our answer
        let digits = (1..(i-1)).rev().collect::<Vec<_>>();
        for p in digit_permutations(&digits) {
            if is_prime_with_cache(&p, &cache) {
                println!("max = {:?}", p);
                return;
            }
        }
    }
}
