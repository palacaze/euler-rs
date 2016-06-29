// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a² + b² = c²
//
// For example, 32 + 42 = 9 + 16 = 25 = 52.
//
// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.

// let n = 1000, simple fiddering with both equations shows that:
//
// The smallest edge, a, may not be more than n / 3 in order to satisfy both
// a < b < c and the equality a + b + c = n, so we only need to test a = 1..n/3
// b may also not be more than n/2

fn main() {
    let n = 1000;
    for a in 1..n/3+1 {
        for b in a+1..n/2 {
            let c = n - a - b;
            if a*a + b*b == c*c {
                println!("(a,b,c), prod = {}, {}, {}, {}", a, b, c, a*b*c);
                break;
            }
        }
    }
}
