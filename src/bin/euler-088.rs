// Product-sum numbers
//
// A natural number, N, that can be written as the sum and product of a given set of at least two
// natural numbers, {a1, a2, ... , ak} is called a product-sum number: N = a1 + a2 + ... + ak = a1
// × a2 × ... × ak.
//
// For example, 6 = 1 + 2 + 3 = 1 × 2 × 3.
//
// For a given set of size, k, we shall call the smallest N with this property a minimal
// product-sum number. The minimal product-sum numbers for sets of size, k = 2, 3, 4, 5, and 6 are
// as follows.
//
// k=2: 4  = 2 × 2 = 2 + 2
// k=3: 6  = 1 × 2 × 3 = 1 + 2 + 3
// k=4: 8  = 1 × 1 × 2 × 4 = 1 + 1 + 2 + 4
// k=5: 8  = 1 × 1 × 2 × 2 × 2 = 1 + 1 + 2 + 2 + 2
// k=6: 12 = 1 × 1 × 1 × 1 × 2 × 6 = 1 + 1 + 1 + 1 + 2 + 6
//
// Hence for 2≤k≤6, the sum of all the minimal product-sum numbers is 4+6+8+12 = 30; note that 8 is
// only counted once in the sum.
//
// In fact, as the complete set of minimal product-sum numbers for 2≤k≤12 is {4, 6, 8, 12, 15, 16},
// the sum is 61.
//
// What is the sum of all the minimal product-sum numbers for 2≤k≤12000?

// We can easily find an upper bound for numbers to test for.
// For any fixed k, let's call n = k-2, this is the sum of k-2 ones.
// A product-sum number for k is: n + a + b = a * b, which can also be
// written a = (b + n) / (b - 1)
// - if n (and k) is pair, then b = 2 and a = n + 2 = k is a solution
// - if n (and k) is impair, then b = 3 and a = (n+3)/2 = (k+1)/2 is a solution
// The conclusion is that there always is a solution and that it is never more than 2k

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

// calculate prime factors of all the numbers less than n
fn prime_factors(n: usize) -> Vec<Vec<usize>> {
    let mut f = vec![Vec::new(); n];
    for i in 2..n {
        // if i is prime
        if f[i].is_empty() {
            // add it as prime factor to all the multiples of i
            // and multiples of powers of i
            for p in (0..).scan(1, |a, _| {*a *= i; Some(*a)}) {
                if p > n { break; }
                for k in (p..n).step_by(p) {
                    f[k].push(i);
                }
            }
        }
    }

    f
}

// Build unique "partitions" of the supplied set
// We don't keep individual elements in each partition's group, only the product,
// which is equivalent to generating every list of all the dividers associations
// whose product is set.product().
fn make_partitions(set: &[usize]) -> Vec<Vec<usize>> {
    if set.is_empty() { return Vec::new(); }

    let mut parts = vec![vec![set[0]]];
    for &e in set[1..].iter() {
        let mut tmp = Vec::with_capacity(parts.len() * 2);
        for part in &parts {
            // new partition with e apart
            let mut p = part.clone();
            p.push(e);
            tmp.push(p);

            for i in 0..part.len() {
                // when a number appears more than once, we make sure to avoid
                // generating duplicates
                if i > 0 && part[i] == part[i-1] { continue; }
                let mut p = part.clone();
                p[i] *= e;
                tmp.push(p);
            }
        }
        parts = tmp;
    }

    parts
}

fn test_product_sum(num: usize, prod: usize, sum: usize, mut a: &mut [usize]) {
    let t = num + prod - sum;

    if t < 12000 && (a[t] == 0 || prod < a[t]) {
        a[t] = prod;
    }

    for i in 2.. {
        if i * prod > 24_000 { break; }
        test_product_sum(num + 1, prod * i, sum + i, &mut a);
    }
}

fn solve_brute() -> usize {
    let mut a = vec![0; 12_001];
    test_product_sum(0,1,0, &mut a);

    a[1] = 0;
    a.sort();
    a.dedup();
    a.iter().sum()
}

// We adopt a reverse strategy to decompose numbers N from 2 to 2 * n that act as
// product-sum numbers, and find out which k they can be assigned to.
// In order to do that, we must find all the product partitions of N (lists of
// dividers whose product equals N). This gives us lists of numbers whose sum
// we can then fill with ones to equate N. From the number of ones added we deduct
// k and assign N to it.
pub fn solve() -> usize {
    let n = 12_001;
    let mut v = vec![0; n];
    let facs = prime_factors(2 * n);
    let mut count = n - 1;

    'outer: for p in 2..2*n {
        for part in make_partitions(&facs[p]) {
            let c = part.len();
            let s: usize = part.iter().sum();
            if s > p { continue; }
            let k = c + p - s;
            if k < n && v[k] == 0 {
                v[k] = p;
                count -= 1;
                if count == 0 {
                    break 'outer;
                }
            }
        }
    }

    v[1] = 0; // count from 2
    v.sort();
    v.dedup();
    v.iter().sum()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("best area {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_brute();
    println!("best area {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_88() {
        let s = solve();
        assert_eq!(7587457, s);
    }

    #[bench]
    fn bench_88(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

