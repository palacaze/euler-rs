// 10001st prime
//
// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
//
// What is the 10 001st prime number?

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

// A prime numbers iterator
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
    let nb = 10001;

    // prime numbers less than sqrt(nb);
    let primes = PrimeCounter::new();
    let r : u64 = primes.take(nb).last().unwrap();

    println!("prime = {:?}", r);
}
