// Special subset sums: optimum Problem 103
//
// Let S(A) represent the sum of elements in set A of size n. We shall call it a special sum set if
// for any two non-empty disjoint subsets, B and C, the following properties are true:
//
//     S(B) ≠ S(C); that is, sums of subsets cannot be equal.
//     If B contains more elements than C then S(B) > S(C).
//
// If S(A) is minimised for a given n, we shall call it an optimum special sum set. The first five
// optimum special sum sets are given below.
//
// n = 1: {1}
// n = 2: {1, 2}
// n = 3: {2, 3, 4}
// n = 4: {3, 5, 6, 7}
// n = 5: {6, 9, 11, 12, 13}
//
// It seems that for a given optimum set, A = {a1, a2, ... , an}, the next optimum set is of the
// form B = {b, a1+b, a2+b, ... ,an+b}, where b is the "middle" element on the previous row.
//
// By applying this "rule" we would expect the optimum set for n = 6 to be A = {11, 17, 20, 22, 23,
// 24}, with S(A) = 117. However, this is not the optimum set, as we have merely applied an
// algorithm to provide a near optimum set. The optimum set for n = 6 is A = {11, 18, 19, 20, 22,
// 25}, with S(A) = 115 and corresponding set string: 111819202225.
//
// Given that A is an optimum special sum set for n = 7, find its set string.
//
// NOTE: This problem is related to Problem 105 and Problem 106.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

// max size of a set with n+1 elements
fn next_max(set: &[usize]) -> usize {
    let i = set.len() / 2 + (set.len() % 2 == 0) as usize;
    set.iter().sum::<usize>() + (set.len() + 1) * set[i]
}

// Validation of rule 1 for the set [set, e].
// At this point we already know that rule 2 is verified and that set is a
// special sum set, we only need to consider subsets containing e,
// with an equal number of elements 2 vs 2 and 3 vs 3. 1 vs 1 is obvious
fn is_special_set(set: &[usize], e: usize) -> bool {
    let len = set.len();

    if len < 2 {
        return true;
    }

    if len == 2 {
        return e != set[0] + set[1];
    }

    if len == 3 {
        return set[0] + e != set[1] + set[2];
    }

    for i1 in 0..len-2 {
        for i2 in i1+1..len-1 {
            for i3 in i2+1..len {
                let s1 = set[i1]+set[i3] - set[i2];
                let s2 = set[i2]+set[i3] - set[i1];
                if s1 == e || s2 == e { return false; }
                if s2 > e { break; }
            }
        }
    }

    if len < 5 {
        return true;
    }

    for (i1, &e1) in set[..len-4].iter().enumerate() {
        for (i2, &e2) in set[..len-3].iter().enumerate().skip(i1+1) {
            for (i3, &e3) in set[..len-2].iter().enumerate().skip(i2+1) {
                for (i4, &e4) in set[..len-1].iter().enumerate().skip(i3+1) {
                    for &e5 in set.iter().skip(i4+1) {
                        if e3+e4+e5 == e1+e2+e ||
                           e2+e3+e5 == e1+e3+e ||
                           e2+e3+e5 == e1+e4+e ||
                           e2+e3+e4 == e1+e5+e ||
                           e1+e4+e5 == e2+e3+e ||
                           e1+e3+e5 == e2+e4+e ||
                           e1+e3+e4 == e2+e5+e ||
                           e1+e2+e5 == e3+e4+e ||
                           e1+e2+e4 == e3+e5+e {
                               return false;
                        }
                    }
                }
            }
        }
    }

    return true;
}

// incrementaly build special sum sets, making use of the fact that sublists
// must also be special sum sets
fn find_special_set(n: usize, mut cur_min: &mut (usize, Vec<usize>)) {
    let mut set = vec![0; n];
    let mut max = vec![0; n];
    let mut p = 0;
    set[p] = n-2;
    max[p] = cur_min.0;

    loop {
        set[p] += 1;
        let sum = set[0..p+1].iter().sum::<usize>();

        if set[p] == max[p] || sum + set[p] * (n - p - 1) >= cur_min.0 {
            if p == 0 { return; }
            p -= 1;
            continue;
        }

        if is_special_set(&set[0..p], set[p]) {
            if p+1 == n {
                if sum < cur_min.0 {
                    cur_min.0 = sum;
                    cur_min.1 = set.clone();

                    // not really correct, we could meet better solutions
                    return;
                }

                // we won't find a better candidate
                p -= 1;
            }
            else {
                // min for next item
                let start = set[p] + 1;

                // max for next item, that let's us enforce rule 2
                let mut end = (cur_min.0 - sum) / (n - p - 1) + 1;
                if p > 0 {
                    let lim = set[0] + set[1];
                    if lim < end { end = lim; }
                }
                if p > 2 {
                    let lim = set[0] + set[1] + set[2] - set[p];
                    if lim < end { end = lim; }
                }
                if p > 4 {
                    let lim = set[0] + set[1] + set[2] + set[3] - set[p] - set[p-1];
                    if lim < end { end = lim; }
                }

                if end > start {
                    p += 1;
                    set[p] = set[p-1];
                    max[p] = end;
                }
            }
        }
    }
}

pub fn solve() -> String {
    let n = 7;
    let prev_opi = &[11, 18, 19, 20, 22, 25];
    let mut cur_min = (next_max(prev_opi) + 1, vec![0; n]);
    find_special_set(n, &mut cur_min);
    cur_min.1.iter().map(|i| i.to_string()).fold(String::new(), |a, s| a+&s)
}

// find out valid numbers that, combined with set, make up a new special sum set
// with one more element.
fn valid_next_in_set(set: &[usize], start: usize, end: usize) -> Vec<usize> {
    if set.len() < 3 {
        return (start..end).collect();
    }

    if set.len() == 3 {
        let rem = set[1] + set[2] - set[0];
        return (start..end).filter(|&i| i != rem).collect();
    }

    let mut rg = vec![false; end];

    {
        let mut mark = |x, y| if x >= start+y && x < y+end { rg[x-y] = true; };

        for (i1, &e1) in set.iter().enumerate() {
            for (i2, &e2) in set.iter().enumerate().skip(i1+1) {
                for &e3 in set.iter().skip(i2+1) {
                    mark(e1+e2, e3);
                    mark(e1+e3, e2);
                    mark(e2+e3, e1);
                }
            }
        }

        if set.len() >= 5 {
            for (i1, &e1) in set.iter().enumerate() {
                for (i2, &e2) in set.iter().enumerate().skip(i1+1) {
                    for (i3, &e3) in set.iter().enumerate().skip(i2+1) {
                        for (i4, &e4) in set.iter().enumerate().skip(i3+1) {
                            for &e5 in set.iter().skip(i4+1) {
                                mark(e3+e4+e5, e1+e2);
                                mark(e2+e3+e5, e1+e3);
                                mark(e2+e3+e5, e1+e4);
                                mark(e2+e3+e4, e1+e5);
                                mark(e1+e4+e5, e2+e3);
                                mark(e1+e3+e5, e2+e4);
                                mark(e1+e3+e4, e2+e5);
                                mark(e1+e2+e5, e3+e4);
                                mark(e1+e2+e4, e3+e5);
                                mark(e1+e2+e3, e4+e5);
                            }
                        }
                    }
                }
            }
        }
    }

    (start..end).filter(|&i| !rg[i]).collect()
}

// recursively find special sum sets, storing the minimum
fn find_special_set_list(n: usize, set: &[usize], mut cur_min: &mut (usize, Vec<usize>)) {
    let len = set.len();
    let sum = set.iter().sum::<usize>();
    let start = if len > 0 { set[len-1]+1 } else { n-1 };

    let mut end = (cur_min.0 - sum) / (n - len) + 1;
    if len > 1 {
        let lim = set[0] + set[1];
        if lim < end { end = lim; }
    }
    if len > 3 {
        let lim = set[0] + set[1] + set[2] - set[len-1];
        if lim < end { end = lim; }
    }
    if len > 5 {
        let lim = set[0] + set[1] + set[2] + set[3] - set[len-1] - set[len-2];
        if lim < end { end = lim; }
    }

    if end <= start { return; }

    // search a new number to add to the set
    for i in valid_next_in_set(set, start, end) {
        // stop searching if we can't improve the current optimal set
        if sum + i * (n - len) >= cur_min.0 { break; }

        let mut set_i = set.to_vec();
        set_i.push(i);

        // found required set length
        if set_i.len() == n {
            if sum + i < cur_min.0 {
                cur_min.0 = sum + i;
                cur_min.1 = set_i;
            }
            // we won't find a better candidate
            break;
        } else {
            // otherwise add new elements
            find_special_set_list(n, &set_i, &mut cur_min);
        }
    }
}

pub fn solve_list() -> String {
    let n = 7;
    let prev_opi = &[11, 18, 19, 20, 22, 25];
    let mut cur_min = (next_max(prev_opi) + 1, vec![0; n]);
    find_special_set_list(n, &[], &mut cur_min);

    cur_min.1.iter().map(|i| i.to_string()).fold(String::new(), |a, s| a+&s)
}

fn main() {
    let start = PreciseTime::now();
    let s = solve();
    println!("optimum set: {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_list();
    println!("optimum set: {} ({})", s, start.to(PreciseTime::now()));

}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_103() {
        let s = solve();
        assert_eq!("20313839404245", s);
    }

    #[bench]
    fn bench_103(b: &mut Bencher) {
        b.iter(|| black_box(solve()));
    }
}

