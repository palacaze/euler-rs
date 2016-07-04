// Pandigital multiples
//
// Take the number 192 and multiply it by each of 1, 2, and 3:
//
//     192 × 1 = 192
//     192 × 2 = 384
//     192 × 3 = 576
//
// By concatenating each product we get the 1 to 9 pandigital, 192384576.
// We will call 192384576 the concatenated product of 192 and (1,2,3)
//
// The same can be achieved by starting with 9 and multiplying by 1, 2, 3, 4, and 5,
// giving the pandigital, 918273645, which is the concatenated product of 9 and (1,2,3,4,5).
//
// What is the largest 1 to 9 pandigital 9-digit number that can be formed as the
// concatenated product of an integer with (1,2, ... , n) where n > 1?

// let's limit the range of work by working out the number ranges that give
// exactly 9 digits for given n number
// n = 2: 5000 <= x < 10000
// n = 3:  100 <= x < 334
// n = 4:   25 <= x < 34
// n = 5:    5 <= x < 10
// n = 6:    3 <= x < 4
// n = 7: impossible

// account for digits in the number n
#[inline]
fn check_digits(n: usize, account: &mut [bool]) {
    let mut q = n;
    while q != 0 {
        account[q % 10] = true;
        q /= 10;
    }
}

// check whether the list of numbers v represents a pandigital set
#[inline]
fn is_pandigital(v: &[usize]) -> bool {
    let mut counter = vec![false; 10];
    for n in v {
        check_digits(*n, &mut counter);
    }
    return counter.iter().skip(1).all(|&x| x);
}

fn search_pandigital(rg: &(usize, usize, usize)) -> Vec<(String,usize,usize)> {
    let mut v = Vec::new();
    for x in rg.1..rg.2 {
        let p = (1..(rg.0+1)).map(|i| i*x).collect::<Vec<_>>();
        if is_pandigital(&p) {
            let num = p.iter().fold(String::new(), |a, x| a + &x.to_string());
            v.push((num, rg.0, x));
        }
    }
    v
}

fn main() {
    let rg = &[(2, 5000, 10000),
               (3,  100, 334  ),
               (4,   25, 34   ),
               (5,    5, 10   ),
               (6,    3, 4    )];
    
    let m = rg.iter().flat_map(search_pandigital).max();
    println!("max = {:?}", m);
}
