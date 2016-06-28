// Largest prime factor
//
// The prime factors of 13195 are 5, 7, 13 and 29.
//
// What is the largest prime factor of the number 600851475143 ?

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
    let nb = 600851475143u64;
    let lim = (nb as f64).sqrt() as u64 + 1;

    // prime numbers less than sqrt(nb);
    let primes = PrimeCounter::new();
    let r : Vec<u64> = primes.take_while(|x| x <= &lim).filter(|x| nb % x == 0).collect();

    println!("sum = {:?}", r);
}
