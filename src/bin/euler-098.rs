// Anagramic squares
//
// By replacing each of the letters in the word CARE with 1, 2, 9, and 6 respectively, we form a
// square number: 1296 = 36². What is remarkable is that, by using the same digital substitutions, the
// anagram, RACE, also forms a square number: 9216 = 96². We shall call CARE (and RACE) a square
// anagram word pair and specify further that leading zeroes are not permitted, neither may a
// different letter have the same digital value as another letter.
//
// Using words.txt (right click and 'Save Link/Target As...'), a 16K text file containing nearly
// two-thousand common English words, find all the square anagram word pairs (a palindromic word is
// NOT considered to be an anagram of itself).
//
// What is the largest square number formed by any member of such a pair?
//
// NOTE: All anagrams formed must be contained in the given text file.

#![feature(test)]
extern crate test;
extern crate time;
extern crate euler;

use euler::int::Sqrt;
use time::PreciseTime;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// read data an form a list of anagrams
fn read_data(path: &str) -> Vec<Vec<Vec<u8>>> {
    let path = Path::new(&path);
    let mut line = String::new();
    if let Err(_) = File::open(&path).unwrap().read_to_string(&mut line) {
        panic!("File reading failed");
    }
    let mut map = HashMap::new();

    for w in line.split(',') {
        let s = w.trim_matches('"').chars().map(|c| c as u8 - b'A').collect::<Vec<_>>();
        let mut k = s.clone();
        k.sort();
        let e = map.entry(k).or_insert_with(Vec::new);
        e.push(s);
    }

    map.into_iter().filter_map(|(_,v)| if v.len() > 1 {Some(v)} else {None}).collect()
}

// count digits in a number
fn to_digits(mut n: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(10);
    while n != 0 {
        let t = n / 10;
        v.push((n - 10*t) as u8);
        n = t;
    }
    v.reverse();
    v
}

// build the list of squares, sorted by number of digits
fn build_squares(len: usize) -> Vec<Vec<(usize, Vec<u8>)>> {
    let mut sq = vec![Vec::new(); len+1];
    let mut max = 10;
    let mut c = 1;

    for i in 1.. {
        let s = i * i;
        if s >= max {
            c += 1;
            max *= 10;
        }

        if c > len { break; }
        sq[c].push((s, to_digits(s)));
    }

    sq
}

// build a letter to digit lut from a square and a word
fn maybe_map(word: &[u8], digits: &[u8]) -> Option<[u8; 26]> {
    assert_eq!(word.len(), digits.len());
    let mut imap = [26u8; 10]; // digit  -> letter
    let mut map = [10u8; 26];  // letter -> digit

    for (&l, &d) in word.iter().zip(digits.iter()) {
        // each letter must map a unique digit
        let i = l as usize;
        if map[i] < 10 && map[i] != d {
            return None;
        }
        map[i] = d;

        // and each digit must represent a unique letter
        let i = d as usize;
        if imap[i] < 26 && imap[i] != l {
            return None;
        }
        imap[i] = l;
    }

    Some(map)
}

// attempt to find a digit - letters for a given list of anagrams and a square
fn try_mapping(words: &[Vec<u8>], sq: &(usize, Vec<u8>)) -> Option<usize> {
    let mut max = 0;
    for (i, word) in words.iter().enumerate() {
        if let Some(map) = maybe_map(&word, &sq.1) {
            for w in words.iter().skip(i+1) {
                // the first digit can't be a 0
                if map[w[0] as usize] == 0 {
                    continue;
                }

                // form number from letter to digit mapping
                let n2 = w.iter().map(|c| map[*c as usize]).fold(0, |a, x| 10*a + x as usize);
                let n = n2.sqrt();

                // if it is a perfect square store it
                if n * n == n2 {
                    if max == 0 { max = sq.0; }
                    if n2 > max { max = n2; }
                }
            }
        }
    }
    if max > 0 { Some(max) } else { None }
}

pub fn solve(path: &str) -> usize {
    let ana = read_data(path);
    let max_len = ana.iter().map(|ref v| v[0].len()).max().unwrap();
    let squares = build_squares(max_len);
    let mut max_square = 0;

    // test every list of anagrams
    for a in &ana {
        // test each square with same number of digits as there are letters
        for sq in &squares[a[0].len()] {
            if let Some(max) = try_mapping(a, sq) {
                if max > max_square {
                    max_square = max;
                }
            }
        }
    }

    max_square
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
    fn test_98() {
        let path = "data/p098_words.txt";
        let s = solve(path);
        assert_eq!(18769, s);
    }

    #[bench]
    fn bench_98(b: &mut Bencher) {
        let path = "data/p098_words.txt";
        b.iter(|| black_box(solve(path)));
    }
}

