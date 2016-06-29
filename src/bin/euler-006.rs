// The sum of the squares of the first ten natural numbers is,
// 12 + 22 + ... + 102 = 385
//
// The square of the sum of the first ten natural numbers is,
// (1 + 2 + ... + 10)2 = 552 = 3025
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    let nb = 100;
    let sum_square : u64 = (1..nb+1).fold(0, |a, x| a + x * x);
    let sum : u64 = (1..nb+1).fold(0, |a, x| a + x);
    println!("res = {:?}", sum * sum - sum_square);
}
