// Counting fractions in a range
//
// Consider the fraction, n/d, where n and d are positive integers. If n<d and HCF(n,d)=1, it is
// called a reduced proper fraction.
//
// If we list the set of reduced proper fractions for d ≤ 8 in ascending order of size, we get:
//
// 1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 5/8, 2/3, 5/7, 3/4, 4/5, 5/6,
// 6/7, 7/8
//
// It can be seen that there are 3 fractions between 1/3 and 1/2.
//
// How many fractions lie between 1/3 and 1/2 in the sorted set of reduced proper fractions for d ≤
// 12,000?

#![feature(step_by)]
#![feature(test)]
extern crate test;
extern crate primal;
extern crate euler;
extern crate time;
use time::PreciseTime;
use euler::int::{Gcd, Sqrt, Parity};

// LUT of 64 first primes
static PRIMES_TABLE: &'static [u32] = &[
    0,  0,  0,  1,  0,  2,  0,  3,  0,  0,  0,  4,  0,  5,  0,  0,  0,
    6,  0,  7,  0,  0,  0,  8,  0,  0,  0,  0,  0,  9,  0, 10,  0,  0,
    0,  0,  0, 11,  0,  0,  0, 12,  0, 13,  0,  0,  0, 14,  0,  0,  0,
    0,  0, 15,  0,  0,  0,  0,  0, 16,  0, 17,  0,  0,  0,  0,  0, 18,
    0,  0,  0, 19,  0, 20,  0,  0,  0,  0,  0, 21,  0,  0,  0, 22,  0,
    0,  0,  0,  0, 23,  0,  0,  0,  0,  0,  0,  0, 24,  0,  0,  0, 25,
    0, 26,  0,  0,  0, 27,  0, 28,  0,  0,  0, 29,  0,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0,  0, 30,  0,  0,  0, 31,  0,  0,  0,  0,
    0, 32,  0, 33,  0,  0,  0,  0,  0,  0,  0,  0,  0, 34,  0, 35,  0,
    0,  0,  0,  0, 36,  0,  0,  0,  0,  0, 37,  0,  0,  0, 38,  0,  0,
    0,  0,  0, 39,  0,  0,  0,  0,  0, 40,  0, 41,  0,  0,  0,  0,  0,
    0,  0,  0,  0, 42,  0, 43,  0,  0,  0, 44,  0, 45,  0,  0,  0,  0,
    0,  0,  0,  0,  0,  0,  0, 46,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0,  0, 47,  0,  0,  0, 48,  0, 49,  0,  0,  0, 50,  0,  0,  0,  0,
    0, 51,  0, 52,  0,  0,  0,  0,  0,  0,  0,  0,  0, 53,  0,  0,  0,
    0,  0, 54,  0,  0,  0,  0,  0, 55,  0,  0,  0,  0,  0, 56,  0, 57,
    0,  0,  0,  0,  0, 58,  0,  0,  0, 59,  0, 60,  0,  0,  0,  0,  0,
    0,  0,  0,  0, 61,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,  0,
    0, 62,  0,  0,  0, 63];

// calculate the totient of i, the brute-force way (before I searched on the internet)
// divs are the prime factors of i
fn count_coprimes_k(i: usize, k: usize, divs: &[usize]) -> usize {
    let mut count = 0;

    // count co-primes encountered below i
    for &j in divs {
        count += (i-1) / j / k;
    }

    // When a number is multiple of 2 prime factors, it has been counted twice, so
    // we remove duplicates. However, if it was multiples of 3 prime factors, we
    // just removed it 3 times so we must add it again...
    for c in 2..(divs.len()+1) {
        for m in divs.iter().combinations_n(c).map(|x| x.iter().fold(1, |a, &x| a*x)) {
            let num = (i-1) / m;
            if c.is_even() { count -= num / k; } else { count += num / k; }
        }
    }

    i / k - 1 - count
}

fn common_element(a: &[usize], b: &[usize]) -> bool {
    let mut j = 0;
    for &e in a {
        while j < b.len() && b[j] < e { j += 1; }
        if j >= b.len() { return false; }
        if b[j] == e { return true; }
    }
    false
}

#[derive(Clone, Default, Debug)]
struct Factors {
    small: u64,
    tail: Vec<usize>,
}

impl Factors {
    fn new(n: usize, sieve: &primal::Sieve) -> Self {
        let fac = sieve.factor(n).unwrap_or(Vec::new());
        let mut s = 0u64;
        let mut t = Vec::new();
        for &(p,_) in &fac {
            if p < PRIMES_TABLE.len() { s |= 1 << PRIMES_TABLE[p]; }
            else { t.push(p); }
        }
        Factors{ small: s, tail: t }
    }

    fn is_coprime(&self, other: &Factors) -> bool {
        (self.small & other.small) == 0 && (!common_element(&self.tail, &other.tail))
    }
}

pub fn solve_sieve() -> usize {
    let nb: usize = 12_001;
    let sieve = primal::Sieve::new(nb.sqrt() + 1);
    let facs = (0..nb).map(|i| Factors::new(i, &sieve)).collect::<Vec<_>>();

    let mut count = 0;
    for d in 2..nb {
        for n in (d/3+1)..(d+1)/2 {
            if facs[d].is_coprime(&facs[n]) {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_brute() -> usize {
    let nb: usize = 12_001;
    let mut count = 0;
    for d in 2..nb {
        let f2 = d % 2 == 0;
        let f3 = d % 3 == 0;
        for n in (d/3+1)..(d+1)/2 {
            if (f2 && n % 2 == 0) || (f3 && n % 3 == 0) { continue; }
            if n.gcd(d) == 1 {
                count += 1;
            }
        }
    }

    count
}

pub fn solve_totient() -> usize {
    let nb = 12_001;
    let sieve = primal::Sieve::new(nb.sqrt() + 1);
    let s = (2..nb)
        .map(|i| {
            let divs = sieve.factor(i).unwrap().iter().map(|&(p,_)| p).collect::<Vec<_>>();
            let c2 = count_coprimes_k(i, 2, &divs);
            let c3 = count_coprimes_k(i, 3, &divs);
            c2 - c3
        })
        .fold(0, |a, c| a + c);
    s - 1
}

fn main() {
    // 390 msec
    let start = PreciseTime::now();
    let s = solve_brute();
    println!("number of fractions brute: {:?} ({})", s, start.to(PreciseTime::now()));

    // 58 ms
    let start = PreciseTime::now();
    let s = solve_sieve();
    println!("number of fractions sieve: {:?} ({})", s, start.to(PreciseTime::now()));

    // 12 ms
    let start = PreciseTime::now();
    let s = solve_totient();
    println!("number of fractions totient: {:?} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_sieve_073() {
        let s = solve_sieve();
        assert_eq!(7295372, s);
    }

    #[test]
    fn test_totient_073() {
        let s = solve_totient();
        assert_eq!(7295372, s);
    }

    #[test]
    fn test_brute_073() {
        let s = solve_brute();
        assert_eq!(7295372, s);
    }

    #[bench]
    fn bench_073(b: &mut Bencher) {
        b.iter(|| black_box(solve_totient()));
    }
}

