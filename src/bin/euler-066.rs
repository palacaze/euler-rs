// Diophantine equation
//
// Consider quadratic Diophantine equations of the form:
//
// x2 – Dy2 = 1
//
// For example, when D=13, the minimal solution in x is 6492 – 13×1802 = 1.
//
// It can be assumed that there are no solutions in positive integers when D
// is square.
//
// By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, we obtain the
// following:
//
// 32 – 2×22 = 1
// 22 – 3×12 = 1
// 92 – 5×42 = 1
// 52 – 6×22 = 1
// 82 – 7×32 = 1
//
// Hence, by considering minimal solutions in x for D ≤ 7, the largest x is
// obtained when D=5.
//
// Find the value of D ≤ 1000 in minimal solutions of x for which the largest
// value of x is obtained.

// for this problem we approximate the square root of D as a continued fraction
// with as many steps as it needs to solve the equation.

#![feature(test)]
extern crate test;

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

// search for a continued fraction of d that solves the equation
fn find_d_root_fraction(d: u64) -> Mpz {
    let fac = factorize(d);
    let dd = Mpz::from(d);

    for i in 1.. {
        let (x, y) = fraction(&fac, i);
        if &x * &x == &dd * &y * &y + Mpz::one() {
            return x;
        }
    }

    Mpz::zero()
}

pub fn solve() -> u64 {
    let nb = 1001;
    let mut xmax = Mpz::zero();
    let mut dmax = 0;

    for d in 1..nb {
        if d.is_square() { continue; }
        let x = find_d_root_fraction(d);
        if x > xmax {
            xmax = x;
            dmax = d;
        }
    }

    dmax
}

fn main() {
    let s = solve();
    println!("D max: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_66() {
        let s = solve();
        assert_eq!(661, s);
    }

    #[bench]
    fn bench_66(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
