// Arranged probability
//
// If a box contains twenty-one coloured discs, composed of fifteen blue discs and six red discs,
// and two discs were taken at random, it can be seen that the probability of taking two blue
// discs, P(BB) = (15/21)×(14/20) = 1/2.
//
// The next such arrangement, for which there is exactly 50% chance of taking two blue discs at
// random, is a box containing eighty-five blue discs and thirty-five red discs.
//
// By finding the first arrangement to contain over 10^12 = 1,000,000,000,000 discs in total,
// determine the number of blue discs that the box would contain.

// We are looking for the value b such that 2 b (b-1) = (r+b)(b-1+r),
// and smallest r+b with a+b > 10^12
// That means b = r + (1 + sqrt(8r² + 1)) / 2
// so r > 10^12 / (2+sqrt(2)) and there is a s such that s² - 8r² = 1
// This is Diophantine's / Pell's equation, solved in pb 66

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

pub fn solve() -> u64 {
    let nb = 1_000_000_000_000u64;

    // the fundamental solution to Pell's equation s² - 8r² = 1
    // is obviously s = 3, r = 1
    let s1 = 3;
    let r1 = 1;
    let mut s = s1;
    let mut r = r1;

    // test every derived solution to Pell's Equation
    loop {
        if s % 2 == 1 {  // b must be odd
            let b = r + (s + 1) / 2;
            if b + r > nb {
                return b;
            }
        }

        // next solution to Pell's equation
        let t = s * s1 + 8 * r * r1;
        r = s * r1 + r * s1;
        s = t;
    }
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("blue discs: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_100() {
        let s = solve();
        assert_eq!(756872327473, s);
    }

    #[bench]
    fn bench_100(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
