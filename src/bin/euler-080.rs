// Square root digital expansion
//
// It is well known that if the square root of a natural number is not an integer, then it is
// irrational. The decimal expansion of such square roots is infinite without any repeating pattern
// at all.
//
// The square root of two is 1.41421356237309504880..., and the digital sum of the first one
// hundred decimal digits is 475.
//
// For the first one hundred natural numbers, find the total of the digital sums of the first one
// hundred decimal digits for all the irrational square roots.

// I think the text is wrong, if we sum only *decimal digits*, as stated, we should ignore the
// integral part of the irrational number. Here the first one hundred decimals of sqrt(2) sum to
// 481. but using the integral part and 99 decimal digits sum to 475.
// we first form a continued fraction of the square root of sufficient length, then expand it

#![feature(test)]
extern crate test;
extern crate time;
use time::PreciseTime;

extern crate euler;
use euler::int::Sqrt;

extern crate gmp;
use gmp::mpz::Mpz;

// continued fraction factorization from euler pb 064
fn factorize(s: u64) -> (u64, Vec<u64>) {
    let r = s.sqrt();
    if r * r == s {
        return (r, Vec::new());
    }

    let mut f = (r, r, 1);
    let mut v = Vec::new();

    loop {
        let d = (s - f.1 * f.1) / f.2;
        let a = (r + f.1) / d;
        let n = a * d - f.1;
        f = (a, n, d);

        if !v.is_empty() && v[0] == f {
            break;
        }

        v.push(f);
    }

    (r, v.iter().map(|f| f.0).collect())
}

// square root as rational number approx from euler pb 065
fn fraction(fac: &(u64, Vec<u64>), it: usize) -> (Mpz, Mpz) {
    let cf = fac.1.iter().cycle().cloned().take(it).collect::<Vec<_>>();

    let mut n = Mpz::zero();
    let mut d = Mpz::one();

    for i in cf.iter().rev() {
        let t = d.clone();
        d = Mpz::from(*i) * d + n;
        n = t;
    }

    n = &d * Mpz::from(fac.0) + n;
    (n, d)
}

pub fn solve() -> u64 {
    let nb = 101;
    let base = Mpz::from(10).pow(101 as u32);
    let mut count = 0;

    for i in 1..nb {
        let s = i.sqrt();

        // only irrational roots
        if s * s == i { continue; }

        // In order to produce a decimal part of at least 100 digits, both the
        // numerator denominator of the decimal part of the continued fraction
        // should be composed of at least 100 digits.
        // If a continued fraction has a sequence (a0; [a1, a2, a3]), the
        // numerator is bigger than the denominator, which in turn has a value
        // of at least a1*a2*a3*....
        // Ensuring enough digits means building a continued fraction with enough
        // cycles so that (a1 * a2 * a3)^u > 10^100
        // so  u > 100 / log10(a1*a2*a3)
        let fac = factorize(i);
        let u = 101.0 / (fac.1.iter().fold(1, |a, c| a * *c) as f64).log10();
        let (n, d) = fraction(&fac, (u as usize) * fac.1.len());
        let r = (&n * &base) / &d;

        // decimal part
        count += r.to_string().chars().take(100)
                  .map(|c| c.to_digit(10).unwrap() as u64)
                  .fold(0, |a, c| a + c);
    }

    count
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("sum decimal digits: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_080() {
        let s = solve();
        assert_eq!(40886, s);
    }

    #[bench]
    fn bench_080(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

