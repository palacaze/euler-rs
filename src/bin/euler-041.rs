// Pandigital prime
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits
// 1 to n exactly once. For example, 2143 is a 4-digit pandigital and is also prime.
//
// What is the largest n-digit pandigital prime that exists?

// no need to try 8 or 9 digits, because divisibility rules show those are divisible by 3
// The best seems to be going backward from 987654321, we will test all digits permutations
// for primes

#[macro_use]
extern crate itertools;
use itertools::Itertools;


fn is_prime(n : &usize) -> bool {
    if *n == 1 { return false; }
    if *n == 2 { return true;  }

    let lim = (*n as f32).sqrt() as usize + 1;
    for x in (3..lim).step(2) {
        if *n % x == 0 {
            return false;
        }
    }
    true
}

// Generate all the numbers through digits permutation from supplied list of digits.
// The digit_set order is respected, ie if the digits in digit_set are sorted,
// the output vector will also be sorted
fn digit_permutations(digit_set: &[usize]) -> Vec<usize> {
    let len = digit_set.len();
    if len == 1 { return digit_set.to_vec(); }

    let mut v = Vec::new();

    for i in 0..len {
        let mut s = digit_set.to_vec();
        let f = digit_set[i] * 10usize.pow(len as u32 -1);
        s.remove(i);
        let mut p = digit_permutations(&s);
        for e in &mut p {
            *e += f;
        }

        v.extend(p)
    }
    v
}

fn main() {
    // we test from biggest number of digits down
    for i in (2..8).rev() {
        // we create a reversed ordered list of digits in order to iterate
        // permutations also from the biggest down.
        // That way we know that the first encountered prime will be our answer
        let digits = (1..(i+1)).rev().collect::<Vec<_>>();
        for p in digit_permutations(&digits) {
            if is_prime(&p) {
                println!("max = {:?}", p);
                return;
            }
        }
    }
}
