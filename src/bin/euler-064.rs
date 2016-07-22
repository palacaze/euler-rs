// Odd period square roots
//
// All square roots are periodic when written as continued fractions and can
// be written in the form:
//                     1
// √N = a0 + ———————————————————
//                       1
//           a1 +  ——————————————
//   	  	                 1
//                 a2 +  ————————
//                       a3 + ...
//
// For example, let us consider √23:
//
// √23 = 4 + √23 — 4 = 4 +  1    = 4 +       1
//                        —————        ———————————
//                          1              √23 – 3
//                        —————        1 + ———————
//                        √23—4              7
//
//
// If we continue we would get the following expansion:
// √23 = 4 +          1
//           ————————————————————
//           1 +        1
//               ————————————————
//   	  	      3 +      1
//                   ————————————
//                    1 +    1
//                        ———————
//                        8 + ...
//
// The process can be summarised as follows:
// a0 = 4, 1 / (√23—4) =  (√23+4) / 7  = 1 + (√23—3) / 7
// a1 = 1, 7 / (√23—3) = 7(√23+3) / 14 = 3 + (√23—3) / 2
// a2 = 3, 2 / (√23—3) = 2(√23+3) / 14 = 1 + (√23—4) / 7
// a3 = 1, 7 / (√23—4) = 7(√23+4) / 7  = 8 + (√23—4)
// a4 = 8, 1 / (√23—4) =  (√23+4) / 7  = 1 + (√23—3) / 7
// a5 = 1, 7 / (√23—3) = 7(√23+3) / 14 = 3 + (√23—3) / 2
// a6 = 3, 2 / (√23—3) = 2(√23+3) / 14 = 1 + (√23—4) / 7
// a7 = 1, 7 / (√23—4) = 7(√23+4) / 7  = 8 + (√23—4)
//
// It can be seen that the sequence is repeating. For conciseness, we use the
// notation √23 = [4;(1,3,1,8)], to indicate that the block (1,3,1,8) repeats
// indefinitely.
//
// The first ten continued fraction representations of (irrational) square
// roots are:
//
// √2=[1;(2)], period=1
// √3=[1;(1,2)], period=2
// √5=[2;(4)], period=1
// √6=[2;(2,4)], period=2
// √7=[2;(1,1,1,4)], period=4
// √8=[2;(1,4)], period=2
// √10=[3;(6)], period=1
// √11=[3;(3,6)], period=2
// √12= [3;(2,6)], period=2
// √13=[3;(1,1,1,1,6)], period=5
//
// Exactly four continued fractions, for N ≤ 13, have an odd period.
//
// How many continued fractions for N ≤ 10000 have an odd period?


#![feature(test)]
extern crate test;

extern crate euler;
use euler::int::{Sqrt,Parity};

// Frac represents the denominator of our continued fraction at step i,
// with form: a + (√s - n) / d,
// s being the number whose square root we are factorizing.
// As a side note, we can prove that this form is always applicable:
// when getting the next iteration, we get the form g (√s + n) / h,
// for which g will always divide h.
#[derive(Clone, Copy, Default, PartialEq)]
struct Frac {
    a: u64, // ai coefficient
    n: u64, // numerator of fraction i
    d: u64, // integral part of denominator
}

impl Frac {
    fn new(a: u64, n: u64, d: u64) -> Self {
        Frac { a: a, n: n, d: d }
    }
}

fn factorize(s: u64) -> Vec<Frac> {
    let r = s.sqrt();
    if r * r == s {
        return Vec::new();
    }

    let mut f = Frac::new(r, r, 1);
    let mut v = Vec::new();

    loop {
        // we know that s - f.n² is divisible by f.d, no truncation here
        let d = (s - f.n * f.n) / f.d;
        let a = (r + f.n) / d;
        let n = a * d - f.n;
        f = Frac::new(a, n, d);

        // we stop once we find a cycle
        if !v.is_empty() && v[0] == f {
            break;
        }

        v.push(f);
    }

    v
}

pub fn solve() -> usize {
    (1..10001).map(factorize).filter(|v| v.len().is_odd()).count()
}

fn main() {
    let s = solve();
    println!("odd cycles: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_64() {
        let s = solve();
        assert_eq!(1322, s);
    }

    #[bench]
    fn bench_64(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
