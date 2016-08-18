// Arithmetic expressions
//
// By using each of the digits from the set, {1, 2, 3, 4}, exactly once, and making use of the four
// arithmetic operations (+, −, *, /) and brackets/parentheses, it is possible to form different
// positive integer targets.
//
// For example,
//
// 8 = (4 * (1 + 3)) / 2
// 14 = 4 * (3 + 1 / 2)
// 19 = 4 * (2 + 3) − 1
// 36 = 3 * 4 * (2 + 1)
//
// Note that concatenations of the digits, like 12 + 34, are not allowed.
//
// Using the set, {1, 2, 3, 4}, it is possible to obtain thirty-one different target numbers of
// which 36 is the maximum, and each of the numbers 1 to 28 can be obtained before encountering the
// first non-expressible number.
//
// Find the set of four distinct digits, a < b < c < d, for which the longest set of consecutive
// positive integers, 1 to n, can be obtained, giving your answer as a string: abcd.

// - 5 operator application permutations (There are 6 but 2 are equivalents, when the middle one is applied last)
// - 126 digit combinations (choosing 4 numbers out of 9)
// - 24 permutations of digits
// - 64 combinations of operators (choosing 3 operations out of 4 with repetition)
// -> 967_680 tests in total, which is brute-force compliant

#![feature(test)]
extern crate test;
extern crate time;
extern crate itertools;

use itertools::Itertools;
use time::PreciseTime;

// digits permutations
static PERMS: &'static [[usize;4]] = &[
    [0,1,2,3],[1,0,2,3],[0,2,1,3],[1,2,0,3],[2,0,1,3],[2,1,0,3],
    [0,1,3,2],[1,0,3,2],[0,2,3,1],[1,2,3,0],[2,0,3,1],[2,1,3,0],
    [0,3,1,2],[1,3,0,2],[0,3,2,1],[1,3,2,0],[2,3,0,1],[2,3,1,0],
    [3,0,1,2],[3,1,0,2],[3,0,2,1],[3,1,2,0],[3,2,0,1],[3,2,1,0]];

#[derive(Copy,Clone,PartialEq,Eq)]
enum Op { Add, Sub, Mul, Div }

// operation combinations
fn op_combinations() -> Vec<(Op, Op, Op)> {
    let mut opc = Vec::with_capacity(64);
    let ops = &[Op::Add, Op::Sub, Op::Mul, Op::Div];

    for o1 in ops { for o2 in ops { for o3 in ops {
        opc.push((*o1, *o2, *o3))
    }}}

    opc
}

// operator application as a rational number
fn apply(op: Op, a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    if a.1 == 0 || b.1 == 0 { return (1, 0); }

    match op {
        Op::Add => (a.0*b.1 + b.0*a.1, a.1 * b.1),
        Op::Sub => (a.0*b.1 - b.0*a.1, a.1 * b.1),
        Op::Mul => (a.0 * b.0, a.1 * b.1),
        Op::Div => if b.0 == 0 { (1, 0) } else { (a.0 * b.1, a.1 * b.0) },
    }
}

// 4-digit combinations
fn digit_combinations() -> Vec<Vec<i32>> {
     (1..10).combinations_n(4).collect()
}

// mark a number as formed if positive and integer
fn mark(ok: &mut [bool], (p, q): (i32, i32)) {
    if q != 0 {
        let r = p / q;
        if r > 0 && p == r * q {
            ok[r as usize] = true;
        }
    }
}

pub fn solve() -> String {
    let ops = op_combinations();
    let mut suite = Vec::new();

    for digits in &digit_combinations() {
        let mut ok = vec![false; 9*8*7*6+1];

        for perm in PERMS {
            let a = (digits[perm[0]], 1);
            let b = (digits[perm[1]], 1);
            let c = (digits[perm[2]], 1);
            let d = (digits[perm[3]], 1);

            for op in &ops {
                // the 5 operator application orders
                mark(&mut ok, apply(op.0, a, apply(op.1, b, apply(op.2, c, d))));
                mark(&mut ok, apply(op.0, a, apply(op.2, apply(op.1, b, c), d)));
                mark(&mut ok, apply(op.2, apply(op.1, apply(op.0, a, b), c), d));
                mark(&mut ok, apply(op.2, apply(op.0, a, apply(op.1, b, c)), d));
                mark(&mut ok, apply(op.1, apply(op.0, a, b), apply(op.2, c, d)));
            }
        }

        suite.push((ok.iter().skip(1).take_while(|&i| *i).count(), digits.clone()));
    }

    suite.sort();
    let mut best = suite.last().unwrap().1.clone();
    best.sort();
    best.iter().fold(String::new(), |a, c| a + &c.to_string())
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("digits set: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_93() {
        let s = solve();
        assert_eq!("1258", s);
    }

    #[bench]
    fn bench_93(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

