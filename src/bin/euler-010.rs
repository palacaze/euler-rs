// Summation of primes
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

extern crate euler;

fn main() {
    let nb = 2_000_000u64;

    // prime numbers less than nb;
    let primes = euler::primes::Primes::new();
    let r = primes.take_while(|x| x < &nb).fold(0u64, |a, x| a + x);

    println!("sum = {:?}", r);
}
