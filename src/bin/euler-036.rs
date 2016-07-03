// Double-base palindromes
//
// The decimal number, 585 = 10010010012 (binary), is palindromic in both bases.
//
// Find the sum of all numbers, less than one million, which are palindromic in base 10 and base 2.
//
// (Please note that the palindromic number, in either base, may not include leading zeros.)

// palindromic and no leading 0 means odd number

#![feature(step_by)]

fn is_decimal_palindromic(n : usize) -> bool {
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

fn is_set(num: usize, digit: usize) -> bool {
    ((num >> digit) & 0x1) == 0x1
}

fn is_binary_palindromic(n: usize) -> bool {
    let l = (n.next_power_of_two() as f64).log2() as usize;
    (0..(l/2)).all(|i| is_set(n, i) == is_set(n, l-1-i))
}

fn is_palindromic(n: &usize) -> bool {
    is_binary_palindromic(*n) && is_decimal_palindromic(*n)
}

fn main() {
    let nb = 1_000_000;
    let sum = (1..nb).step_by(2).filter(is_palindromic).fold(0, |a,c| a+c);
    let las = (1..nb).step_by(2).filter(is_palindromic).collect::<Vec<_>>();
    println!("sum = {}, {:?}", sum, las);
}
