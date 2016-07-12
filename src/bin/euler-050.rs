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


pub fn solve_incremental() -> (u64, u64, u64) {
    let nb = 1_000_000u64;

    // build initial list of cumulated sum of primes until nb
    let primes = euler::primes::Primes::new();
    let psum = primes.scan(0, |sum, p| {
                          let old = *sum;
                          *sum += p;
                          Some(old)
                     }).take_while(|x| *x < nb).collect::<Vec<_>>();

    let mut start = 0;
    let mut end = 0;

    // repeatedly search the first prime from the end of the list of the cumulated
    // sums, removing one prime from beginning of sum at each iteration
    for s in 0.. {
        // repeat until there is more removed elements from the beginning than we could add
        // from the end, meaning there is no hope to get a better result
        if psum.len() - end < s - start {
            break;
        }

        // substract the first element and search again for a sum with more terms
        let rem = psum[s];
        let e = psum.iter().skip(end).rev().take_while(|&x| !euler::primes::is_prime(*x - rem)).count();
        if e < psum.len() - end - 1 {
            end = psum.len() - e - 1;
            start = s;
        }
    }

    (psum[start+1]-psum[start] as u64, (end-start) as u64, psum[end] - psum[start])
}

pub fn solve_brute() -> (u64, u64, u64) {
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
    let b = solve_brute();
    println!("best count: {} starting from prime {}, sum prime: {}", b.1, b.0, b.2);

    let b = solve_incremental();
    println!("best count: {} starting from prime {}, sum prime: {}", b.1, b.0, b.2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    // too long
    #[test]
    #[ignore]
    fn test_brute_50() {
        let s = solve_brute();
        assert_eq!(997651, s.2);
    }

    #[test]
    fn test_incremental_50() {
        let s = solve_incremental();
        assert_eq!(997651, s.2);
    }

    #[bench]
    fn bench_incremental_50(b: &mut Bencher) {
        b.iter(|| black_box(solve_incremental()));
    }
}

