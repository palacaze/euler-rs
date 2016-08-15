// Roman numerals
//
// For a number written in Roman numerals to be considered valid there are basic rules which must
// be followed. Even though the rules allow some numbers to be expressed in more than one way there
// is always a "best" way of writing a particular number.
//
// For example, it would appear that there are at least six ways of writing the number sixteen:
//
// IIIIIIIIIIIIIIII
// VIIIIIIIIIII
// VVIIIIII
// XIIIIII
// VVVI
// XVI
//
// However, according to the rules only XIIIIII and XVI are valid, and the last example is
// considered to be the most efficient, as it uses the least number of numerals.
//
// The 11K text file, roman.txt (right click and 'Save Link/Target As...'), contains one thousand
// numbers written in valid, but not necessarily minimal, Roman numerals; see About... Roman
// Numerals for the definitive rules for this problem.
//
// Find the number of characters saved by writing each of these in their minimal form.
//
// Note: You can assume that all the Roman numerals in the file contain no more than four
// consecutive identical units.

#![feature(conservative_impl_trait)]
#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

// use std::cmp;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_data(path: &str) -> impl Iterator<Item=String> {
    let path = Path::new(&path);
    BufReader::new(File::open(&path).unwrap())
        .lines().map(|s| s.unwrap())
}

// some sort of Dijkstra min distance algorithm
pub fn solve(path: &str) -> usize {
    read_data(path)
        .map(|s| {
            let mut count = 0;
            if s.contains("VIIII") { count += 3; }
            else if s.contains("IIII") { count += 2; }
            if s.contains("LXXXX") { count += 3; }
            else if s.contains("XXXX") { count += 2; }
            if s.contains("DCCCC") { count += 3; }
            else if s.contains("CCCC") { count += 2; }
            count
        })
        .sum()
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
    fn test_89() {
        let path = "data/p089_roman.txt";
        let s = solve(&path);
        assert_eq!(743, s);
    }

    #[bench]
    fn bench_89(b: &mut Bencher) {
        let path = "data/p089_roman.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

