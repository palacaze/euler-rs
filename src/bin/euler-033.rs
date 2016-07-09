// Digit cancelling fractions
//
// The fraction 49/98 is a curious fraction, as an inexperienced mathematician in attempting to simplify
// it may incorrectly believe that 49/98 = 4/8, which is correct, is obtained by cancelling the 9s.
//
// We shall consider fractions like, 30/50 = 3/5, to be trivial examples.
//
// There are exactly four non-trivial examples of this type of fraction, less than one in value,
// and containing two digits in the numerator and denominator.
//
// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.

fn cancelling_digits(n: usize, d: usize) -> bool {
    let n1 = n % 10;
    let n2 = n / 10;
    let d1 = d % 10;
    let d2 = d / 10;

    if n1 == d1 && n2 * d == d2 * n && n1 != 0 { return true; }
    if n1 == d2 && n2 * d == d1 * n && n1 != 0 { return true; }
    if n2 == d2 && n1 * d == d1 * n && n2 != 0 { return true; }
    if n2 == d1 && n1 * d == d2 * n && n2 != 0 { return true; }
    false
}

// euclid gcd
fn gcd(a: usize, b: usize) -> usize {
    if a == b     { a }
    else if a > b { gcd(a - b, b) }
    else          { gcd(a, b - a) }
}

fn simplify(n: usize, d: usize) -> (usize, usize) {
    let mut mn = n;
    let mut md = d;

    loop {
        let r = gcd(mn, md);
        if r == 1 {
            return (mn, md);
        }
        mn /= r;
        md /= r;
    }
}

fn main() {
    let mut pn = 1;
    let mut pd = 1;

    for n in 10..99 {
        for d in (n+1)..100 {
            if cancelling_digits(n, d) {
                pn *= n;
                pd *= d;
                println!("{} / {}", n, d);
            }
        }
    }

    println!("prod = {:?}", simplify(pn, pd));
}
