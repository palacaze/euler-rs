// Cubic permutations
//
// The cube, 41063625 (3453), can be permuted to produce two other cubes: 56623104 (3843) and
// 66430125 (4053). In fact, 41063625 is the smallest cube which has exactly three permutations of
// its digits which are also cube.
//
// Find the smallest cube for which exactly five permutations of its digits are cube.

#![feature(test)]
extern crate test;

extern crate euler;
use euler::int::PermutTag;

extern crate itertools;
use itertools::Itertools;

use std::collections::HashMap;

pub fn solve_looped() -> u64 {
    let mut hash = HashMap::new();

    for i in 10.. {
        let c = i * i * i;
        let e = hash.entry(c.permut_tag()).or_insert_with(Vec::new);
        e.push(c);
        if e.len() == 5 {
            return e[0];
        }
    }

    0
}

pub fn solve() -> u64 {
    let perms = (1..10_000)
        .map(|i| {
            let c = i * i * i;
            (c.permut_tag(), c)
        })
        .sorted()
        .into_iter()
        .group_by(|&(t, _)| t);

    let mut best_cube = u64::max_value();

    for (_, g) in perms {
        if g.len() == 5 {
            if let Some(m) = g.iter().min() {
                if m.1 < best_cube {
                    best_cube = m.1;
                }
            }
        }
    }

    best_cube
}

fn main() {
    let s = solve();
    println!("minimal cube: {}", s);

    let s = solve_looped();
    println!("minimal cube: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_62() {
        let s = solve();
        assert_eq!(127035954683, s);
    }

    #[bench]
    fn bench_62(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }

    #[bench]
    fn bench_loop_62(b: &mut Bencher) {
        b.iter(|| black_box(solve_looped()));
    }
}
