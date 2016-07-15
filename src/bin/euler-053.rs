// Combinatoric selections
//
// There are exactly ten ways of selecting three from five, 12345:
//
// 123, 124, 125, 134, 135, 145, 234, 235, 245, and 345
//
// In combinatorics, we use the notation, 3C5 = 10.
//
// In general,
// nCr = n! / r!(n−r)!
// 	,where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.
//
// It is not until n = 23, that a value exceeds one-million: 23C10 = 1144066.
//
// How many, not necessarily distinct, values of  nCr, for 1 ≤ n ≤ 100, are greater than one-million?

// nCr is symmetric: nCr == nC(n-r), more over, for i = 0..(n/2), nCi < nC(i+1), so we know
// that for a given n, if fe find a first i for which nCi > 1_000_000, every i between i and n-i
// is > 1_000_000

#![feature(test)]
extern crate test;

// This formula to calculate the binomial is not entirely safe as it may
// overflow. The good news is that we are safe for this particular problem,
// because we go up to n = 100, and stop at binomials of 1_000_000, which
// happens below r = 9. The numerator would be smaller than 100^9, which
// means we never it 19 digits, which is the limit of u64.
fn binomial(n: u64, r: u64) -> u64 {
    (n-r+1..n+1).fold(1, |a,c| a*c) /
    (1..r+1).fold(1, |a,c| a*c)
}

pub fn solve() -> u64 {
    let nb = 1_000_000;
    let mut count = 0;

    // we can brute-solve this
    for n in 1..101 {
        for r in 1..n/2+1 {
            let b = binomial(n, r);
            if b > nb {
                count += n + 1 - 2 * r;
                break
            }
        }
    }

    count
}

fn main() {
    let s = solve();
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_53() {
        let s = solve();
        assert_eq!(4075, s);
    }

    #[bench]
    fn bench_53(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

