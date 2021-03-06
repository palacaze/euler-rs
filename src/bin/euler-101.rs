// If we are presented with the first k terms of a sequence it is impossible to say with certainty
// the value of the next term, as there are infinitely many polynomial functions that can model the
// sequence.
//
// As an example, let us consider the sequence of cube numbers. This is defined by the generating
// function, un = n³: 1, 8, 27, 64, 125, 216, ...
//
// Suppose we were only given the first two terms of this sequence. Working on the principle that
// "simple is best" we should assume a linear relationship and predict the next term to be 15
// (common difference 7). Even if we were presented with the first three terms, by the same
// principle of simplicity, a quadratic relationship should be assumed.
//
// We shall define OP(k, n) to be the nth term of the optimum polynomial generating function for
// the first k terms of a sequence. It should be clear that OP(k, n) will accurately generate the
// terms of the sequence for n ≤ k, and potentially the first incorrect term (FIT) will be OP(k,
// k+1); in which case we shall call it a bad OP (BOP).
//
// As a basis, if we were only given the first term of sequence, it would be most sensible to
// assume constancy; that is, for n ≥ 2, OP(1, n) = u1.
//
// Hence we obtain the following OPs for the cubic sequence:
// OP(1, n) = 1 	        1, 1, 1, 1, ...
// OP(2, n) = 7n−6 	        1, 8, 15, ...
// OP(3, n) = 6n²−11n+6     1, 8, 27, 58, ...
// OP(4, n) = n³ 	        1, 8, 27, 64, 125, ...
//
// Clearly no BOPs exist for k ≥ 4.
//
// By considering the sum of FITs generated by the BOPs (indicated in red above), we obtain
// 1 + 15 + 58 = 74.
//
// Consider the following tenth degree polynomial generating function:
//
// u(n) = 1 − n + n² − n³ + n⁴ − n⁵ + n⁶ − n⁷ + n⁸ − n⁹ + n¹⁰
//
// Find the sum of FITs for the BOPs.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

fn u(n: usize) -> isize {
    let n = n as isize;
    let n2 = n * n;
    let n4 = n2 * n2;
    (1 - n) * (1 + (n2 + n4) * (1 + n4)) + n4 * n4 * n2
}

// a lagrange term, built as a rational number to avoid truncation
fn lagrange_term(xi: &[usize], j: usize, x: usize) -> (isize, isize) {
    (0..xi.len())
        .filter(|&i| i != j)
        .fold((1, 1), |a, i| (a.0 * (x as isize - xi[i]as isize),
                              a.1 * (xi[j]as isize - xi[i] as isize)))
}

// lagrange polynomial interpolated at points (xi, yi), evaluated at x
fn lagrange_interp(xi: &[usize], yi: &[isize], x: usize) -> isize {
    (0..xi.len()).fold(0, |a, i| {
        let lt = lagrange_term(xi, i, x);
        a + yi[i] * lt.0 / lt.1
    })
}

pub fn solve() -> isize {
    let x = (1..11).collect::<Vec<_>>();
    let y = x.iter().map(|&i| u(i)).collect::<Vec<_>>();
    x.iter().fold(0, |a, &i| a + lagrange_interp(&x[..i], &y[..i], i+1))

}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("sum of BOPs: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_101() {
        let s = solve();
        assert_eq!(37076114526, s);
    }

    #[bench]
    fn bench_101(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
