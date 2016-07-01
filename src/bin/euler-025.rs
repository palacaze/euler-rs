// The Fibonacci sequence is defined by the recurrence relation:
//
//     Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
//
// Hence the first 12 terms will be:
//
//     F1 = 1
//     F2 = 1
//     F3 = 2
//     F4 = 3
//     F5 = 5
//     F6 = 8
//     F7 = 13
//     F8 = 21
//     F9 = 34
//     F10 = 55
//     F11 = 89
//     F12 = 144
//
// The 12th term, F12, is the first term to contain three digits.
//
// What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

extern crate num;
use num::bigint::{BigUint, ToBigUint}; 

#[derive(Debug)]
struct FibCounter {
    a : BigUint,
    b : BigUint,
}

impl FibCounter {
    fn new() -> FibCounter {
        FibCounter { a : num::one(), b : num::one() }
    }
}

impl Iterator for FibCounter {
    type Item = BigUint;
    fn next(&mut self) -> Option<Self::Item> {
        let c = &self.a + &self.b;
        self.a = self.b.clone();
        self.b = c;
        Some(self.a.clone())
    }
}

fn main() {
    let fib = FibCounter::new();
    let lim = num::pow(10u64.to_biguint().unwrap(), 999);
    let iter = fib.take_while(|x| x < &lim).count();

    // + 2 because we didn't step over F(0), and iter stop 1 before our goal
    println!("iters= {:?}", iter + 2); 
}
