// Power digit sum
//
// 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 2^1000?

extern crate gmp;
use gmp::mpz::Mpz;

fn sum_digits(n : &Mpz) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).fold(0, |a, d| a + d)
}

fn main() {
    let nb = 1000;
    let n = Mpz::from(2).pow(nb);
    let d = sum_digits(&n);
    println!("{}", d);
}
