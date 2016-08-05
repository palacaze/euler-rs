// Path sum: three ways
//
// NOTE: This problem is a more challenging version of Problem 81.
//
// The minimal path sum in the 5 by 5 matrix below, by starting in any cell in the left column and
// finishing in any cell in the right column, and only moving up, down, and right, is indicated in
// red and bold; the sum is equal to 994.
//
// ⎛ 131   673   234 — 103 —  18 ⎞
// ⎜              |              ⎟
// ⎜ 201 —  96 — 342   965   150 ⎟
// ⎜                             ⎟
// ⎜ 630   803   746   422   111 ⎟
// ⎜                             ⎟
// ⎜ 537   699   497   121   956 ⎟
// ⎜                             ⎟
// ⎝ 805   732   524    37   331 ⎠
//
// Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K text
// file containing a 80 by 80 matrix, from the left column to the right column.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

use std::mem;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_data(path: &str) -> Vec<Vec<u32>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let len = data.lines().count();

    // we create a column-major matrix
    let mut mat = vec![vec![0; len]; len];
    for (r, line) in data.lines().enumerate() {
        for (c, n) in line.split(',').map(|c| c.parse::<u32>().unwrap()).enumerate() {
            mat[c][r] = n;
        }
    }
    mat
}

// min cost to get from curr line to target position on next line at index n
// allowed movements are left, right and up
fn min_cost(curr: &[u32], next: &[u32], n: usize) -> u32 {
    let mut base = curr[n] + next[n];

    // before n
    let mut cost = next[n];
    for i in (0..n).rev() {
        cost += next[i];
        if cost > base { break; }
        if cost + curr[i] < base {
            base = cost + curr[i];
        }
    }

    // after n
    let mut cost = next[n];
    for i in n+1..curr.len() {
        cost += next[i];
        if cost > base { break; }
        if cost + curr[i] < base {
            base = cost + curr[i];
        }
    }
    base
}

pub fn solve(path: &str) -> u32 {
    let mat = read_data(path);
    let n = mat.len();

    // we aggregate min path sums by visiting pathes from left to right
    let mut sums = mat[0].clone();
    let mut temp = vec![0; n];

    for i in 1..n {
        for j in 0..n {
            temp[j] = min_cost(&sums, &mat[i], j);
        }
        mem::swap(&mut sums, &mut temp);
    }

    sums.into_iter().min().unwrap()
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let start = PreciseTime::now();
    let s = solve(&path);
    println!("min path: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_82() {
        let path = "data/p081_matrix.txt";
        let s = solve(&path);
        assert_eq!(260324, s);
    }

    #[bench]
    fn bench_82(b: &mut Bencher) {
        let path = "data/p081_matrix.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

