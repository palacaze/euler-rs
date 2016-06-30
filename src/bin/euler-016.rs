// Power digit sum
//
// 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
//
// What is the sum of the digits of the number 21000?

extern crate num;
use num::bigint::{BigUint, ToBigUint}; 

fn main() {
    let nb = 1000;
    let ten = 10.to_biguint().unwrap();

    let mut n = num::pow(2u64.to_biguint().unwrap(), nb);
    let mut s : BigUint = num::zero();
    while n != num::zero() {
        s = s + n.clone() % &ten;
        n = n.clone() / &ten;
    }

    println!("sum of digits for 2^{} = {}", nb, s);
}
