// Goldbach's other conjecture
//
// It was proposed by Christian Goldbach that every odd composite number can be written as the sum of a prime and twice a square.
//
// 9 = 7 + 2×1^2
// 15 = 7 + 2×2^2
// 21 = 3 + 2×3^2
// 25 = 7 + 2×3^2
// 27 = 19 + 2×2^2
// 33 = 31 + 2×1^2
//
// It turns out that the conjecture was false.
//
// What is the smallest odd composite that cannot be written as the sum of a prime and twice a square?

#![feature(step_by)]

#![feature(test)]
extern crate test;

fn is_prime_cached(n : usize, cache: &[usize]) -> bool {
    let lim = (n as f32).sqrt() as usize + 1;
    for p in cache.iter().skip(1) {
        if *p > lim { break; }
        if n % p == 0 { return false; }
    }
    true
}

pub fn solve() -> usize {
    let mut primes = vec![1, 2, 3, 5, 7];

    'outer: for n in (9..).step_by(2) {
        // n is either prime, in that case we store it or it is not
        if is_prime_cached(n, &primes) {
            primes.push(n);
            continue;
        }

        let sq_lim = (0.5 * n as f32).sqrt() as usize + 1;
        for r in 1..sq_lim {
            let p = n - 2 * r * r;
            if primes.binary_search(&p).is_ok() {
                continue 'outer;
            }
        }

        // if we get here that means we have our counter-example
        return n;
    }
    
    0
}

fn main() {
    let sum = solve();
    println!("Goldbach counter-example = {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_pb() {
        assert_eq!(5777, solve());
    }

    #[bench]
    fn bench_pb(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

