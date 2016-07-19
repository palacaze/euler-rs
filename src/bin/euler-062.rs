// Permuted multiples
//
// It can be seen that the number, 125874, and its double, 251748,
// contain exactly the same digits, but in a different order.
//
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x,
// contain the same digits.

#![feature(test)]
extern crate test;

extern crate itertools;
use itertools::Itertools;

// a tag that represents a set of digits.
// All the permutations of these digits will produce the same tag
// with 6 bits per digit we can handle numbers up to 127 digits long
fn permut_tag(mut n: u64) -> u64 {
    let mut d: u64 = 0;
    while n != 0 {
        d += 1 << (6 * (n % 10));
        n /= 10;
    }
    d
}

pub fn solve() -> u64 {
    let perms = (1..10_000).map(|i| { let c = i*i*i; (permut_tag(c), c) })
                           .sorted().into_iter().group_by(|&(t,_)| t);

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
}

