// Right triangles with integer coordinates
//
// The points P (x1, y1) and Q (x2, y2) are plotted at integer co-ordinates and are joined to the
// origin, O(0,0), to form ΔOPQ.
//
// There are exactly fourteen triangles containing a right angle that can be formed when each
// co-ordinate lies between 0 and 2 inclusive; that is,
// 0 ≤ x1, y1, x2, y2 ≤ 2.
//
// Given that 0 ≤ x1, y1, x2, y2 ≤ 50, how many right triangles can be formed?

#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;

use time::PreciseTime;
use euler::int::Gcd;
use std::cmp;

fn simplify(n: usize, d: usize) -> (usize, usize) {
    let mut mn = n;
    let mut md = d;

    loop {
        let r = mn.gcd(md);
        if r == 1 {
            return (mn, md);
        }
        mn /= r;
        md /= r;
    }
}

pub fn solve() -> usize {
    let nb = 1000;
    let mut count = 3 * nb * nb;

    for x in 1..nb+1 {
        for y in 1..nb+1 {
            let (dx, dy) = simplify(x, y);
            count += cmp::min((nb-y) / dx, x / dy);
            count += cmp::min((nb-x) / dy, y / dx);
        }
    }

    count
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("right triangles: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_90() {
        let s = solve();
        assert_eq!(14234, s);
    }

    #[bench]
    fn bench_90(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

