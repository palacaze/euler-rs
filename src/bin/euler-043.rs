// Sub-string divisibility
//
// The number, 1406357289, is a 0 to 9 pandigital number because it is made up of each of the digits
// 0 to 9 in some order, but it also has a rather interesting sub-string divisibility property.
//
// Let d1 be the 1st digit, d2 be the 2nd digit, and so on. In this way, we note the following:
//
//     d2d3d4=406 is divisible by 2
//     d3d4d5=063 is divisible by 3
//     d4d5d6=635 is divisible by 5
//     d5d6d7=357 is divisible by 7
//     d6d7d8=572 is divisible by 11
//     d7d8d9=728 is divisible by 13
//     d8d9d10=289 is divisible by 17
//
// Find the sum of all 0 to 9 pandigital numbers with this property.

#![feature(test)]
extern crate test;

extern crate itertools;
use itertools::Itertools;

use std::collections::BTreeSet;

#[derive(Debug, Clone)]
struct DigitMapping {
    used: Vec<usize>,
    avail: BTreeSet<usize>,
}

impl DigitMapping {
    // take the digit from avail and put it in used
    fn add(&mut self, digit: usize) {
        self.used.push(digit);
        self.avail.remove(&digit);
    }

    // add remaining avail digits to used
    fn finish(&mut self) {
        self.used.extend(self.avail.iter());
    }
}

// convert a number to a list of digits.
// The digits are stored in reverse order
// We assume 3 digits, as per the needs of the problem
fn number_to_digits(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut q = n;
    while q != 0 {
        v.push(q % 10);
        q /= 10;
    }

    while v.len() < 3 {
        v.push(0);
    }

    v
}

// reverse conversion
fn digits_to_number(v: &[usize]) -> usize {
    v.iter().rev().fold(0, |a, d| 10 * a + d)
}

// here we assume 3 digits
fn unique_digits(v: &[usize]) -> bool {
    assert_eq!(v.len(), 3);
    v[0] != v[1] && v[0] != v[2] && v[1] != v[2]
}

// recurse over the divisibility list in order to compose and filter
// numbers that respect the rules
fn ensure_divisibility(divisors: &[usize], mappings: &[DigitMapping]) -> Vec<DigitMapping> {
    if divisors.is_empty() {
        // assign last digit
        return mappings.iter().map(|m| {
            let mut m2 = m.clone();
            m2.finish();
            m2
        }).collect::<Vec<_>>();
    }

    let mut v = Vec::new();
    // we need to add a digit from the 'avail' set of digits of every mapping
    // to the end of the 'used' list, so that the last 3 digits of set form
    // a number divisible by dividors[0]
    for m in mappings {
        for d in &m.avail {
            let n = digits_to_number(&[m.used[m.used.len()-2], m.used[m.used.len()-1], *d]);
            if n % divisors[0] == 0 {
                let mut nm = m.clone();
                nm.add(*d);
                v.push(nm);
            }
        }
    }

    ensure_divisibility(&divisors[1..], &v)
}

// we will move backward from the end in order to satisfy all the criteria
pub fn solve() -> usize {
    let set = (0..10).collect::<BTreeSet<_>>();

    // first the set of 3 digit numbers divisible by 17
    let v = (17..1000).step(17).map(number_to_digits)
                               .filter(|x| unique_digits(x))
                               .map(|v| {
                                   let mut s = set.clone();
                                   for d in &v { s.remove(d); }
                                   DigitMapping { used: v.clone(), avail: s }
                               })
                               .collect::<Vec<_>>();

    let m = ensure_divisibility(&[13, 11, 7, 5, 3, 2], &v).iter().map(|m| digits_to_number(&m.used)).collect::<Vec<_>>();
    // println!("pandigital divisible sub-strings = {:?}", m);

    // sum
    m.iter().fold(0, |a,c| a+c)
}

fn main() {
    let sum = solve();
    println!("sum = {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_pb() {
        assert_eq!(16695334890, solve());
    }

    #[bench]
    fn bench_pb(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

