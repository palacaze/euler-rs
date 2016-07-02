// Coin sums
//
// In England the currency is made up of pound, £, and pence, p, and there are eight coins in general circulation:
//
//     1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).
//
// It is possible to make £2 in the following way:
//
//     1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p
//
// How many different ways can £2 be made using any number of coins?

static COINS: &'static [i32] = &[200, 100, 50, 20, 10, 5, 2, 1];

// number of ways of filling amount with avail coin types
fn fill_ways(amount: i32, avail: &[i32]) -> u32 {
    if avail.is_empty() { return 0; }
    let mut num = 0;
    for i in 0..(amount/avail[0] + 1) {
        let rem = amount - i * avail[0];
        num += if rem == 0 { 1 } else { fill_ways(rem, &avail[1..]) };
    }
    num
}

fn main() {
    let target: i32 = 200;
    let sum = fill_ways(target, &COINS);

    println!("sum = {}", sum);
}
