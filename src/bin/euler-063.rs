// Powerful digit counts
//
// The 5-digit number, 16807=75, is also a fifth power. Similarly, the 9-digit
// number, 134217728=89, is a ninth power.
//
// How many n-digit positive integers exist which are also an nth power?

// a number x has n digits if 10^(n-1) ≤ x < 10^n
// we also want n = a^n, so it is obvious that a < 10, and we need to count
// the number of digits of a^n for 1 ≤ a < 10.
// what is the limit to look for n ? we handled the upper limit, there is
// also the lower limit, so in our case we will hit 10^(n-1) ≤ a^n,
// for each a from 1 to 9 we have our second condition
// that gives the second condition n ≤ 1.0 / (1.0 - log10(a))
// we can thus give the answer mathematically with no checking

pub fn solve() -> usize {
    (1..10)
        .map(|a| (1.0 / (1.0 - (a as f64).log10())) as usize)
        .fold(0, |a, c| a + c)
}

fn main() {
    let s = solve();
    println!("number: {:?}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_63() {
        let s = solve();
        assert_eq!(49, s);
    }
}
