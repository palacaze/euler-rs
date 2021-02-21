// Special subset sums: testing
//
// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if
// for any two non-empty disjoint subsets, B and C, the following properties are true:
//
//     S(B) ≠ S(C); that is, sums of subsets cannot be equal.
//     If B contains more elements than C then S(B) > S(C).
//
// For example, {81, 88, 75, 42, 87, 84, 86, 65} is not a special sum set because 65 + 87 + 88 = 75
// + 81 + 84, whereas {157, 150, 164, 119, 79, 159, 161, 139, 158} satisfies both rules for all
// possible subset pair combinations and S(A) = 1286.
//
// Using sets.txt (right click and "Save Link/Target As..."), a 4K text file with one-hundred sets
// containing seven to twelve elements (the two examples given above are the first two sets in the
// file), identify all the special sum sets, A1, A2, ..., Ak, and find the value of S(A1) + S(A2) +
// ... + S(Ak).
//
// NOTE: This problem is related to Problem 103 and Problem 106.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;
use std::ops::AddAssign;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn read_data(path: &str) -> impl Iterator<Item=Vec<usize>> {
    let path = Path::new(&path);
    BufReader::new(File::open(&path).unwrap())
        .lines()
        .map(|s| {
            let mut v = s.unwrap()
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            v.sort();
            v
        })
}

struct CombinationSums<'a, T> where T: 'a {
    set: &'a [T],
    n: usize,
    idx: Vec<usize>,
    max: Vec<usize>,
}

impl<'a, T> CombinationSums<'a, T> where T: AddAssign + Default + Copy {
    // iterate over combinations n elements in set and return the sum of elements
    fn new(set: &'a [T], n: usize) -> Self {
        assert!(set.len() >= n);
        let mut idx = (0..n).collect::<Vec<_>>();
        idx[n-1] -= 1;
        let max = (set.len()-n..set.len()).collect::<Vec<_>>();
        CombinationSums { set: set, n: n, idx: idx, max: max }
    }

    fn next(&mut self, sum: &mut T) -> bool {
        // end of iteration
        if self.idx[0] >= self.max[0] {
            return false;
        }

        // next element
        let mut c = self.n - 1;
        while self.idx[c] == self.max[c] {
            c -= 1;
        }
        self.idx[c] += 1;
        for i in c+1..self.n {
            self.idx[i] = self.idx[i-1] + 1;
        }

        // store new combination
        *sum = T::default();
        for i in 0..self.n {
            *sum += self.set[self.idx[i]];
        }

        true
    }
}

// Determine whether set is a special sum set
// Assuming rule 2, we can simplify rule 1 statement to "each subset sum is unique"
// if subset sums of size x are unique, this is also the case for subsets of size x-1,
// so we only need to test for size n/2
fn is_special_set(set: &[usize]) -> bool {
    let len = set.len();

    // first test rule 2 for worst case scenarii
    for i in 1..(len+1)/2 {
        if set[0..i+1].iter().sum::<usize>() <= set[len-i..].iter().sum::<usize>() {
            return false;
        }
    }

    // rule 1, we test that for every couple of disjoint subsets S1 and S2
    // of equal length, sum(S1) != sum(S2)
    if len <= 2 { return true; }
    if len == 3 { return set[2] != set[0] + set[1]; }
    if len == 4 { return set[0] + set[3] != set[1] + set[2]; }

    // cache to check unicity
    let mut c = vec![false; set[len-1] * (len/2)];

    // check that every subset sum is unique
    let mut comb = CombinationSums::new(set, len/2);
    let mut sum = 0;

    while comb.next(&mut sum) {
        if c[sum] { return false; }
        c[sum] = true;
    }

    return true;
}

pub fn solve(path: &str) -> usize {
    read_data(path)
        .filter(|x| is_special_set(x))
        .fold(0, |a, s| a + s.iter().sum::<usize>())
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");
    let start = PreciseTime::now();
    let s = solve(&path);
    println!("special sum sets sum: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::CombinationSums;
    use test::{Bencher, black_box};

    #[test]
    fn test_combi_105() {
        let set = &[0, 1, 2, 3, 4];
        let mut comb = CombinationSums::new(set, 3);
        let mut r = Vec::new();
        let mut s = 0;
        while comb.next(&mut s) {
            r.push(s);
        }

        assert_eq!(&r, &[3, 4, 5, 5, 6, 7, 6, 7, 8, 9]);
    }

    #[test]
    fn test_105() {
        let path = "data/p105_sets.txt";
        let s = solve(path);
        assert_eq!(73702, s);
    }

    #[bench]
    fn bench_105(b: &mut Bencher) {
        let path = "data/p105_sets.txt";
        b.iter(|| black_box(solve(path)));
    }
}

