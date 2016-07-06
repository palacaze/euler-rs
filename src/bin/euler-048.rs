// Self powers
//
// The series, 11 + 22 + 33 + ... + 1010 = 10405071317.
//
// Find the last ten digits of the series, 11 + 22 + 33 + ... + 10001000.

#![feature(test)]
extern crate test;

extern crate num;
use num::bigint::{BigUint, ToBigUint};
use num::cast::ToPrimitive;

const NUM: usize = 1001;
const TENTEN: usize = 10_000_000_000;

pub fn solve_brute() -> usize {
    let div = num::pow(10u64.to_biguint().unwrap(), 10);
    let sum : BigUint = (1..NUM).map(|i| num::pow(i.to_biguint().unwrap(), i) % div.clone()).fold(num::zero(), |a,c|a+c);
    sum.to_usize().unwrap() % TENTEN
}

fn num_digits(n: usize) -> usize {
     let mut digits = 1;
     let mut pten = 10;
     while pten <= n {
         digits += 1;
         pten *= 10;
     }
     digits
}

// calculate the product of x and b, with 10 digits truncation
fn partial_prod(x: usize, b: usize) -> usize {
    // we must truncate before and after to avoid overflow
    let dx = num_digits(x);
    let db = num_digits(b);
    if dx > 5 && db > 5 { ((x % 100_000) * b) % TENTEN }
    else { (x * b) %  TENTEN }
}

pub fn solve_partial() -> usize {
    let mut sum = 0;
    for i in 1..NUM {
        let mut f = 1;
        for _ in 0..i {
            f = partial_prod(f, i);
        }
        sum += f;
    }
    sum % TENTEN
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

