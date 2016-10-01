// Prime summations
//
// It is possible to write ten as the sum of primes in exactly five different ways:
//
// 7 + 3
// 5 + 5
// 5 + 3 + 2
// 3 + 3 + 2 + 2
// 2 + 2 + 2 + 2 + 2
//
// What is the first value which can be written as the sum of primes in over
// five thousand different ways?

#![feature(test)]
extern crate test;
extern crate time;
extern crate primal;
use time::PreciseTime;

fn solve_upto(n: usize, target: usize) -> Option<usize> {
    // means of filling from 1 to n, using ones... none
    let mut count = vec![0; n + 1];

    // add new numbers from 2 to n-1
    for i in primal::Primes::all().take_while(|&i| i <= n) {
        count[i] += 1;
        for j in i+1..(n+1) {
            count[j] += count[j - i];
        }
    }

    count.iter().position(|&x| x > target)
}

pub fn solve() -> usize {
    let target = 5_000;
    let mut n = 100;
    // we don't know the range to span, start from 100 and
    // increase if we don't get an answer
    loop {
        if let Some(c) = solve_upto(n, target) {
            return c;
        }

        n *= 2;
    }
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
    fn test_077() {
        let s = solve();
        assert_eq!(71, s);
    }

    #[bench]
    fn bench_077(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

