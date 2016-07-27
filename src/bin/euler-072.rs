// Counting fractions
//
// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
// called a reduced proper fraction.
//
// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//
// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
// 6/7, 7/8
//
// It can be seen that there are 21 elements in this set.
//
// How many elements would be contained in the set of reduced proper fractions for d ≤ 1,000,000?

#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate primal;

extern crate rayon;
use rayon::prelude::*;

extern crate euler;
use euler::int::{Sqrt};

// calculate totients under n
fn totients(n: usize) -> Vec<usize> {
    let mut t = vec![0; n] ;
    for i in 2..n {

        // i is a prime
        if t[i] == 0 {
            t[i] = i - 1;
            for j in (2*i..n).step_by(i) {
                // initialize new entries
                if t[j] == 0 {
                    t[j] = j;
                }
                t[j] = t[j] * (i - 1) / i;
            }
        }
    }
    t
}

// calculate the totient using euler's formula
fn totient(i: usize, sieve: &primal::Sieve) -> usize {
    sieve.factor(i).unwrap().iter().fold(i, |a, &(p, _)| a * (p-1) / p)
}

pub fn solve_brute_par() -> usize {
    let nb = 1_000_001;
    let sieve = primal::Sieve::new(nb.sqrt()+1);
    (2..nb).into_par_iter().map(|i| totient(i, &sieve)).sum()
}

pub fn solve_sieve() -> usize {
    let nb = 1_000_001;
    totients(nb).into_iter().fold(0, |a, c| a + c)
}

fn main() {
    // 145 msec
    let s = solve_brute_par();
    println!("number of fractions: {:?}", s);

    // 35 msec
    let s = solve_sieve();
    println!("number of fractions: {:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_brute_072() {
        let s = solve_brute_par();
        assert_eq!(303963552391, s);
    }

    #[bench]
    fn bench_072(b: &mut Bencher) {
        b.iter(|| black_box(solve_sieve()));
    }
}

