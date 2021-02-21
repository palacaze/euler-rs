// Cube digit pairs
//
// Each of the six faces on a cube has a different digit (0 to 9) written on it; the same is done
// to a second cube. By placing the two cubes side-by-side in different positions we can form a
// variety of 2-digit numbers.
//
// For example, the square number 64 could be formed:
//
//    —————————    —————————
//   |         |  |         |
//   |    6    |  |    4    |
//   |         |  |         |
//    —————————    —————————
//
// In fact, by carefully choosing the digits on both cubes it is possible to display all of the
// square numbers below one-hundred: 01, 04, 09, 16, 25, 36, 49, 64, and 81.
//
// For example, one way this can be achieved is by placing {0, 5, 6, 7, 8, 9} on one cube and {1,
// 2, 3, 4, 8, 9} on the other cube.
//
// However, for this problem we shall allow the 6 or 9 to be turned upside-down so that an
// arrangement like {0, 5, 6, 7, 8, 9} and {1, 2, 3, 4, 6, 7} allows for all nine square numbers to
// be displayed; otherwise it would be impossible to obtain 09.
//
// In determining a distinct arrangement we are interested in the digits on each cube, not the
// order.
//
// {1, 2, 3, 4, 5, 6} is equivalent to {3, 6, 4, 1, 2, 5} {1, 2, 3, 4, 5, 6} is distinct from {1,
// 2, 3, 4, 5, 9}
//
// But because we are allowing 6 and 9 to be reversed, the two distinct sets in the last example
// both represent the extended set {1, 2, 3, 4, 5, 6, 9} for the purpose of forming 2-digit
// numbers.
//
// How many distinct arrangements of the two cubes allow for all of the square numbers to be
// displayed?

#![feature(test)]
extern crate test;
extern crate time;
extern crate itertools;

use itertools::Itertools;
use time::PreciseTime;
use std::mem;
use std::convert;

#[derive(Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Digits(u16);

impl Digits {
    fn set(&mut self, d: usize) {
        assert!(d < 10);
        self.0 |= 1 << d;
    }

    fn is_set(&self, d: usize) -> bool {
        assert!(d < 10);
        self.0 & (1 << d) == 1 << d
    }

    fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn unset_digits(self) -> impl Iterator<Item=usize> {
        (0..10).filter(move |&i| !self.is_set(i))
    }
}

impl convert::From<usize> for Digits {
    fn from(n: usize) -> Self {
        let mut d = Digits::default();
        d.set(n);
        d
    }
}

fn arrangements(d: Digits) -> Vec<Digits> {
    if d.len() == 6 {
        return vec![d];
    }

    d.unset_digits()
        .combinations(6 - d.len())
        .map(|c| {
            let mut a = d;
            for i in c { a.set(i); }
            a
        })
        .collect()
}

fn extend_arr<'a>(arr: &'a [(Digits,Digits)], p: usize, q: usize) -> impl Iterator<Item=(Digits,Digits)>+'a {
    arr.iter().cloned()
        .map(move |mut a| {
            a.0.set(p);
            a.1.set(q);
            a
        })
        .chain(arr.iter().cloned()
            .map(move |mut a| {
                a.0.set(q);
                a.1.set(p);
                a
            })
        )
}

pub fn solve() -> usize {
    let digits = [(0,1), (0,4), (0,6), (1,6), (2,5), (3,6), (4,6), (6,4), (8,1)];
    let mut arr = vec![(Digits::from(digits[0].0),
                        Digits::from(digits[0].1))];

    // build call the combinations of pairs of lists of digits that
    // can represent all the squares
    for &(p, q) in &digits[1..] {
        let tmp = arr;
        arr = extend_arr(&tmp, p, q).collect::<Vec<_>>();

        // handle 6-9 equivalence
        if p == 6 {
            arr.extend(extend_arr(&tmp, 9, q));
        }
        if q == 6 {
            arr.extend(extend_arr(&tmp, p, 9));
        }
    }

    // remove lists that are too long for a dice
    arr.retain(|a| a.0.len() <= 6 && a.1.len() <= 6);

    // remove duplicates
    for a in arr.iter_mut() {
        if a.1 > a.0 { mem::swap(&mut a.0, &mut a.1); }
    }
    arr.sort();
    arr.dedup();

    // generate all arrangements, filling unused faces of each dice
    // with combinations of available digits
    let mut arr = arr.iter()
        .flat_map(|a| {
            let arr0 = arrangements(a.0);
            let arr1 = arrangements(a.1);
            arr0.into_iter()
                .cartesian_product(arr1.into_iter())
                .map(|(d0, d1)| {
                    if d0 < d1 { (d0,d1) } else { (d1,d0) }
                })
                .collect_vec()
        })
        .collect_vec();

    // remove duplicates again
    arr.sort();
    arr.dedup();

    arr.len()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("dice arrangements: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_90() {
        let s = solve();
        assert_eq!(1217, s);
    }

    #[bench]
    fn bench_90(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

