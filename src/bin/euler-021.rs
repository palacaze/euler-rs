// Amicable numbers
//
// Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
// If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.
//
// For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284.
// The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.
//
// Evaluate the sum of all the amicable numbers under 10000.

#[derive(Debug, Copy, Clone)]
struct Num {
    amical : bool,
    sum : usize,
}

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

// we search all the dividors sums, stored in a vector and remove uniques
fn main() {
    let nb = 10000;
    let mut res = vec![Num { amical : false, sum : 0 } ; nb];

    for i in 1..nb {
        if res[i].sum > 0 {
            continue;
        }

        let d = sum_divisors(i);
        res[i].sum = d;

        if d < nb {
            if res[d].sum == 0 {
                res[d].sum = sum_divisors(d);
            }
            if res[d].sum == i && i != d {
                res[i].amical = true;
                res[d].amical = true;
            }
        }
    }

    let sum_amical = res.iter().enumerate().filter(|&(_, x)| x.amical).fold(0, |a, (i, _)| a + i);
    println!("sum amical numbers = {:?}", sum_amical);
}
