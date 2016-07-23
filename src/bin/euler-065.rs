// Convergents of e
//
// The square root of 2 can be written as an infinite continued fraction.
//
// If we continue we would get the following expansion:
// √2 = 1 +          1
//          ————————————————————
//          2 +        1
//              ————————————————
//   	 	      2 +    1
//                  ————————————
//                   2 +   1
//                       ———————
//                       2 + ...
//
// The infinite continued fraction can be written, √2 = [1;(2)], (2) indicates
// that 2 repeats ad infinitum. In a similar way, √23 = [4;(1,3,1,8)].
//
// It turns out that the sequence of partial values of continued fractions for
// square roots provide the best rational approximations. Let us consider the
// convergents for √2.
//
// 1 + 1/2 = 3/2
//
//        1
// 1 + ——————— = 7/5
//     2 + 1/2
//
//        1
// 1 + ————————— = 17/12
//     2 +   1
//         —————
//         2+1/2
//
//              1
// 1 + ———————————————————— = 41/29
//     2 +        1
//         ————————————————
//  	      2 +   1
//             ————————————
//              2 +   1/2
//
// Hence the sequence of the first ten convergents for √2 are:
// 1, 3/2, 7/5, 17/12, 41/29, 99/70, 239/169, 577/408, 1393/985, 3363/2378, ...
//
// What is most surprising is that the important mathematical constant,
// e = [2; 1,2,1, 1,4,1, 1,6,1 , ... , 1,2k,1, ...].
//
// The first ten terms in the sequence of convergents for e are:
// 2, 3, 8/3, 11/4, 19/7, 87/32, 106/39, 193/71, 1264/465, 1457/536, ...
//
// The sum of digits in the numerator of the 10th convergent is 1+4+5+7=17.
//
// Find the sum of digits in the numerator of the 100th convergent of the
// continued fraction for e.

#![feature(test)]
extern crate test;

extern crate gmp;
use gmp::mpz::Mpz;

extern crate euler;
use euler::biguint::BigUint;
use euler::int::Digits;

pub fn solve_gmp() -> usize {
    let nb = 100;
    let mut cf = vec![1; nb-1];
    for i in 0..nb/3 {
        cf[3*i+1] = 2*(i+1);
    }

    let mut n = Mpz::zero();
    let mut d = Mpz::one();

    for i in cf.iter().rev() {
        let t = d.clone();
        d = Mpz::from(*i as u64) * d + n;
        n = t;
    }

    n = d * Mpz::from(2u64) + n;
    n.to_string().chars().map(|c|c.to_digit(10).unwrap()).fold(0, |a, c| a + c as usize)
}

pub fn solve() -> usize {
    let nb = 100;
    let mut cf = vec![1; nb-1];
    for i in 0..nb/3 {
        cf[3*i+1] = 2*(i+1);
    }

    let mut n = BigUint::zero();
    let mut d = BigUint::one();

    for i in cf.iter().rev() {
        let t = d.clone();
        d = *i * d + n;
        n = t;
    }

    n = 2u64 * d + n;
    n.to_digits().iter().fold(0, |a, c| a + *c as usize)
}

fn main() {
    let s = solve();
    println!("sum digits: {}", s);

    let s = solve_gmp();
    println!("sum digits: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_65() {
        let s = solve();
        assert_eq!(272, s);
    }

    #[bench]
    fn bench_65(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }

    #[bench]
    fn bench_gmp_65(b: &mut Bencher) {
        b.iter(|| black_box(solve_gmp()));
    }
}
