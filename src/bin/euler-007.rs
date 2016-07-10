// 10001st prime
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?

extern crate euler;

fn main() {
    let nb = 10001;

    let primes = euler::primes::Primes::new();
    let r : u64 = primes.take(nb).last().unwrap();

    println!("prime = {:?}", r);
}
