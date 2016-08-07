// Path sum: four ways
//
// NOTE: This problem is a significantly more challenging version of Problem 81.
//
// In the 5 by 5 matrix below, the minimal path sum from the top left to the bottom right, by
// moving left, right, up, and down, is indicated in bold red and is equal to 2297.
//
// ⎛ 131   673   234 — 103 —  18 ⎞
// ⎜  |           |           |  ⎟
// ⎜ 201 —  96 — 342   965   150 ⎟
// ⎜                          |  ⎟
// ⎜ 630   803   746   422 — 111 ⎟
// ⎜                    |        ⎟
// ⎜ 537   699   497   121   956 ⎟
// ⎜                    |        ⎟
// ⎝ 805   732   524    37 — 331 ⎠
//
// Find the minimal path sum, in matrix.txt (right click and "Save Link/Target As..."), a 31K text
// file containing a 80 by 80 matrix, from the left column to the right column.

#![feature(test)]
extern crate test;
extern crate time;

use time::PreciseTime;

use std::cmp;
use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Node {
    weight: u32,
    dist: u32,
    ok: bool,
    col: usize,
    row: usize,
}

trait Heap<T>
    where T: PartialOrd {
    fn push_heap(&mut self, e: T);
    fn remove_heap(&mut self, e: &T);
}

impl Node {
    fn new(w: u32, c: usize, r: usize) -> Self {
        Node { weight: w, dist: u32::max_value(), ok: false, col: c, row: r }
    }
}

impl cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<cmp::Ordering> {
        let o = other.dist.cmp(&self.dist);  // inverted for min weight
        if o == cmp::Ordering::Equal {
            let co = self.col.cmp(&other.col);
            if co == cmp::Ordering::Equal {
                Some(self.row.cmp(&other.row))
            }
            else { Some(co) }

        }
        else { Some(o) }
    }
}

impl cmp::Ord for Node {
    fn cmp(&self, other: &Node) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> Heap<Node> for Vec<Node> {
    fn push_heap(&mut self, e: Node) {
        match self.binary_search(&e) {
            Ok(i) => self.insert(i, e),
            Err(i) => self.insert(i, e)
        }
    }

    fn remove_heap(&mut self, e: &Node) {
        if let Ok(i) = self.binary_search(e) {
            self.remove(i);
        }
    }
}

fn read_data(path: &str) -> Vec<Vec<Node>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let len = data.lines().count();

    // we create a column-major matrix
    let mut mat = vec![vec![Node::default(); len]; len];
    for (r, line) in data.lines().enumerate() {
        for (c, w) in line.split(',').map(|c| c.parse::<u32>().unwrap()).enumerate() {
            mat[c][r] = Node::new(w, c, r);
        }
    }
    mat
}

fn visit<'a>(n: &Node, t: &mut Node, heap: &mut Vec<Node>) {
    if t.ok { return; } // already visited

    let d = n.dist + t.weight;
    if d < t.dist {
        t.dist = d;
        heap.remove_heap(&t);
        heap.push_heap(t.clone());
    }
}

// some sort of Dijkstra min distance algorithm
pub fn solve(path: &str) -> u32 {
    let mut mat = read_data(path);
    let n = mat.len();
    mat[0][0].dist = mat[0][0].weight;

    // fill a priority queue with every element
    let mut heap: Vec<Node> = Vec::with_capacity(n * n);
    for col in &mat {
        for e in col {
            heap.push_heap(e.clone());
        }
    }

    // we aggregate min distances by visiting pathes all the elements
    while !mat[n-1][n-1].ok {
        let e: Node = heap.pop().unwrap();
        if e.col > 0 { visit(&e, &mut mat[e.col-1][e.row], &mut heap); }
        if e.col < n-1 { visit(&e, &mut mat[e.col+1][e.row], &mut heap); }
        if e.row > 0 { visit(&e, &mut mat[e.col][e.row-1], &mut heap); }
        if e.row < n-1 { visit(&e, &mut mat[e.col][e.row+1], &mut heap); }
        mat[e.col][e.row].ok = true;
    }

    mat[n-1][n-1].dist
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let start = PreciseTime::now();
    let s = solve(&path);
    println!("min path: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_83() {
        let path = "data/p081_matrix.txt";
        let s = solve(&path);
        assert_eq!(425185, s);
    }

    #[bench]
    fn bench_83(b: &mut Bencher) {
        let path = "data/p081_matrix.txt";
        b.iter(|| black_box(solve(&path)));
    }
}

