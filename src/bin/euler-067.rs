// Maximum path sum II Problem 67
//
// By starting at the top of the triangle below and moving to adjacent numbers on the row below,
// the maximum total from top to bottom is 23.
//
// 3
// 7 4
// 2 4 6
// 8 5 9 3
//
// That is, 3 + 7 + 4 + 9 = 23.
//
// Find the maximum total from top to bottom in triangle.txt (right click and 'Save Link/Target
// As...'), a 15K text file containing a triangle with one-hundred rows.
//
// NOTE: This is a much more difficult version of Problem 18. It is not possible to try every route
// to solve this problem, as there are 299 altogether! If you could check one trillion (1012)
// routes every second it would take over twenty billion years to check them all. There is an
// efficient algorithm to solve it. ;o)

#![feature(test)]
extern crate test;

use std::str::FromStr;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_data(path: &str) -> Vec<Vec<usize>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    data.lines()
        .map(|l| l.split(' ')
                  .map(|c| usize::from_str(c).unwrap())
                  .collect::<Vec<_>>())
        .rev()
        .collect()
}

pub fn solve(path: &str) -> usize {
    let tri = read_data(path);

    // from the bottom-up, we will aggregate the best subpasses in order to find a result
    // in O(n²)
    let mut sub_len = tri[0].clone();
    let len = tri.len();

    for i in 1..len {
        for j in 0..len-i {
            sub_len[j] = std::cmp::max(sub_len[j], sub_len[j+1]) + tri[i][j];
        }
    }

    sub_len[0]
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let s = solve(&path);
    println!("max path sum: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_67() {
        let path = "data/p067_triangle.txt";
        let s = solve(&path);
        assert_eq!(7273, s);
    }

    #[bench]
    fn bench_67(b: &mut Bencher) {
        let path = "data/p067_triangle.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

