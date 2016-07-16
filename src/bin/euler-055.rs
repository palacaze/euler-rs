// Lychrel numbers
//
// If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.
//
// Not all numbers produce palindromes so quickly. For example,
//
// 349 + 943 = 1292,
// 1292 + 2921 = 4213
// 4213 + 3124 = 7337
//
// That is, 349 took three iterations to arrive at a palindrome.
//
// Although no one has proved it yet, it is thought that some numbers, like
// 196, never produce a palindrome. A number that never forms a palindrome
// through the reverse and add process is called a Lychrel number. Due to the
// theoretical nature of these numbers, and for the purpose of this problem,
// we shall assume that a number is Lychrel until proven otherwise. In
// addition you are given that for every number below ten-thousand, it will
// either (i) become a palindrome in less than fifty iterations, or, (ii) no
// one, with all the computing power that exists, has managed so far to map it
// to a palindrome. In fact, 10677 is the first number to be shown to require
// over fifty iterations before producing a palindrome:
// 4668731596684224866951378664 (53 iterations, 28-digits).
//
// Surprisingly, there are palindromic numbers that are themselves Lychrel
// numbers; the first example is 4994.
//
// How many Lychrel numbers are there below ten-thousand?
//
// NOTE: Wording was modified slightly on 24 April 2007 to emphasise the
// theoretical nature of Lychrel numbers.

#![feature(test)]
extern crate test;

fn digits(mut n: u64) -> Vec<u8> {
    let mut v = Vec::new();
    while n != 0 {
        v.push((n % 10) as u8);
        n /= 10;
    }
    v
}

fn is_palindromic(lst : &[u8]) -> bool {
    let l = lst.len();
    for i in 0..l/2 {
        if lst[i] != lst[l-1-i] {
            return false;
        }
    }
    true
}

// we reverse and add the digits by hand, we would overflow u64 otherwise
fn reverse_and_add(lst: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(lst.len() + 1);
    let mut carry = 0;

    for (a, b) in lst.iter().zip(lst.iter().rev()) {
        let d = a + b + carry;
        carry = d / 10;
        v.push((d % 10) as u8);
    }
    if carry > 0 {
        v.push(carry);
    }
    v
}

fn is_lychrel(n: &u64) -> bool {
    let mut v = reverse_and_add(&digits(*n));
    for _ in 1..50 {
        if is_palindromic(&v) {
            return false;
        }
        v = reverse_and_add(&v);
    }
    true
}

pub fn solve() -> usize {
    let nb = 10_000;
    (1..nb).filter(is_lychrel).count()
}

fn main() {
    let s = solve();
    println!("Lychrel numbers: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_55() {
        let s = solve();
        assert_eq!(249, s);
    }

    #[bench]
    fn bench_55(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

