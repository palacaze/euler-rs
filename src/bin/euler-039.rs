// Integer right triangles
//
// If p is the perimeter of a right angle triangle with integral length sides, {a,b,c},
// there are exactly three solutions for p = 120.
//
// {20,48,52}, {24,45,51}, {30,40,50}
//
// For which value of p ≤ 1000, is the number of solutions maximised?

// Without loss of generality, we can enforce a ≤ b ≤ c
// The smallest edge, a, may not be more than p / 3 in order to satisfy both
// the ordering and a + b + c = n, so we only need to test a = 1..n/3
// more over, we can show that b must not exceed bmax = n(n-2a) / 2(n-a).
// furthermore, as a >= bmax, that means that also calculate a better
// upper bound for a, with the equation a = bmax, which gives us
// amax = n * (1-sqrt(2) / 2) = 0.292 n, which is slightly better than n/3

fn square_root(n: usize) -> Option<usize> {
    let s = (n as f64).sqrt() as usize;
    if s * s == n { return Some(s); }
    if (s+1) * (s+1) == n { Some(s+1) } else { None }
}

fn main() {
    let n = 1000;
    let mut count = vec![0; n+1];
    let amax = (n as f32 * (1.0 - 2f32.sqrt() / 2.0)) as usize;

    for a in 2..amax {
        let bmax = n * (n - 2*a) / (2 * (n - a)) + 1;
        for b in a..bmax {
            if let Some(c) = square_root(a*a + b*b) {
                let sum = a + b + c;
                if c < b || sum > n { break; }
                count[sum] += 1;
            }
        }
    }
    
    let best = count.iter().enumerate().map(|(x,y)| (y,x)).max().unwrap();
    println!("max = {} for p = {}", best.0, best.1);
}
