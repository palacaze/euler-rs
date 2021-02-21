// Triangle containment Problem 102
//
// Three distinct points are plotted at random on a Cartesian plane, for which -1000 ≤ x, y ≤ 1000,
// such that a triangle is formed.
//
// Consider the following two triangles:
//
// A(-340,495), B(-153,-910), C(835,-947)
//
// X(-175,41), Y(-421,-714), Z(574,-645)
//
// It can be verified that triangle ABC contains the origin, whereas triangle XYZ does not.
//
// Using triangles.txt (right click and 'Save Link/Target As...'), a 27K text file containing the
// co-ordinates of one thousand "random" triangles, find the number of triangles for which the
// interior contains the origin.
//
// NOTE: The first two examples in the file represent the triangles in the example given above.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Triangle = Vec<isize>;

fn read_data(path: &str) -> impl Iterator<Item=Triangle> {
    let path = Path::new(&path);
    BufReader::new(File::open(&path).unwrap())
        .lines()
        .map(|s| s.unwrap().split(',')
                  .map(|x| x.parse::<isize>().unwrap())
                  .collect::<Triangle>())
}

fn cross(x0: isize, y0: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x0 - x1) * (y0 - y2) - (y0 - y1) * (x0 - x2)
}

pub fn origin_inside(t: &Triangle) -> bool {
    let d = cross(t[0], t[1], 0, 0, t[2], t[3]);
    d * cross(t[0], t[1], 0, 0, t[4], t[5]) < 0 &&
    d * cross(t[4], t[5], 0, 0, t[2], t[3]) < 0
}

pub fn solve(path: &str) -> usize {
    read_data(path).filter(origin_inside).count()
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let start = PreciseTime::now();
    let s = solve(&path);
    println!("origin in triangle: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_102() {
        let path = "data/p102_triangles.txt";
        let s = solve(path);
        assert_eq!(228, s);
    }

    #[bench]
    fn bench_102(b: &mut Bencher) {
        let path = "data/p102_triangles.txt";
        b.iter(|| black_box(solve(path)));
    }
}

