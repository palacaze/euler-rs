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

#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;
extern crate itertools;
use itertools::Itertools;
use euler::int::Sqrt;
use time::PreciseTime;

fn prime_factors(n: usize) -> Vec<Vec<(usize,usize)>> {
    let mut f = vec![Vec::new(); n];
    for i in 2..n {
        // if i is prime
        if f[i].is_empty() {
            // add it as prime factor to all the multiples of i
            for k in (i..n).step_by(i) {
                f[k].push((i, 1));
            }

            // now we handle numbers with several i factors
            for p in (0..).scan(i, |a, _| {*a *= i; Some(*a)}) {
                if p > n { break; }
                for k in (p..n).step_by(p) {
                    f[k].last_mut().unwrap().1 += 1;
                }
            }
        }
    }

    f
}

fn square_root(n: usize) -> Option<usize> {
    let s = n.sqrt() as usize;
    if s * s == n { Some(s) } else { None }
}

// build combinations of prime factors of a²
fn prime_combinations(a: usize, fac: &[(usize, usize)]) -> Vec<usize> {
    // prime factors of a²
    // if p1, p2, ..., pn are the prime factors of a, with repetition r1, r2, ..., rn
    // we create a list of vectors of powers of prime factors:
    // [[1, p1, p1^2, ..., p1^r1], [1, p2, p2^2, ..., p2^r2], ..., [1, pn, pn^2, ..., pn^rn]]
    let fac = fac.iter()
        .map(|&(p,c)| (0..(2*c+1))
             .scan(1, |m, _| { let t = *m; *m *= p; Some(t) }).collect_vec())
        .collect_vec();

    let mut v = vec![1];
    for i in 0..fac.len() {
        v.sort();  // sort in order to allow early break
        let t = v;
        v = Vec::with_capacity(t.len() * fac[i].len());
        for m in &fac[i] {
            for &f in &t {
                let n = f * m;
                // as c-b < c+b, we can pick c-b < a
                if n > a { break; }
                v.push(n);
            }
        }
    }

    v
}

// a² + b² = c²  means a² = (c-b)(c+b)
// we assume with no loss of generality a < b < c
// so (c-b) divides a², which makes it easy to compose candidates
// from a prime factors.
pub fn solve_sieve() -> usize {
    let nb = 1_500_000;
    let mut count = vec![0; nb+1];
    let amax = (nb as f32 * (1.0 - 2f32.sqrt() / 2.0)) as usize;
    let facs = prime_factors(amax + 1);

    for a in 2..amax {
        // combination of prime factors of a² that represent c-b
        let c_minus_b = prime_combinations(a, &facs[a]);

        // a² = (c-b)(c+b), with a+b+c < L, so c-b > a² / (L-a)
        let a2 = a * a;
        let min = a2 / (nb - a);

        // deduce c+b from c-b, and b, c
        for cmb in c_minus_b.iter().filter(|&i| i >= &min) {
            let cpb = a2 / cmb;
            let b = (cpb - cmb) / 2;
            let c = (cpb + cmb) / 2;
            let l = a + cpb;
            if a < b && b < c && l <= nb && a2 + b*b == c*c {
                count[l] += 1;
            }
        }
    }

    count.into_iter().filter(|i| i == &1).count()
}

pub fn solve() -> usize {
    let n = 10000;
    let mut count = vec![0; n+1];
    let amax = (n as f32 * (1.0 - 2f32.sqrt() / 2.0)) as usize;

    for a in 2..amax {
        let bmax = n * (n - 2*a) / (2 * (n - a)) + 1;
        for b in a..bmax {
            if let Some(c) = square_root(a*a + b*b) {
                let sum = a + b + c;
                if c < b || sum > n { break; }
                count[sum] += 1;
            }
        }
    }

    count.into_iter().filter(|i| i == &1).count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("unique interger triangles: {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_sieve();
    println!("unique interger triangles: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_sieve_075() {
        let s = solve_sieve();
        assert_eq!(161667, s);
    }

    #[bench]
    #[ignore] // too long
    fn bench_075(b: &mut Bencher) {
        b.iter(|| black_box(solve_sieve()));
    }
}

