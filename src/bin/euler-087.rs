// Prime power triples
//
// The smallest number expressible as the sum of a prime square, prime cube, and prime fourth power is 28. In fact, there are exactly four numbers below fifty that can be expressed in such a way:
//
// 28 = 22 + 23 + 24
// 33 = 32 + 23 + 24
// 49 = 52 + 23 + 24
// 47 = 22 + 33 + 24
//
// How many numbers below fifty million can be expressed as the sum of a prime square, prime cube, and prime fourth power?

#![feature(test)]
extern crate test;
extern crate time;
extern crate primal;

use time::PreciseTime;

pub fn solve() -> usize {
    let n = 50_000_000;
    let primes = primal::Primes::all().take_while(|&p| p*p < n).collect::<Vec<_>>();
    let primes2 = primes.iter().map(|i| i*i).filter(|i| *i < n).collect::<Vec<_>>();
    let primes3 = primes.iter().map(|i| i*i*i).filter(|i| *i < n).collect::<Vec<_>>();
    let primes4 = primes2.iter().map(|i| i*i).filter(|i| *i < n).collect::<Vec<_>>();
    let mut count = vec![false; n];

    for i in &primes2 {
        for j in &primes3 {
            if i+j >= n { break; }
            for k in &primes4 {
                let t = i+j+k;
                if t >= n { break; }
                count[t] = true;
            }
        }
    }

    count.iter().filter(|&x| *x).count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("prime power triples: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_87() {
        let s = solve();
        assert_eq!(1097343, s);
    }

    #[bench]
    fn bench_87(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

