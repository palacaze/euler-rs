// Circular primes
//
// The number, 197, is called a circular prime because all rotations of the digits:
// 197, 971, and 719, are themselves prime.
//
// There are thirteen such primes below 100: 2, 3, 5, 7, 11, 13, 17, 31, 37, 71, 73, 79, and 97.
//
// How many circular primes are there below one million?

use std::collections::HashMap;

#[derive(Debug,PartialEq,Copy,Clone)]
enum Circular { Untested, Yes, No }

fn is_prime(n : &usize) -> bool {
    if *n == 1 { return false; }
    if *n == 2 { return true;  }

    let lim = (*n as f32).sqrt() as usize + 1;
    for x in 2..lim {
        if *n % x == 0 {
            return false;
        }
    }
    true
}

fn no_pair_or_five_digit(n: &usize) -> bool {
    let mut q = *n;
    while q != 0 {
        let r = q % 10;
        if r == 0 || r == 2 || r == 4 || r == 6 || r == 8 || r == 5 {
            return false;
        }
        q /= 10;
    }
    true
}

// we could do the log10() + 1 but not sure this is faster
fn count_digits(n: usize) -> usize {
    if n < 10 { return 1; }
    if n < 100 { return 2; }
    if n < 1000 { return 3; }
    if n < 10000 { return 4; }
    if n < 100000 { return 5; }
    if n < 1000000 { return 6; }
    if n < 10000000 { return 7; }
    8
}

fn circular_permutations(n: usize) -> Option<Vec<usize>> {
    let digits = count_digits(n);
    let fac = 10usize.pow(digits as u32 - 1);

    // generate digit permutations by extracting the last digits and replacing
    // the first with this one
    let mut v = (0..digits).scan(n, |x, _| {*x = (*x / 10) + (*x % 10) * fac; Some(*x) }).collect::<Vec<_>>();

    v.sort();    // we want unicity
    v.dedup();
    if v.is_empty() { None } else { Some(v) }
}

// First of all we will compute every prime under nb, it will serve as a
// cache, which will avoid duplicate computations.
// We can reduce the workload further by remarking that any prime with a pair digit (0,2,4,6,8) or
// a 5 can't be circular (at the exception of 5 itself) because one of its
// permutation with end with a 0 or 5, denoting a non-prime number.
// Then we will traverse the set, for each entry we will generate the permutations
// and test if they appear in the set.
// We need a marker, to set if each prime is, untested, circular or not circular.

fn main() {
    let nb = 1_000_000;
    let mut primes = (10..nb).filter(no_pair_or_five_digit).filter(is_prime)
                             .map(|x| (x, Circular::Untested)).collect::<HashMap<_,_>>();

    for (n, c) in &primes.clone() {
        // already visited
        if *c != Circular::Untested { continue; }

        if let Some(perms) = circular_permutations(*n) {
            // check if each permutation is inside the cache, thus proving circularity
            let is_circ = perms.iter().all(|p| primes.contains_key(p));

            // mark the new state
            for p in &perms {
                if let Some(v) = primes.get_mut(p) {
                    *v = if is_circ { Circular::Yes } else { Circular::No };
                }
            }
        }
    }

    // collect the results
    let count = primes.values().filter(|&v| *v == Circular::Yes).count();
    println!("number of circular primes under {}: {}", nb, count + 4); // we started from 10, so we miss 2, 3, 5, 7
}
