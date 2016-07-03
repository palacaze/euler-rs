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

#![feature(step_by)] 

#[macro_use]
extern crate itertools;

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

fn is_truncable_prime(n : &usize) -> bool {
    let mut q = *n;
    let mut r = 0;
    
    for i in 0.. {
        if q == 0 { break; }
        if !is_prime(&q) { return false; }
        r += (q % 10) * 10usize.pow(i);
        if !is_prime(&r) { return false; }
        q /= 10;
    }

    true
}

// generate the set.len() ^ count permutations with repetitions
// of count digits in set
fn perm_rep(count: usize, set: &Vec<u8>) -> Vec<Vec<u8>> {
    if count == 1 {
        // first digit
        return set.iter().map(|x| vec![*x]).collect::<Vec<_>>();
    }

    // previous digits
    let perm = perm_rep(count - 1, set);
    perm.iter().cartesian_product(set.iter()).map(|(v,c)| {
        let mut v = v.clone();
        v.push(*c);
        v }).collect::<Vec<_>>()
}

// generate a number from a digits permutation, a first digit and a last digit
fn gen_num(digits: &Vec<u8>, d1: u8, d2: u8) -> usize {
    let mut dig = vec![d1];
    dig.extend(digits.iter().cloned());
    dig.push(d2);
    dig.iter().enumerate().map(|(i,x)| (*x as usize) * 10usize.pow(i as u32)).fold(0, |a,c| a+c)
}

// generate all the candidate numbers with count digits
fn truncable_primes(count: usize) -> Vec<usize> {
    if count < 2  { return Vec::new();}    // nothing with 1 digit
    if count == 2 { return vec![23, 53, 37, 73]; } // only those numbers with 2 digits

    let perms = perm_rep(count - 2, &vec!(1u8, 3u8, 7u8, 9u8));
    perms.iter().cartesian_product(vec![(2,3),(2,7),(5,3),(5,7),(3,3),(3,7),(7,3),(7,7)].iter())
                .map(|(v,&(d1,d2))| gen_num(v,d1,d2))
                .filter(is_truncable_prime).collect::<Vec<_>>()
}

fn main() {
    // brute force solution, takes 500 ms
    let r1 = (11..1_000_000).step_by(2).filter(is_truncable_prime).collect::<Vec<_>>();
    println!("{:?} -> sum = {}", r1, r1.iter().fold(0, |a,c| a+c));

    // smart solution, take only 4 ms
    let r2 = (2..7).flat_map(|i| truncable_primes(i).into_iter()).collect::<Vec<_>>();
    println!("{:?} -> sum = {}", r2, r2.iter().fold(0, |a,c| a+c));
}
