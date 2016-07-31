// Coin partitions
//
// Let p(n) represent the number of different ways in which n coins can be separated into piles.
// For example, five coins can be separated into piles in exactly seven different ways, so p(5)=7.
// OOOOO
// OOOO   O
// OOO   OO
// OOO   O   O
// OO   OO   O
// OO   O   O   O
// O   O   O   O   O
//
// Find the least value of n for which p(n) is divisible by one million.

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
// we test each numbers partition in turn, caching p(n) for better operation
pub fn solve() -> usize {
    let target = 1_000_000;
    let penta = gen_penta(100_000);  // enough pentagonal numbers
    let mut part = vec![1];

    for n in 1.. {
        let pn = penta.iter().take_while(|&&(k,_)| k <= n)
            .map(|&(k,s)| s * part[n-k])
            .fold(0, |a, c| (a + c) % target);  // sum with modulo to avoid ovreflow
        if pn % target == 0 {
            return n;
        }
        part.push(pn);
    }
    0
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("n: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_078() {
        let s = solve();
        assert_eq!(55374, s);
    }

    #[bench]
    fn bench_076(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

