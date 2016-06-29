// Summation of primes
//
// The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//
// Find the sum of all the primes below two million.

#[derive(Debug)]
struct PrimeCounter {
    v : Vec<u64>,
}

impl PrimeCounter {
    fn new() -> PrimeCounter {
        PrimeCounter { v :  Vec::new() }
    }

    fn is_prime(& self, n : u64) -> bool {
        for x in &self.v {
            if n % x == 0 {
                return false;
            }
        }
        true
    }
}

// prime numbers iterator
impl Iterator for PrimeCounter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut n : Self::Item = match self.v.last() {
            Some(x) => x + 1,
            None => 2,
        };

        loop {
            if self.is_prime(n) {
                self.v.push(n);
                break;
            }
            n += 1;
        }
        Some(n)
    }
}

fn main() {
    let nb = 2_000_000u64;

    // prime numbers less than nb;
    let primes = PrimeCounter::new();
    let r = primes.take_while(|x| x < &nb).fold(0u64, |a, x| a + x);

    println!("sum = {:?}", r);
}
