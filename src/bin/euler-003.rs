// Largest prime factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

extern crate euler;

fn main() {
    let nb = 600851475143u64;
    let factors = euler::primes::prime_factors(nb);
    println!("sum = {:?}", factors.iter().map(|&(x,_)| x).max().unwrap());
}
