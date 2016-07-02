// Pandigital products
//
// We shall say that an n-digit number is pandigital if it makes use of all the digits 1 to n exactly once;
// for example, the 5-digit number, 15234, is 1 through 5 pandigital.
//
// The product 7254 is unusual, as the identity, 39 × 186 = 7254, containing multiplicand, multiplier,
// and product is 1 through 9 pandigital.
//
// Find the sum of all products whose multiplicand/multiplier/product identity can be written
// as a 1 through 9 pandigital.
// HINT: Some products can be obtained in more than one way so be sure to only include it once in your sum.

// first let's try to limit the scope of the search.
// We need exactly 9 digits, we notice that:
// * The product of 2 2-digit numbers can't produce a 5-digit number -> less than 9 digits
// * The product of 2 3-digit numbers produces a number of a least 5 digits -> more than 9 digits
// * The product of a 2-digit number and a 3-digit number can work
// * the product of a 1-digit number and a 4-digit number can also work.
// * other ranges are useless
// we will cycle a = 102..987 and b = 10..89, reducing at each outer loop the scope of the inner
// loop

#[inline]
fn check_digits(n: usize, account: &mut [bool]) {
    let mut q = n;
    while q != 0 {
        account[q % 10] = true;
        q /= 10;
    }
}

#[inline]
fn is_pandigital(a: usize, b: usize, p: usize) -> bool {
    let mut counter = vec![false; 10];
    check_digits(a, &mut counter);
    check_digits(b, &mut counter);
    check_digits(p, &mut counter);
    return counter.iter().skip(1).all(|&x| x);
}

fn search_range(a1: usize, a2: usize, prod_digits: u32, pan: &mut Vec<usize>) {
    for a in a1..(a2+1) {
        let mi = 10usize.pow(prod_digits-1) / a;    // we want the product to respect the number of digits
        let ma = 10usize.pow(prod_digits) / a + 1;  // which gives us both limits.
        for b in mi..ma {
            let p = a * b;
            if is_pandigital(a, b, p) {
                pan.push(p);
            }
        }
    }
}

fn main() {
    let mut pan = Vec::new();
    search_range(102, 987, 4, &mut pan); // 3d-2d product, with a 4-digits result
    search_range(1, 9, 4, &mut pan);     // 1d-4d product, with a 4-digits result
    
    pan.sort();
    pan.dedup();
    let sum = pan.iter().fold(0, |a, &p| a + p);
    println!("sum = {}", sum);
}
