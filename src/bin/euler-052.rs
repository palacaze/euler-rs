// Permuted multiples
//
// It can be seen that the number, 125874, and its double, 251748,
// contain exactly the same digits, but in a different order.
//
// Find the smallest positive integer, x, such that 2x, 3x, 4x, 5x, and 6x,
// contain the same digits.

#![feature(test)]
extern crate test;

// with 4 digits, we need 3 bits per digit to account every digit,
// this is usable to compare permutations for numbers up to 9,999,999
fn count_digits(mut n: u64) -> u32 {
    let mut d: u32 = 0;
    while n != 0 {
        d += 1 << (3 * (n % 10));
        n /= 10;
    }
    d
}

pub fn solve() -> u64 {
    // same digits means that the first digit must be a one, as x/6 keeps
    // the same number of digits
    for i in 2..8 {
        let b = 10u64.pow(i);
        for j in 0..(b*2/3 + 1) {
            let n = b + j;
            let c = count_digits(n);
            if (2..7).map(|i| count_digits(i*n)).all(|x| x == c) {
                return n;
            }
        }
    }

    0
}

fn main() {
    let s = solve();
    println!("{} {} {} {} {} {}", s, 2*s, 3*s, 4*s, 5*s, 6*s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_52() {
        let s = solve();
        assert_eq!(142857, s);
    }

    #[bench]
    fn bench_52(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

