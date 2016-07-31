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
extern crate itertools;
use itertools::Itertools;
use time::PreciseTime;

fn gen_penta(n: usize) -> Vec<(usize, isize)> {
    let s = &[1, -1];
    let m = (1..).map(|i| i * (3*i-1) / 2).zip(s.iter().cycle());
    let p = (1..).map(|i| i * (3*i+1) / 2).zip(s.iter().cycle());
    m.interleave(p).take_while(|&(i,_)| i <= n).map(|(a,&b)| (a,b)).collect()
}

// euler partition formula, using pentagonal numbers
// p(n) = p(n-1) + p(n-2) - p(n-5) - p(n-7) +...
// p(n) = Σ (-1)^(k-1).p(n-gk)
//   where gk = k(3k-1)/2, k both negative and positive so that gk ≤ n
pub fn solve_partition() -> isize {
    let n = 100;
    let penta = gen_penta(n);
    let mut part = vec![0; n + 1];
    part[0] = 1;

    // add new numbers from 2 to n-1
    for i in 1..n+1 {
        for &p in penta.iter().take_while(|&&(k,_)| k <= i) {
            part[i] += p.1 * part[i-p.0];
        }
    }

    part[n] - 1     // -1 because the sum 100 + 0 is disallowed
}

// simple quadratic solution
pub fn solve() -> usize {
    let n = 100;
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
    let s = solve_partition();
    println!("count of sums: {} ({})", s, start.to(PreciseTime::now()));

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

    #[test]
    fn test_partition_076() {
        let s = solve_partition();
        assert_eq!(190569291, s);
    }

    #[bench]
    fn bench_076(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }

    #[bench]
    fn bench_partition_076(b: &mut Bencher) {
        b.iter(|| black_box(solve_partition()));
    }
}

