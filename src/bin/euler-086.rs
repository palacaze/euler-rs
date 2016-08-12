// Cuboid route
//
// A spider, S, sits in one corner of a cuboid room, measuring 6 by 5 by 3, and a fly, F, sits in
// the opposite corner. By travelling on the surfaces of the room the shortest "straight line"
// distance from S to F is 10 and the path is shown on the diagram.
//
// However, there are up to three "shortest" path candidates for any given cuboid and the shortest
// route doesn't always have integer length.
//
// It can be shown that there are exactly 2060 distinct cuboids, ignoring rotations, with integer
// dimensions, up to a maximum size of M by M by M, for which the shortest route has integer length
// when M = 100. This is the least value of M for which the number of solutions first exceeds two
// thousand; the number of solutions when M = 99 is 1975.
//
// Find the least value of M such that the number of solutions first exceeds one million.

// Given edges a, b and c, we can derive that the shortest distance are variations of
// L = sqrt((a+c)²+b²), which gives 3 potential solutions when permuting the 3 letters.
// Moreover, the smallest of the 3 variations always occurs when b is the biggest number.
// So, given the constraint a ≤ b ≤ c, the min distance is sqrt((a+b)²+c²)
// We thus reduced the problem to finding integer right triangles, we can use the
// results of problem 075 to that end.

#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate time;
extern crate itertools;

use itertools::Itertools;
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
// we assume with no loss of generality a ≤ b < c
// so (c-b) divides a², which makes it easy to compose candidates
// from a² prime factors.
// The post condition is a < mmax and b < 2 mmax
fn find_right_triangles(mmax: usize) -> Vec<(usize, usize)> {
    let mut sides = Vec::new();
    let facs = prime_factors(mmax);

    for a in 2..mmax {
        // combination of prime factors of a² that represent c-b
        let c_minus_b = prime_combinations(a, &facs[a]);

        let a2 = a * a;

        // deduce c+b from c-b
        for cmb in c_minus_b {
            let cpb = a2 / cmb;
            let b = (cpb - cmb) / 2;
            let c = (cpb + cmb) / 2;
            if a <= b && b <= c && b < 2 * mmax && a2 + b*b == c*c {
                sides.push((a, b));
            }
        }
    }

    sides
}

pub fn solve() -> usize {
    let mmax = 2000;

    // store count of cuboids per max room dimensions
    let mut dist = vec![0; mmax];

    // find all right triangles (p, q, _) with one side less than mmax
    // and the other less than 2.mmax.
    // Each triangle can lead to several (a, b, c) triplets so that either:
    // - p = a + b  and  q = c, or
    // - p = c  and  q = a + b
    // The prerequesites are a,b,c < mmax and a ≤ b ≤ c
    for (p, q) in find_right_triangles(mmax) {
        // first c = q
        if q < mmax {
            dist[q] += p/2;
        }

        // now c = p
        if p < mmax && q <= 2*p {
            dist[p] += p+1 - (q+1) / 2;
        }
    }

    // count distinct cuboids for each max room size
    dist.iter().skip(1)
        .scan(0, |a, &t| {
            let c = *a;
            *a += t;
            Some(c)
        })
        .take_while(|&i| i <= 1_000_000)
        .count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("max cube size: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_86() {
        let s = solve();
        assert_eq!(1818, s);
    }

    #[bench]
    fn bench_86(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

