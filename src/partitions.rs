use std::cmp;

pub struct Partition<'a, T: 'a> {
    n: usize,
    set: &'a [T],
    state: Vec<usize>,
    max: Vec<usize>,
}

impl<'a, T> Partition<'a, T> {
    /// create a new set partitions iterator for the given slice
    pub fn new(set: &'a [T]) -> Self {
        let n = set.len();
        Partition {
            n: n,
            set: set,
            state: vec![0; n],
            max: vec![0; n]
        }
    }
}

impl<'a, T> Iterator for Partition<'a, T> {
    type Item = Vec<Vec<&'a T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.n;
        if n == 0 { return None }

        // build current partition
        let mut part = vec![Vec::new(); self.max[n-1] + 2];
        for (i, &p) in self.state.iter().enumerate() {
            part[p].push(&self.set[i]);
        }

        // evaluate next partition
        let m = (0..n).rev().find(|&i| self.state[i] <= self.max[i]).unwrap();

        // this means we are finished
        if m == 0 {
            self.n = 0;
        }
        else {
            self.state[m] += 1;
            for i in m+1..n {
                self.state[i] = 0;
                self.max[i] = cmp::max(self.state[i-1], self.max[i-1]);
            }
        }

        if part[part.len()-1].is_empty() {
            part.pop();
        }
        Some(part)
    }
}

pub trait Partitions {
    type Item;

    /// creates a set partitions iterator
    fn partitions(&self) -> Partition<Self::Item>;
}

impl<T> Partitions for [T] {
    type Item = T;

    /// creates a set partitions iterator over the elements of the slice
    fn partitions(&self) -> Partition<Self::Item> {
        Partition::new(self)
    }
}

