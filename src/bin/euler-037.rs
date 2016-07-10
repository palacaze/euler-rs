// Truncatable primes
//
// The number 3797 has an interesting property. Being prime itself, it is possible to
// continuously remove digits from left to right, and remain prime at each stage:
// 3797, 797, 97, and 7. Similarly we can work from right to left: 3797, 379, 37, and 3.
//
// Find the sum of the only eleven primes that are both truncatable from left to right and right to left.
//
// NOTE: 2, 3, 5, and 7 are not considered to be truncatable primes.

// primes numbers can't be odd nor begin with a five, so 0,2,4,5,6,8 are banned
// moreover, 1 and 9 are not prime, so any conforming prime can't start of finish with a 1 or 9
// all in all, conforming numbers may only start an finish with 3 or 7
// as an exception, the first number may be 2 or 5, because those are primes and only appear
// for the last truncation, that gives a one-digit number

#[macro_use]
extern crate itertools;

extern crate euler;
use euler::primes;

use std::ops::Add;
use itertools::Itertools;

fn is_truncable_prime(n : &usize) -> bool {
    let mut q = *n;
    let mut r = 0;

    for i in 0.. {
        if q == 0 { break; }
        if ! primes::is_prime(q as u64) { return false; }
        r += (q % 10) * 10usize.pow(i);
        if ! primes::is_prime(r as u64) { return false; }
        q /= 10;
    }

    true
}

// Generate all the numbers through digits permutation from supplied lists
// of digits.
fn digit_permutations(digit_sets: &[Vec<usize>]) -> Vec<usize> {
    let len = digit_sets.len();
    if len == 1 { return digit_sets[0].clone(); }

    let perms = digit_permutations(&digit_sets[..(len-1)]);
    perms.iter().cartesian_product(digit_sets[len-1].iter())
               .map(|(v,c)| { *v * 10  + *c as usize }).collect_vec()
}

// generate all the truncable numbers of given digits count
fn truncable_primes(count: usize) -> Vec<usize> {
    let first_digit = vec![2, 3, 5, 7];
    let mid_digit   = vec![1, 3, 7, 9];
    let last_digit  = vec![3, 7];

    // the digit sets to be used for number generation
    let mut sets = vec![first_digit];
    for _ in 0..(count-2) { sets.push(mid_digit.clone()); }
    sets.push(last_digit);

    let mut perms = digit_permutations(&sets);
    perms.retain(is_truncable_prime);
    perms
}

fn main() {
    // brute force solution, takes 500 ms
    let r1 = (11..1_000_000).step(2).filter(is_truncable_prime).collect_vec();
    println!("{:?} -> sum = {}", r1, r1.iter().fold(0, Add::add));

    // smart solution, take only 4 ms
    let r2 = (2..7).flat_map(|i| truncable_primes(i).into_iter()).collect_vec();
    println!("{:?} -> sum = {}", r2, r2.iter().fold(0, Add::add));
}

