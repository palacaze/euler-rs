// Digit fifth powers
//
// Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:
//
//     1634 = 1⁴ + 6⁴ + 3⁴ + 4⁴
//     8208 = 8⁴ + 2⁴ + 0⁴ + 8⁴
//     9474 = 9⁴ + 4⁴ + 7⁴ + 4⁴
//
// As 1 = 14 is not a sum it is not included.
//
// The sum of these numbers is 1634 + 8208 + 9474 = 19316.
//
// Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.

// first we need to evaluate the max number of digits we want to try,
// the max value for a number with x digits is x * 9⁵ = x * 59049
// with 6 digits we get a number up to 999999 and 6 * 59049 = 354294
// from 7 digits, we can never match teh number with a sum because 7*9⁵ < 1.000.000
// We conclude that we can limit ourselves to the range [10-6.9^5]

fn sum_digit_power(n: u64, p: u32) -> u64 {
    let mut q = n;
    let mut sum = 0;
    while q != 0 {
        let r = q % 10;
        sum += r.pow(p);
        q /= 10;
    }
    sum
}

fn main() {
    let power : u32 = 5;
    let mi = 10;
    let ma = 6 * 9u64.pow(5) + 1;

    let elems = (mi..ma).map(|x| (x, sum_digit_power(x, power)))
                        .filter(|&(x, s)| x == s)
                        .map(|(_,s)| s).collect::<Vec<_>>();
                 
    let sum = elems.iter().fold(0, |a,c| a+c);

    println!("numbers: {:?}, sum = {:?}", elems, sum);
}
