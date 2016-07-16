// Square root convergents
//
// It is possible to show that the square root of two can be expressed as an
// infinite continued fraction.
//
// √2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
//
// By expanding this for the first four iterations, we get:
//
// 1 + 1/2 = 3/2 = 1.5
// 1 + 1/(2 + 1/2) = 7/5 = 1.4
// 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
// 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
//
// The next three expansions are 99/70, 239/169, and 577/408, but the eighth
// expansion, 1393/985, is the first example where the number of digits in the
// numerator exceeds the number of digits in the denominator.
//
// In the first one-thousand expansions, how many fractions contain a
// numerator with more digits than denominator?

#![feature(test)]
extern crate test;

#[macro_use]
extern crate itertools;

extern crate gmp;
use gmp::mpz::Mpz;

use std::cmp;

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

// add 2 numbers represented as lists of digits
pub fn add3(a: &[u8], b: &[u8], c: &[u8]) -> Vec<u8> {
    let (ra, rb, rc) = if a.len() > b.len() {
            if a.len() > c.len() { (a, b, c) } else { (c, a, b) }
        }
        else {
            if b.len() > c.len() { (b, a, c) } else { (c, b, a) }
        };

    let mut v = Vec::with_capacity(ra.len() + 1);

    let mut carry = 0;

    for (da, db, dc) in izip!(ra.iter(), rb.iter().chain([0u8].iter().cycle()), rc.iter().chain([0u8].iter().cycle())) {
        let d = da + db + dc + carry;
        carry = d / 10;
        v.push((d % 10) as u8);
    }

    if carry > 0 {
        v.push(carry);
    }

    v
}

// hand made arithmetic
pub fn solve() -> usize {
    let nb = 1001;

    let mut c = 0;
    let mut n = vec![1u8];
    let mut d = vec![1u8];

    for _ in 2..nb {
        // add3() is faster than 2 add() calls
        let t = add3(&n, &d, &d);
        d = add(&n, &d);
        n = t;

        if n.len() > d.len() {
            c += 1;
        }
    }

    c
}

fn simplify(n: &Mpz, d: &Mpz) -> (Mpz, Mpz) {
    let mut mn = n.clone();
    let mut md = d.clone();

    loop {
        let r = mn.gcd(&md);
        if r == Mpz::one() {
            return (mn, md);
        }
        mn = &mn / &r;
        md = &md / &r;
    }
}

fn digits_count(n: &Mpz) -> usize {
    n.to_string().chars().count()
}

// gmp one, which is surprisingly 2 times slower without simplification
// and 4 times with
pub fn solve_gmp() -> usize {
    let nb = 1001;

    let mut c = 0;
    let mut n = Mpz::from(1);
    let mut d = Mpz::from(1);

    for _ in 2..nb {
        let t = &n + &d + &d;
        d = &n + &d;
        n = t;

        // test simplification of rational number
        let (sn, sd) = simplify(&n, &d);

        if digits_count(&sn) > digits_count(&sd) {
            c += 1;
        }
    }

    c
}

fn main() {
    let s = solve();
    println!("manual: {}", s);

    let s = solve_gmp();
    println!("gmp: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_57() {
        let s = solve();
        assert_eq!(153, s);
    }

    #[bench]
    fn bench_57(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }

    #[bench]
    fn bench_gmp_57(b: &mut Bencher) {
        b.iter(|| black_box(solve_gmp()));
    }
}

