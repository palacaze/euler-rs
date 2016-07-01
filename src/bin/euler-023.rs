// Non-abundant sums
//
// A perfect number is a number for which the sum of its proper divisors is exactly equal to the number.
// For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that
// 28 is a perfect number.
//
// A number n is called deficient if the sum of its proper divisors is less than n and it is called
// abundant if this sum exceeds n.
//
// As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be
// written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that
// all integers greater than 28123 can be written as the sum of two abundant numbers. However, this
// upper limit cannot be reduced any further by analysis even though it is known that the greatest
// number that cannot be expressed as the sum of two abundant numbers is less than this limit.
//
// Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

// here we now that if d is a divisor of n, then n/d is also one
// that mean we can search d up to sqrt(n), and add its n/d counterpart
// finally we add 1 and possibly sqrt(n)
fn sum_divisors(n : usize) -> usize {
    if n == 1 {
        return 1;
    }

    let mut s = (n as f64).sqrt() as usize;
    let square = s * s == n;
    if !square { s += 1 }

    let sum = (2..s).filter(|x| n % x == 0).fold(0, |a, d| a + d + n / d);
    1 + sum + if square { s } else { 0 }
}

fn is_abundant(n : usize) -> bool {
    sum_divisors(n) > n
}

fn is_sum_abundants(n : usize, abundants : &Vec<bool>) -> bool {
    for j in 1..(n/2+1) {
        // -1 because 'abundants' indexes from 1 instead of 0
        if abundants[j-1] && abundants[n-j-1] {
            return true;
        }
    }
    false
}

fn main() {
    let lim = 28124;
    let abundants : Vec<bool> = (1..lim).map(is_abundant).collect();
    let sum = (1..lim).filter(|i| !is_sum_abundants(*i, &abundants)).fold(0, |a, c| a+c);
    println!("sum = {:?}", sum);
}
