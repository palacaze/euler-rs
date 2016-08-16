// Square digit chains
//
// A number chain is created by continuously adding the square of the digits in a number to form a
// new number until it has been seen before.
//
// For example,
//
// 44 → 32 → 13 → 10 → 1 → 1
// 85 → 89 → 145 → 42 → 20 → 4 → 16 → 37 → 58 → 89
//
// Therefore any chain that arrives at 1 or 89 will become stuck in an endless loop. What is most
// amazing is that EVERY starting number will eventually arrive at 1 or 89.
//
// How many starting numbers below ten million will arrive at 89?

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

fn square_digits(mut n: usize) -> usize {
    let mut s = 0;
    while n != 0 {
        let t = n / 10;
        let d = n - 10*t;
        s += d * d;
        n = t;
    }
    s
}

pub fn solve() -> usize {
    let nb = 10_000_000;
    let cache_size = ((nb as f64).log10() as usize + 1) * 9 * 9 + 1;
    let mut cache = vec![false; cache_size];

    // build cache
    for i in 2..cache_size {
        let mut n = i;
        loop {
            n = square_digits(n);
            if n == 1 { break; }
            if n == 89 {
                cache[i] = true;
                break;
            }
        }
    }
    
    // count
    (2..nb).map(square_digits).filter(|&i| cache[i]).count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("right triangles: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_92() {
        let s = solve();
        assert_eq!(8581146, s);
    }

    #[bench]
    fn bench_92(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

