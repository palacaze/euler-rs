// Counting summations
//
// It is possible to write five as a sum in exactly six different ways:
//
// 4 + 1
// 3 + 2
// 3 + 1 + 1
// 2 + 2 + 1
// 2 + 1 + 1 + 1
// 1 + 1 + 1 + 1 + 1
//
// How many different ways can one hundred be written as a sum of at least
// two positive integers?

#![feature(test)]
extern crate test;
extern crate time;
use time::PreciseTime;

pub fn solve() -> usize {
    let n = 1000;
    // means of filling from 1 to n, using ones
    let mut count = vec![1; n + 1];

    // add new numbers from 2 to n-1
    for i in 2..n {
        count[i] += 1;
        for j in i+1..(n+1) {
            count[j] += count[j - i];
        }
    }

    count[n]
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("count of sums: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_076() {
        let s = solve();
        assert_eq!(190569291, s);
    }

    #[bench]
    fn bench_076(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

