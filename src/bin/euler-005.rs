// Smallest multiple
//
// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use std::collections::HashMap;

// prime number decomposition
fn prime_decomp(n : usize) -> HashMap<usize, u32> {
    let mut m = n;
    let mut h = HashMap::new();

    while m != 1 {
        for i in 2..m+1 {
            if m % i == 0 {
                let e = h.entry(i as usize).or_insert(0);
                *e += 1;
                m /= i;
                break;
            }
        }
    }
    h
}


fn main() {
    let nb = 20;
    let mut h = HashMap::new();

    // We count the max number of primes for each entry
    for i in 1..nb+1 {
        let dec = prime_decomp(i);
        for (p, n) in dec {
            let e = h.entry(p).or_insert(n);
            if *e < n {
                *e = n;
            }
        }
    }

    // now we build our best solution as the product of the primes
    let r = h.iter().fold(1, |a, (p, n)| a * p.pow(*n));
    println!("res = {:?}", r);
}
