// Largest palindrome product
//
// A palindromic number reads the same both ways.
// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//
// Find the largest palindrome made from the product of two 3-digit numbers.

fn is_palindromic(n : u64) -> bool {
    let mut v : Vec<u8> = Vec::with_capacity(6);
    let mut m = n;
    while m != 0 {
        v.push((m % 10) as u8);
        m /= 10;
    }

    let l = v.len();
    for i in 0..l/2 {
        if v[i] != v[l-1-i] {
            return false;
        }
    }
    true
}

fn main() {
    let mut m : u64 = 0;
    let mut im : u64 = 0;
    let mut jm : u64 = 0;

    for i in 100..1000 {
        for j in i..1000 {
            let p = i * j;
            if is_palindromic(p) && p > m {
                m = p;
                im = i;
                jm = j;
            }
        }
    }

    println!("best = {} x {} = {}", im, jm, m);
}
