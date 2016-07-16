// Powerful digit sum
//
// A googol (10^100) is a massive number: one followed by one-hundred zeros;
// 100^100 is almost unimaginably large: one followed by two-hundred zeros.
// Despite their size, the sum of the digits in each number is only 1.
//
// Considering natural numbers of the form, a^b, where a, b < 100, what is the
// maximum digital sum?

// this time I will try to avoid big integer libraries

#![feature(test)]
extern crate test;

extern crate gmp;
use gmp::mpz::Mpz;

use std::cmp;

pub fn digits(mut n: usize) -> Vec<u8> {
    let mut v = Vec::new();
    while n != 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }
    v
}

// add 2 numbers represented as lists of digits
pub fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
    let (ra, rb) = if a.len() > b.len() { (a,b) } else { (b,a) };

    let len = cmp::max(a.len(), b.len()) + 1;
    let mut v = Vec::with_capacity(len);

    let mut carry = 0;

    for (da, db) in ra.iter().zip(rb.iter().chain([0u8].iter().cycle()))  {
        let d = da + db + carry;
        carry = d / 10;
        v.push((d % 10) as u8);
    }

    if carry > 0 {
        v.push(carry);
    }

    v
}

fn odd(n: usize) -> bool { n & 0x01 == 1 }

// Multiply 2 numbers one of them, potentially very big, represented
// as a list of digits. We do it through Egyptian multiplication using
// additions, O(log(n)).
pub fn mul(a: &[u8], mut b: usize) -> Vec<u8> {
    let mut n = a.to_owned();
    let mut r = Vec::new();
    loop {
        if odd(b) {
            r = add(&r, &n);
            if b == 1 {
                return r;
            }
        }

        b /= 2;
        n = add(&n, &n);
    }
}

// handmade arithmetic
pub fn solve() -> (usize, usize, usize) {
    let nb = 100;
    let mut best = (0, 0, 0);

    for a in 2..nb {
        let mut n = vec![1u8];
        for b in 1..nb {
            n = mul(&n, a);
            let count = n.iter().fold(0, |a, &c| a + (c as usize));
            if count > best.0 {
                best = (count, a, b);
            }
        }
    }

    best
}

fn sum_digits(n : &Mpz) -> usize {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).fold(0, |a, d| a + d as usize)
}

// gmp arithmetic
pub fn solve_gmp() -> (usize, usize, usize) {
    let nb = 100;
    let mut best = (0, 0, 0);

    for a in 2..nb {
        let ga = Mpz::from(a as u64);
        let mut n = Mpz::from(1);
        for b in 1..nb {
            n = n * &ga;
            let count = sum_digits(&n);
            if count > best.0 {
                best = (count, a, b);
            }
        }
    }

    best
}

fn main() {
    let s = solve();
    println!("max sum of digits for {} ^ {} = {}", s.1, s.2, s.0);

    let s = solve_gmp();
    println!("max sum of digits for {} ^ {} = {}", s.1, s.2, s.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_add_56() {
        assert_eq!(add(&digits(999), &digits(999)), &[8, 9, 9, 1]);
        assert_eq!(add(&digits(999), &digits(99)), &[8, 9, 0, 1]);
    }

    #[test]
    fn test_mul_56() {
        assert_eq!(mul(&digits(999), 11), &[9, 8, 9, 0, 1]);
    }

    #[test]
    fn test_56() {
        let s = solve();
        assert_eq!(972, s.0);
    }

    #[bench]
    fn bench_56(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }

    #[bench]
    fn bench_gmp_56(b: &mut Bencher) {
        b.iter(|| black_box(solve_gmp()));
    }
}

