// Passcode derivation
//
// A common security method used for online banking is to ask the user for three random characters
// from a passcode. For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th
// characters; the expected reply would be: 317.
//
// The text file, keylog.txt, contains fifty successful login attempts.
//
// Given that the three characters are always asked for in order, analyse the file so as to
// determine the shortest possible secret passcode of unknown length.

#![feature(test)]
extern crate test;
extern crate permutohedron;
extern crate time;

use permutohedron::Heap;
use time::PreciseTime;

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn read_data(path: &str) -> Vec<Vec<u8>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let mut v = data
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8)
                  .collect::<Vec<_>>())
        .collect::<Vec<_>>();
    v.sort();
    v.dedup();
    v
}

fn match_order(sample: &[u8], key: &[u8]) -> bool {

    let mut k = 0;
    for &e in sample {
        while k < key.len() && key[k] != e {
            k += 1;
        }
        if k >= key.len() {
            return false;
        }
    }

    true
}

pub fn solve(path: &str) -> String {
    let logs = read_data(path);
    let mut chars = logs.iter().flat_map(|s| s.clone()).collect::<Vec<_>>();
    chars.sort();
    chars.dedup();

    'next_key: for key in Heap::new(&mut chars) {
        for log in &logs {
            if !match_order(&log, &key) { continue 'next_key; }
        }

        return key.iter().map(|c| std::char::from_digit(*c as u32, 10).unwrap()).collect();
    }

    String::new()
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let start = PreciseTime::now();
    let s = solve(&path);
    println!("assuming unique digits: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_67() {
        let path = "data/p079_keylog.txt";
        let s = solve(&path);
        assert_eq!("73162890", s);
    }

    #[bench]
    fn bench_67(b: &mut Bencher) {
        let path = "data/p079_keylog.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

