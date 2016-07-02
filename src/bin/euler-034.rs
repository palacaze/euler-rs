// Digit factorials
//
// 145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.
//
// Find the sum of all numbers which are equal to the sum of the factorial of their digits.
//
// Note: as 1! = 1 and 2! = 2 are not sums they are not included.

// 9! = 362880, we only need to inspect while num_digit * 9! > num
// with 6 digits we can represent 999.999 < 6*9! = 2.177.280
// with 7 digits we can go up to 9.999.999 > 7*9! = 2.540.160
// from 8 digits, we can't represent the number with sums of factorials, bacause
// the number is > 10e7 but we can't have a sum of more than ~3e6
// => we search in the range 3-7*9!

static FACS: &'static [usize] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

fn sum_fac_digits(n: usize) -> usize {
    let mut q = n;
    let mut sum = 0;
    while q != 0 {
        sum += FACS[q % 10];
        q /= 10;
    }
    sum
}

fn main() {
    let lim = 7 * FACS[9];
    let sum = (3..lim).filter(|x| *x == sum_fac_digits(*x)).fold(0, |a,c| a+c);
    println!("sum = {:?}", sum);
}
