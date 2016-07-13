// Self powers
//
// The series, 11 + 22 + 33 + ... + 1010 = 10405071317.
//
// Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

#![feature(test)]
extern crate test;

extern crate gmp;
use gmp::mpz::Mpz;

const NUM: usize = 1001;
const TENTEN: usize = 10_000_000_000;

pub fn solve_brute() -> usize {
    let div = Mpz::from(10).pow(10);
    let sum = (1..NUM).map(|i| Mpz::from(i as u64).pow(i as u32) % &div).fold(Mpz::zero(), |a,c|a+c);
    (Into::<Option<u64>>::into(&sum).unwrap() as usize) % TENTEN
}

fn exp_mod(mut b: usize, mut e: usize) -> usize {
    let mut r = 1;
    while e > 0 {
        if e % 2 == 1 {
            r = mul_mod(r, b);
        }
        e /= 2;
        b = mul_mod(b, b);
    }
    r
}

// calculate the product of x and b, with 10 digits truncation
fn mul_mod(mut x: usize, mut y: usize) -> usize {
    // we must truncate before and after to avoid overflow
    if x > TENTEN { x %= TENTEN }
    if y > TENTEN { y %= TENTEN }
    let a = x % 100_000;
    let b = y % 100_000;
    (x * b + (y - b) * a) % TENTEN
}

pub fn solve_partial() -> usize {
    ((1..NUM).map(|i| exp_mod(i, i)).fold(0, |a,c| a + c)) % TENTEN
}

fn main() {
    let val = solve_brute();
    println!("first 10 brute = {:?}", val);

    let val = solve_partial();
    println!("first 10 partial = {:?}", val);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_brute_48() {
        assert_eq!(9110846700, solve_brute());
    }

    #[bench]
    fn bench_brute_48(b: &mut Bencher) {
        b.iter(|| black_box(solve_brute()));
    }

    #[test]
    fn test_partial_48() {
        assert_eq!(9110846700, solve_partial());
    }

    #[bench]
    fn bench_partial_48(b: &mut Bencher) {
        b.iter(|| black_box(solve_partial()));
    }
}

