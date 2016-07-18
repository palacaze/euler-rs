// XOR decryption
//
// Each character on a computer is assigned a unique code and the preferred
// standard is ASCII (American Standard Code for Information Interchange). For
// example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.
//
// A modern encryption method is to take a text file, convert the bytes to
// ASCII, then XOR each byte with a given value, taken from a secret key. The
// advantage with the XOR function is that using the same encryption key on
// the cipher text, restores the plain text; for example, 65 XOR 42 = 107,
// then 107 XOR 42 = 65.
//
// For unbreakable encryption, the key is the same length as the plain text
// message, and the key is made up of random bytes. The user would keep the
// encrypted message and the encryption key in different locations, and
// without both "halves", it is impossible to decrypt the message.
//
// Unfortunately, this method is impractical for most users, so the modified
// method is to use a password as a key. If the password is shorter than the
// message, which is likely, the key is repeated cyclically throughout the
// message. The balance for this method is using a sufficiently long password
// key for security, but short enough to be memorable.
//
// Your task has been made easy, as the encryption key consists of three lower
// case characters. Using cipher.txt (right click and 'Save Link/Target
// As...'), a file containing the encrypted ASCII codes, and the knowledge
// that the plain text must contain common English words, decrypt the message
// and find the sum of the ASCII values in the original text.

#![feature(test)]
extern crate test;

extern crate itertools;
use itertools::Itertools;

extern crate permutohedron;
use permutohedron::Heap;

use std::str::FromStr;
use std::env;
use std::io::Read;
use std::error::Error;
use std::fs::File;
use std::path::Path;

const A: u8 = 'a' as u8;
const Z: u8 = 'z' as u8;
const SPACE: u8 = ' ' as u8;

fn decrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter().zip(key.iter().cycle()).map(|(d,k)| d^k).collect::<Vec<_>>()
}

fn read_data(path: &str) -> Vec<u8> {
    let path = Path::new(&path);
    let display = path.display();

    // open
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // read
    let mut data = String::new();
    if let Err(why) = file.read_to_string(&mut data) {
        panic!("couldn't read {}: {}", display, why.description());
    };

    // parsing
    data.lines().nth(0).unwrap().split(',').map(|c| u8::from_str(c).unwrap()).collect::<Vec<_>>()
}

pub fn solve_simpler(path: &str) -> (usize, String) {
    let data = read_data(path);

    // letters frequency analysis: in a 1200 letters text,
    // we expect at least 200 spaces, the most probable character
    // we step 3 times over the data with a stride to get in turn each letter of the password
    let mut key = Vec::new();
    for i in 0..3 {
        let l = (A..Z+1).map(|k| (data.iter().skip(i).step(3).map(|x| x^k).filter(|&c| c == SPACE).count(), k))
                        .sorted().last().unwrap().1;
        key.push(l);
    }

    let txt = String::from_utf8(decrypt(&data, &key)).unwrap();
    (txt.chars().fold(0, |a, c| a + c as usize), txt)
}

pub fn solve(path: &str) -> (usize, String) {
    let data = read_data(path);

    // letters frequency analysis: in a 1200 letters text,
    // we expect at least 200 spaces, the most probable character
    let mut occ = (A..Z+1).map(|k| (decrypt(&data, &[k]).iter().filter(|&c| *c == SPACE).count(), k))
                          .collect::<Vec<_>>();

    // keep 3 best rated letters
    occ.sort();
    let mut letters = occ.iter().rev().map(|&(_, c)| c)
                         .take(3).collect::<Vec<_>>();

    let mut num = 0;
    let mut txt = String::new();

    // words: we try every 3-letters password permutation and keep
    // the result with the most "the" occurences
    for key in Heap::new(&mut letters) {
        let d = decrypt(&data, &key);
        let t = String::from_utf8(d.clone()).unwrap();
        let n = t.matches("the").count();

        if n > num {
            num = n;
            txt = t;
        }
    }

    (txt.chars().fold(0, |a, c| a + c as usize), txt)
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let s = solve_simpler(&path);
    println!("text: {:?}", s.1);
    println!("key sum: {}", s.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_59() {
        let path = "data/p059_cipher.txt";
        let s = solve(&path);
        assert_eq!(107359, s.0);
    }

    #[test]
    fn test_simpler_59() {
        let path = "data/p059_cipher.txt";
        let s = solve_simpler(&path);
        assert_eq!(107359, s.0);
    }

    #[bench]
    fn bench_59(b: &mut Bencher) {
        let path = "data/p059_cipher.txt";
        b.iter(|| black_box(solve(&path)));
    }

    #[bench]
    fn bench_simpler_59(b: &mut Bencher) {
        let path = "data/p059_cipher.txt";
        b.iter(|| black_box(solve_simpler(&path)));
    }
}

