// Pentagon numbers
//
// Pentagonal numbers are generated by the formula, Pn=n(3n−1)/2. The first ten pentagonal numbers are:
//
// 1, 5, 12, 22, 35, 51, 70, 92, 117, 145, ...
//
// It can be seen that P4 + P7 = 22 + 70 = 92 = P8. However, their difference, 70 − 22 = 48, is not pentagonal.
//
// Find the pair of pentagonal numbers, Pj and Pk, for which their sum and difference are pentagonal
// and D = |Pk − Pj| is minimised; what is the value of D?

#![feature(test)]
extern crate test;

fn pentagon(n: usize) -> usize {
    n * (3 * n -1) / 2
}

fn nearest_pentagon_index(pn: usize) -> usize {
    let s = (1.0 + 24.0 * pn as f64).sqrt();
    ((1.0 + s) / 6.0) as usize
}

fn pentagon_index(pn: usize) -> Option<usize> {
    let n = nearest_pentagon_index(pn);
    if pentagon(n) == pn { Some(n) }
    else if pentagon(n+1) == n+1 { Some(n + 1) }
    else { None }
}

fn is_pentagon(pn: usize) -> bool {
    pentagon_index(pn).is_some()
}

pub fn solve() -> usize {
    // current value of pn - pk
    let mut diff = 10_000_000_000;  // big value as a starting point

    for n in 2.. {
        // We know that for any k, p_n - p_k >= p_n - p_n-1 >= 3n + 1
        // so if 3n+1 becomes bigger than our best diff yet, we know we
        // can't find a smaller diff and the problem is solved
        if 3 * n + 1 > diff {
            break;
        }

        let pn = pentagon(n);
        // we can impose a lower bound for k, as we want pn - pk < diff
        // we simply start from the first k ensuring this inequality
        let kmin = if diff >= pn {1} else { nearest_pentagon_index(pn-diff) };
        for k in kmin..n {
            let pk = pentagon(k);
            if is_pentagon(pn - pk) && is_pentagon(pn + pk) {
                diff = pn - pk;
            }
        }
    }
    diff
}

fn main() {
    let sum = solve();
    println!("min |Pn - Pk| = {:?}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_pb() {
        assert_eq!(5482660, solve());
    }

    #[bench]
    fn bench_pb(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

