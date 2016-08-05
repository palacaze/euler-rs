// Path sum: two ways Problem 81
//
// In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by only
// moving to the right and down, is indicated in bold red and is equal to 2427.
// ⎛ 131   673   234   103    18 ⎞
// ⎜  |                          ⎟
// ⎜ 201 —  96 — 342   965   150 ⎟
// ⎜              |              ⎟
// ⎜ 630   803   746 — 422   111 ⎟
// ⎜                    |        ⎟
// ⎜ 537   699   497   121   956 ⎟
// ⎜                    |        ⎟
// ⎝ 805   732   524    37 — 331 ⎠
//
// Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K text
// file containing a 80 by 80 matrix, from the top left to the bottom right by only moving right
// and down.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

use std::mem;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_data(path: &str) -> Vec<Vec<i64>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    data.lines().map(|l| {
            l.split(',').map(|c| c.parse::<i64>().unwrap()).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn solve(path: &str) -> i64 {
    let mat = read_data(path);
    let n = mat.len();

    // we aggregate path sums by visiting pathes in diagonal
    let mut sums = vec![0; n];
    let mut t    = vec![0; n];
    sums[0] = mat[0][0];

    // top-left part
    for i in 1..n {
        t[0] = sums[0] + mat[0][i];
        t[i] = sums[i-1] + mat[i][0];
        for j in 1..i {
            t[j] = std::cmp::min(sums[j], sums[j-1]) + mat[j][i-j];
        }
        mem::swap(&mut t, &mut sums);
    }

    // bottom-left part
    for i in 1..n {
        for j in 0..n-i {
            sums[j] = std::cmp::min(sums[j], sums[j+1]) + mat[i+j][n-1-j];
        }
    }

    sums[0]
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
    fn test_81() {
        let path = "data/p081_matrix.txt";
        let s = solve(&path);
        assert_eq!(427337, s);
    }

    #[bench]
    fn bench_81(b: &mut Bencher) {
        let path = "data/p081_matrix.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

