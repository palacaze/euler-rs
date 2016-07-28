// Digit factorial chains Problem 74
//
// The number 145 is well known for the property that the sum of the factorial of its digits is
// equal to 145:
//
// 1! + 4! + 5! = 1 + 24 + 120 = 145
//
// Perhaps less well known is 169, in that it produces the longest chain of numbers that link back
// to 169; it turns out that there are only three such loops that exist:
//
// 169 → 363601 → 1454 → 169
// 871 → 45361 → 871
// 872 → 45362 → 872
//
// It is not difficult to prove that EVERY starting number will eventually get stuck in a loop. For
// example,
//
// 69 → 363600 → 1454 → 169 → 363601 (→ 1454)
// 78 → 45360 → 871 → 45361 (→ 871)
// 540 → 145 (→ 145)
//
// Starting with 69 produces a chain of five non-repeating terms, but the longest non-repeating
// chain with a starting number below one million is sixty terms.
//
// How many chains, with a starting number below one million, contain exactly sixty non-repeating
// terms?

#![feature(test)]
extern crate test;
extern crate time;
use time::PreciseTime;

// LUT of factorials
static FACTORIAL: &'static [usize] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn repeats(n: usize, v: &[usize]) -> Option<usize> {
    for (i, &e) in v.iter().enumerate() {
        if e == n { return Some(i); }
    }
    None
}

fn digits_fac_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n != 0 {
        let t = n / 10;
        sum += FACTORIAL[n - 10 * t];
        n = t;
    }
    sum
}

pub fn solve() -> usize {
    let nb = 1_000_000;
    let mut cache = vec![0; 7*FACTORIAL[9] + 1];

    for i in 0..nb {
        if cache[i] != 0 { continue; }
        let mut n = i;
        let mut v = Vec::new();
        loop {
            v.push(n);
            n = digits_fac_sum(n);

            // we found a reapeating term
            if let Some(j) = repeats(n, &v) {
                // store lengths of all encountered numbers
                for k in 0..j {
                    cache[v[k]] = v.len() - k;
                }
                // we also know that a loop has been discovered
                // and all the numbers in that loop have the same
                // length of non-repeating chain
                for k in j..v.len() {
                    cache[v[k]] = v.len() - j;
                }

                break;
            }
            // we found an already encountered number
            if cache[n] != 0 {
                for (k, &e) in v.iter().enumerate() {
                    cache[e] = v.len() - k + cache[n];
                }

                break;
            }
        }
    }

    cache.iter().filter(|&i| i == &60).count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("60 non-reapeating terms: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_sieve_074() {
        let s = solve();
        assert_eq!(402, s);
    }

    #[bench]
    fn bench_074(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

