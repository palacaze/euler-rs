// Factorial digit sum
//
// n! means n × (n − 1) × ... × 3 × 2 × 1
//
// For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
// and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.
//
// Find the sum of the digits in the number 100!

extern crate gmp;
use gmp::mpz::Mpz;

fn sum_digits(n : &Mpz) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).fold(0, |a, d| a + d)
}

fn main() {
    let nb = 100;
    let n = (1..nb + 1).fold(Mpz::one(), |a, x| a * Mpz::from(x));
    let d = sum_digits(&n);
    println!("{}", d);
}
