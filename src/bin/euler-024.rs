// Lexicographic permutations
//
// A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
// of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
// we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
//
// 012   021   102   120   201   210
//
// What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn factorial(n : usize) -> usize {
    (2..n+1).fold(1, |a, x| a * x)
}

fn main() {
    let mut nb : usize = 1_000_000 - 1;  // -1 because we are 0-based
    let mut digits = vec!['0','1','2','3','4','5','6','7','8','9'];

    // there are factorial(n) permutations of n digits. As we know the results to be ordered,
    // we calculate each digit in turn in order to satify this order.
    // There are fac(9) numbers beginning with a 0, fac(9) with a 1...
    // Surely our millionth permutation must begin with a 1e6 / fac(9) = 2
    // the remainder is the new order to satisfy for the remaining digits
    for i in 1..10 {
        let f = factorial(10-i);
        let d = nb / f;
        nb -= d * f;
        print!("{}", digits[d]);
        digits.remove(d);
    }
    
    print!("{}\n", digits[0]);
}
