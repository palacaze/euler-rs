// Power digit sum
//
// 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 21000?

extern crate num;
use num::bigint::{BigUint, ToBigUint}; 

fn sum_digits(n : &BigUint) -> u32 {
    n.to_string().chars().map(|c| c.to_digit(10).unwrap()).fold(0, |a, d| a + d)
}

fn main() {
    let nb = 1000;
    let n = num::pow(2u64.to_biguint().unwrap(), nb);
    let d = sum_digits(&n);
    println!("{}", d);
}
