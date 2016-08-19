// Singular integer right triangles
//
// It turns out that 12 cm is the smallest length of wire that can be bent to form an integer sided
// right angle triangle in exactly one way, but there are many more examples.
//
// 12 cm: (3,4,5)
// 24 cm: (6,8,10)
// 30 cm: (5,12,13)
// 36 cm: (9,12,15)
// 40 cm: (8,15,17)
// 48 cm: (12,16,20)
//
// In contrast, some lengths of wire, like 20 cm, cannot be bent to form an integer sided right
// angle triangle, and other lengths allow more than one solution to be found; for example, using
// 120 cm it is possible to form exactly three different integer sided right angle triangles.
//
// 120 cm: (30,40,50), (20,48,52), (24,45,51)
//
// Given that L is the length of the wire, for how many values of L ≤ 1,500,000 can exactly one
// integer sided right angle triangle be formed?

#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;
use euler::int::Sqrt;
use time::PreciseTime;

// Let's call the lengths c, c and d, the area is A = h * d / 2 where
// h = sqrt(c² - d²/4), so b has to be even and c = d+1 or c = d-1.
// If a = d/2, is half the base of the isocele triangle, it is equivalent
// to searching for integer right triangles (a, b, c)
// where a² + b² = c² and c = 2a-1 or c = 2a+1
pub fn solve_brute() -> usize {
    let lmax = 1_000_000_000;
    let amax = lmax / 6 + 1;
    let mut sum = 0;

    for a in 2..amax {
        let b2 = 3 * a * a - 4 * a + 1;
        if b2.is_square() {
            sum += 6 * a - 2;
        }

        let b2 = 3 * a * a + 4 * a + 1;
        if b2.is_square() {
            sum += 6 * a + 2;
        }
    }

    sum
}

// Using the same idea from brute-force, a² + b² = c² and c = 2a-1 or c = 2a+1
// We also use euclid's formula for triplets generation:
// a = m² - n², b = 2mn, c = m² + n², such that a = 2n² + 1 or a = n² - 1.
// We only need to consider primitive triplets. Non-primitive triplets are the
// such that (ak, bk, ck) = k.(a, b, c), then the triangles are not almost
// equilateral anymore.
pub fn solve_euclid() -> usize {
    let lmax = 1_000_000_000;
    let amax = lmax / 6 + 1;
    let nmax = ((amax+1)/2).sqrt() + 1;
    let mut sum = 0;

    for n in 1..nmax {
        let n2 = n * n;

        let a = 2 * n2 + 1;
        let b2 = 3 * a * a - 4 * a + 1;
        if b2.is_square() {
            sum += 6 * a - 2;
        }

        let a = n2 + 2 * n; // next n
        let b2 = 3 * a * a + 4 * a + 1;
        if b2.is_square() {
            sum += 6 * a + 2;
        }
    }

    sum
}

fn main() {
    let start = PreciseTime::now();
    let s = solve_brute();
    println!("unique interger triangles: {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_euclid();
    println!("unique interger triangles: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_sieve_094() {
        let s = solve_euclid();
        assert_eq!(518408346, s);
    }

    #[bench]
    fn bench_094(b: &mut Bencher) {
        b.iter(|| black_box(solve_euclid()));
    }
}

