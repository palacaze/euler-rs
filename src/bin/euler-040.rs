// Champernowne's constant
//
// An irrational decimal fraction is created by concatenating the positive integers:
//
// 0.123456789101112131415161718192021...
//
// It can be seen that the 12th digit of the fractional part is 1.
//
// If dn represents the nth digit of the fractional part, find the value of the following expression.
//
// d1 × d10 × d100 × d1000 × d10000 × d100000 × d1000000

// we count:
// 9   1-digit numbers,
// 90  2-digit numbers,
// 900 3 digit numbers,
//
// we count Nx = 9*10^(x-1) numbers with x digits, using up x*N digits once concatenated

fn extract_digit(n: usize, d: usize) -> usize {
    let mut m = n;
    for _ in 0..d {
        m /= 10;
    }
    m % 10
}

fn digit_at_rel_offset(offset: usize, order: usize) -> usize {
    let numbers_for_order = 9 * 10usize.pow(order as u32 - 1);
    let digits_for_order  = order * numbers_for_order;
    if offset <= digits_for_order {
        let number = 10usize.pow(order as u32 - 1) + (offset-1) / order;
        let digit  = (offset - 1) % order;
        extract_digit(number, order - digit - 1)
    }
    else {
        digit_at_rel_offset(offset - digits_for_order, order + 1)
    }
}

fn digit_at_offset(offset: usize) -> usize {
    return digit_at_rel_offset(offset, 1);
}

fn main() {
    let r = (0..7).map(|i| digit_at_offset(10usize.pow(i))).fold(1, |a,c| a*c);
    println!("product = {}", r);
}
