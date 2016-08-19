// Amicable chains
//
// The proper divisors of a number are all the divisors excluding the number itself. For example,
// the proper divisors of 28 are 1, 2, 4, 7, and 14. As the sum of these divisors is equal to 28,
// we call it a perfect number.
//
// Interestingly the sum of the proper divisors of 220 is 284 and the sum of the proper divisors of
// 284 is 220, forming a chain of two numbers. For this reason, 220 and 284 are called an amicable
// pair.
//
// Perhaps less well known are longer chains. For example, starting with 12496, we form a chain of
// five numbers:
//
// 12496 → 14288 → 15472 → 14536 → 14264 (→ 12496 → ...)
//
// Since this chain returns to its starting point, it is called an amicable chain.
//
// Find the smallest member of the longest amicable chain with no element exceeding one million.

#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Chain {
    Unvisited,
    Invalid,
    Length(usize),
}

fn repeats(n: usize, v: &[usize]) -> Option<usize> {
    for (i, &e) in v.iter().enumerate() {
        if e == n { return Some(i); }
    }
    None
}

// calculate the sum of proper dividors in a range in one go
fn sum_divisors(n: usize) -> Vec<usize> {
    let mut sum = vec![1; n+1];
    for i in 2..n/2+1 {
        for j in (2*i..n+1).step_by(i) {
            sum[j] += i;
        }
    }
    sum
}

// mark all the chain as invalid
fn invalidate(lens: &mut [Chain], chain: &[usize]) {
    for &e in chain {
        lens[e] = Chain::Invalid;
    }
}

pub fn solve() -> usize {
    let nb = 1_000_000;
    let sums = sum_divisors(nb);

    let mut lens = vec![Chain::Unvisited; nb+1];
    let mut len_max = 0;

    // find the
    for i in 2..nb+1 {
        if lens[i] != Chain::Unvisited { continue; }

        let mut p = i;
        let mut chain = vec![p];

        loop {
            p = sums[p];

            // found an invalid element
            if p > nb {
                invalidate(&mut lens, &chain);
                break;
            }

            // we found a reapeating term
            if let Some(k) = repeats(p, &chain) {
                // elements before the beginning of the chain are invalid
                invalidate(&mut lens, &chain[..k]);

                // mark all the chain with the same length
                let len = chain.len() - k;
                for &e in &chain[k..] {
                    lens[e] = Chain::Length(len);
                }

                // remember max encountered length
                if len > len_max {
                    len_max = len;
                }

                break;
            }

            // test the element for status
            match lens[p] {
                Chain::Unvisited => chain.push(p),
                Chain::Invalid | Chain::Length(_) => {
                    invalidate(&mut lens, &chain);
                    break;
                },
            }
        }
    }

    lens.iter().take_while(|&c| *c != Chain::Length(len_max)).count()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("element of longest chain: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_95() {
        let s = solve();
        assert_eq!(14316, s);
    }

    #[bench]
    fn bench_95(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

