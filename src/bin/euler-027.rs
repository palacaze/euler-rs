// Quadratic primes
//
// Euler discovered the remarkable quadratic formula:
//
// n² + n + 41
//
// It turns out that the formula will produce 40 primes for the consecutive values n = 0 to 39.
// However, when n = 40, 402 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 41² + 41 + 41 is clearly divisible by 41.
//
// The incredible formula  n² − 79n + 1601 was discovered, which produces 80 primes for the consecutive values n = 0 to 79.
// The product of the coefficients, −79 and 1601, is −126479.
//
// Considering quadratics of the form:
//
//     n² + an + b, where |a| < 1000 and |b| < 1000
//
//     where |n| is the modulus/absolute value of n
//     e.g. |11| = 11 and |−4| = 4
//
// Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number
// of primes for consecutive values of n, starting with n = 0.

extern crate euler;

fn count_primes(a: i64, b: i64) -> usize {
    (0..).take_while(|n| {
            let r = n * (n + a) + b;
            if r <= 0 { false } else { euler::primes::is_prime(r as u64) }
        }).count()
}

fn main() {
    let mut num_primes = 0;
    let mut ma = 0;
    let mut mb = 0;

    for a in (-1000 as i64)..1000 {
        for b in (-1000 as i64)..1000 {
            let n = count_primes(a, b);
            if n > num_primes {
                num_primes = n;
                ma = a;
                mb = b;
            }
        }
    }

    println!("longest is {} for (a, b) = ({}, {}, a.b = {})", num_primes, ma, mb, ma * mb);
}
