// Prime digit replacements
//
// By replacing the 1st digit of the 2-digit number *3, it turns out that six
// of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.
//
// By replacing the 3rd and 4th digits of 56**3 with the same digit,
// this 5-digit number is the first example having seven primes among
// the ten generated numbers, yielding the family: 56003, 56113, 56333,
// 56443, 56663, 56773, and 56993. Consequently 56003, being the first member
// of this family, is the smallest prime with this property.
//
// Find the smallest prime which, by replacing part of the number (not
// necessarily adjacent digits) with the same digit, is part of an eight
// prime value family.

// First of all we observe that the last digit (least significant digit)
// can't be replaced, as it would lead to even numbers. Moreover the first
// digit can be one of (1, 3, 7, 9).
//
// Moreover, if we replace 1, 2, 4, 5 or 7, 8 digits, 3 among the 10 generated
// numbers would be multiples of 3, so those can be ignored. We only need to
// consider the case or 3 and possibly 6 digits replacement (6 is unlikely
// because there is no upper hand to the number of digits in the numbers).
// Additionaly, the sum of the not-replaced digits can't be a multiple of 3

// How many digits shoud we explore.
// - 4 is not possible. We can only change the last digit. 3 and 9 lead to
//   multiples of 3. For 1, 0001, 1111 and 9991 are not prime. For 7, 7777,
//   2227 and 3337 are not prime.
// - 5 we have to explore replacing 3 digits out of 4,

#![feature(test)]
extern crate test;

extern crate euler;

#[macro_use]
extern crate itertools;
use itertools::Itertools;

// A digit in the number can be either Replacable, Free or Fixed with a value
#[derive(Debug, Copy, Clone, PartialEq)]
enum Digit {
    Replace,
    Free,
    Fixed(u8),
}

#[derive(Debug, Clone, PartialEq)]
struct Number {
    digits: Vec<Digit>,
}

impl Number {
    fn new(n: Vec<Digit>) -> Self {
        Number { digits: n }
    }

    fn fixed_helper(v: Vec<Number>, to_fix: &[usize]) -> Vec<Number> {
        let len = to_fix.len();
        if len == 0 { return v; }

        let perms = Number::fixed_helper(v, &to_fix[..(len-1)]);
        perms.iter().cartesian_product(0..10)
                    .map(|(n, d)| {
                        let mut n2 = n.clone();
                        n2.digits[to_fix[len-1]] = Digit::Fixed(d);
                        n2 })
                    .collect_vec()
    }

    // Generate all the number combinations though fixing Free digits
    // Afterwards, all the digits are either of type Replace or Fixed
    fn fixed(&self) -> Vec<Number> {
        let to_fix = self.digits.iter().enumerate()
                           .filter_map(|(i,d)|
                                if *d == Digit::Free { Some(i) } else { None }
                            ).collect::<Vec<_>>();
        Number::fixed_helper(vec![self.clone()], &to_fix)
    }

    // Generate the 10 possible numbers by replacing "Replace" digits with
    // digits from 0 to 9
    // Afterwards, all th
    fn family(&self) -> Vec<u64> {
        (0..10).map(|i| {
                    self.digits.iter().map(|d| {
                        match *d {
                            Digit::Replace => i,
                            Digit::Free => panic!("There should not be any free digits"),
                            Digit::Fixed(f) => f,
                        }
                    }).fold(0u64, |a, c| 10 * a + c as u64)
                }).collect_vec()
    }
}

fn num_digits(mut n: u64) -> usize {
    let mut c = 0;
    while n != 0 {
        c += 1;
        n /= 10;
    }
    c
}

// we test a diqits configuration for any family matching our needs
fn test_mapping(n: &Number) -> Vec<u64> {
    let mut families = Vec::new();
    let v = n.fixed();

    for i in v {
        let primes = i.family().into_iter().filter(|&x| euler::primes::is_prime(x)).collect_vec();

        // We only consider this match if the first prime has the same
        // number of digits as n. Leading zeros are not allowed.
        if primes.len() >= 8 && num_digits(primes[0]) == n.digits.len() {
            families.push(primes[0]);
        }
    }

    families
}

pub fn solve() -> u64 {
    let mut res = Vec::new();

    // we will first test with 5 digit numbers (does not produce anything)
    let pat = Number::new(vec![Digit::Replace; 5]);
    for i in 1..4 {
        let mut n = pat.clone();
        n.digits[i] = Digit::Free;
        for e in &[1, 3, 7, 9] {
            n.digits[4] = Digit::Fixed(*e);
            res.extend_from_slice(&test_mapping(&n));
        }
    }

    res.sort();
    if !res.is_empty() { return res[0]; }

    // test with 6 digit numbers
    let pat = Number::new(vec![Digit::Replace; 6]);
    for i in 0..4 {
        for j in i+1..5 {
            let mut n = pat.clone();
            n.digits[i] = Digit::Free;
            n.digits[j] = Digit::Free;
            for e in &[1, 3, 7, 9] {
                n.digits[5] = Digit::Fixed(*e);
                res.extend_from_slice(&test_mapping(&n));
            }
        }
    }

    res.sort();
    if res.is_empty() { 0 } else { res[0] }
}

fn main() {
    let b = solve();
    println!("{:?}", b);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_51() {
        let s = solve();
        assert_eq!(121313, s);
    }

    #[bench]
    fn bench_51(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

