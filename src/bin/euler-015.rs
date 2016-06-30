// Lattice paths
//
// Starting in the top left corner of a 2×2 grid, and only being able to move
// to the right and down, there are exactly 6 routes to the bottom right corner.
//
// How many such routes are there through a 20×20 grid?

fn factorial(n : u64) -> u64 {
    (2..n+1).fold(1, |a, x| a * x)
}

fn main() {
    let nb : u64 = 20;

    // that's simply the number of ways of positioning 20 elements in a grid of 40,
    // so Binomial(n, r) = n! / (r! (n-r)!) with n = 40 and r = 20

    // problem is, Rust does not have integers bigger than u64, whose max is 2e19.
    // factorial(40) is way bigger (8.e47). We can first elide the factorial(20)
    // from both sides of the division, which simplifies to (40*39*...*21) / factorial(20)
    // Then remark that all the even elements from the numerator can be eliminated,
    // which simplifies further to 39*37*...*21*2^10 / factorial(10)
    let num = ((nb/2)..nb).map(|x| 2*x+1).fold(1, |a, x| a*x) * 2u64.pow(10);
    println!("pathes = {}",  num / factorial(nb/2));
}
