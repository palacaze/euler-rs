// Largest exponential
//
// Comparing two numbers written in index form like 2^11 and 3^7 is not difficult, as any
// calculator would confirm that 2^11 = 2048 < 3^7 = 2187.
//
// However, confirming that 632382^518061 > 519432^525806 would be much more difficult, as both
// numbers contain over three million digits.
//
// Using base_exp.txt (right click and 'Save Link/Target As...'), a 22K text file containing one
// thousand lines with a base/exponent pair on each line, determine which line number has the
// greatest numerical value.
//
// NOTE: The first two lines in the file represent the numbers in the example given above.

#![feature(conservative_impl_trait)]
#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_data(path: &str) -> impl Iterator<Item=(usize, usize)> {
    let path = Path::new(&path);
    BufReader::new(File::open(&path).unwrap())
        .lines()
        .map(|s| {
            let s = s.unwrap();
            let mut i = s.splitn(2, ',').map(|x| x.parse::<usize>().unwrap());
            (i.next().unwrap(), i.next().unwrap())
        })
}

pub fn solve(path: &str) -> usize {
    read_data(path)
        .map(|(b, e)| (e as f64) * (b as f64).log2())
        .enumerate()
        .fold((0, 0f64), |a, x|
            if a.1 < x.1 { x } else { a }
        ).0 + 1
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let start = PreciseTime::now();
    let s = solve(&path);
    println!("longest anagramic square: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_99() {
        let path = "data/p099_base_exp.txt";
        let s = solve(path);
        assert_eq!(18769, s);
    }

    #[bench]
    fn bench_99(b: &mut Bencher) {
        let path = "data/p099_base_exp.txt";
        b.iter(|| black_box(solve(path)));
    }
}

