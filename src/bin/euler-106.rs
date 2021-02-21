// Special subset sums: meta-testing
//
// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if
// for any two non-empty disjoint subsets, B and C, the following properties are true:
//
//     S(B) ≠ S(C); that is, sums of subsets cannot be equal.
//     If B contains more elements than C then S(B) > S(C).
//
// For this problem we shall assume that a given set contains n strictly increasing elements and it
// already satisfies the second rule.
//
// Surprisingly, out of the 25 possible subset pairs that can be obtained from a set for which n =
// 4, only 1 of these pairs need to be tested for equality (first rule). Similarly, when n = 7,
// only 70 out of the 966 subset pairs need to be tested.
//
// For n = 12, how many of the 261625 subset pairs that can be obtained need to be tested for
// equality?
//
// NOTE: This problem is related to Problem 103 and Problem 105.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

// set combinations builder
struct Combinations<'a, T>
    where T: 'a
{
    set: &'a [T],
    n: usize,
    idx: Vec<usize>,
    max: Vec<usize>,
}

impl<'a, T> Combinations<'a, T>
    where T: Default + Copy
{
    fn new(set: &'a [T], n: usize) -> Self {
        let mut idx = (0..n).collect::<Vec<_>>();
        idx[n-1] -= 1;
        let max = (set.len()-n..set.len()).collect::<Vec<_>>();
        Combinations {
            set: set,
            n: n,
            idx: idx,
            max: max,
        }
    }

    // iterate over combinations n elements in set and return current combination
    fn next(&mut self, data: &mut [T]) -> bool {
        let mut c = self.n - 1;

        // next element
        while self.idx[c] == self.max[c] {
            // end of iteration
            if c == 0 {
                return false;
            }
            c -= 1;
        }
        self.idx[c] += 1;
        for i in c+1..self.n {
            self.idx[i] = self.idx[i-1] + 1;
        }

        // store new combination
        for i in 0..self.n {
            data[i] = self.set[self.idx[i]];
        }

        true
    }
}

// count the number of subsets pairs of set size n that are unordered
fn count_unordered_subsets(set: &[usize], n: usize) -> usize {
    let mut v = vec![0; n];
    let mut v2 = vec![0; n];

    let mut comb = Combinations::new(&set, n);
    let mut count = 0;

    // iterate over subsets of size n
    while comb.next(&mut v) {
        v.sort();

        // complement of v
        let c = set.iter().cloned().filter(|i| !v.contains(i)).collect::<Vec<_>>();
        let mut comb2 = Combinations::new(&c, n);

        // iterate over disjoint subsets of v
        while comb2.next(&mut v2) {
            v2.sort();

            // if every element of v is > or < to those of v2, then the disjoint
            // subsets sums or also ordered, and thus don't need to be tested
            let ordered = v.iter().zip(v2.iter()).filter(|&(x, y)| x > y).count();
            if ordered != 0 && ordered != n {
                count += 1;
            }
        }
    }

    count / 2  // each subset is encountered twice
}

pub fn solve() -> usize {
    let n = 12;
    let set = (0..n).collect::<Vec<_>>();
    (2..n/2 + 1).map(|i| count_unordered_subsets(&set, i)).sum::<usize>()
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("special sum sets sum: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_106() {
        let s = solve();
        assert_eq!(21384, s);
    }

    #[bench]
    fn bench_106(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}
