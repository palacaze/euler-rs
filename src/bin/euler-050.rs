// Consecutive prime sum
//
// The prime 41, can be written as the sum of six consecutive primes:
// 41 = 2 + 3 + 5 + 7 + 11 + 13
//
// This is the longest sum of consecutive primes that adds to a prime below one-hundred.
//
// The longest sum of consecutive primes below one-thousand that adds to a prime, contains 21 terms, and is equal to 953.
//
// Which prime, below one-million, can be written as the sum of the most consecutive primes?


#![feature(test)]
extern crate test;

extern crate euler;

#[macro_use]
extern crate itertools;
use itertools::Itertools;
use itertools::FoldWhile::{Continue, Done};


pub fn solve() -> (u64, u64, u64) {
    let nb = 1_000_000u64;
    let primes = euler::primes::generate_primes(nb);

    let mut best_count = 0;
    let mut best_pos = 0;
    let mut best_prime = 0;

    for p in &primes {
        for (i, start) in primes.iter().enumerate() {
            // the first term of the sum can't be more than the target
            // divided by our best count of terms yet
            if *start > p / (best_count + 1) {
                break;
            }

            let sum = primes.iter().skip(i).fold_while((0 ,0), |a, c| {
                if a.1 >= *p { Done(a) } else { Continue((a.0 + 1, a.1 + c)) }
            });

            if sum.1 == *p {
                if sum.0 > best_count {
                    best_count = sum.0;
                    best_pos = *start;
                    best_prime = *p;
                }

                // we found a successful sum, any other sum formed for
                // this prime will have less terms since we start from
                // a higher number, no need to contine
                break;
            }
        }
    }

    (best_pos, best_count, best_prime)
}

fn main() {
    let b = solve();
    println!("best count: {} starting from prime {}, sum prime: {}", b.1, b.0, b.2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_50() {
        let s = solve();
        assert_eq!(997651, s.2);
    }
}

