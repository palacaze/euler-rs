// Ordered fractions
//
// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
// called a reduced proper fraction.
//
// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//
// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
// 6/7, 7/8
//
// It can be seen that 2/5 is the fraction immediately to the left of 3/7.
//
// By listing the set of reduced proper fractions for d ≤ 1,000,000 in ascending order of size,
// find the numerator of the fraction immediately to the left of 3/7.


#![feature(test)]
extern crate test;
extern crate euler;
use euler::int::Gcd;

pub fn solve() -> usize {
    let nb = 1_000_001;
    let mut best = (0, 1);

    for d in (5..nb).rev() {
        let n = 3 * d / 7;
        if 7 * n == 3 * d { continue; }

        // the gcd is not strictly needed, we can replace it
        // with a loose inequality, because if n/d can be reduced,
        // the reduced fraction will be encountered afterwards anyway
        if n * best.1 > d * best.0 && n.gcd(d) == 1 {
            best = (n, d);
        }
    }

    best.0
}

fn main() {
    let s = solve();
    println!("nearest: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_57() {
        let s = solve();
        assert_eq!(428570, s);
    }

    #[bench]
    fn bench_71(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

