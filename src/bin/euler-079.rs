// Passcode derivation
//
// A common security method used for online banking is to ask the user for three random characters
// from a passcode. For example, if the passcode was 531278, they may ask for the 2nd, 3rd, and 5th
// characters; the expected reply would be: 317.
//
// The text file, keylog.txt, contains fifty successful login attempts.
//
// Given that the three characters are always asked for in order, analyse the file so as to
// determine the shortest possible secret passcode of unknown length.

#![feature(test)]
extern crate test;
extern crate permutohedron;
extern crate time;
extern crate petgraph;

use permutohedron::Heap;
use time::PreciseTime;
use petgraph::Graph;

use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::collections::HashMap;

fn read_data(path: &str) -> Vec<Vec<char>> {
    let path = Path::new(&path);
    let mut file = File::open(&path).unwrap();
    let mut data = String::new();
    let _ = file.read_to_string(&mut data).unwrap();
    let mut v = data
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    v.sort();
    v.dedup();
    v
}

pub fn solve_graph(path: &str) -> String {
    let logs = read_data(path);

    // unique chars
    let mut chars = logs.iter().flat_map(|s| s.clone()).collect::<Vec<_>>();
    chars.sort();
    chars.dedup();

    // build a graph with unique chars as nodes
    let mut g = Graph::<char, char>::new();
    let nodes = chars.iter().map(|&c| (c, g.add_node(c))).collect::<HashMap<_, _>>();

    // build a list of directed edges
    let mut edges = logs.iter()
        .flat_map(|l| vec![(*nodes.get(&l[0]).unwrap(), *nodes.get(&l[1]).unwrap()),
                           (*nodes.get(&l[1]).unwrap(), *nodes.get(&l[2]).unwrap())])
        .collect::<Vec<_>>();
    edges.sort();
    edges.dedup();

    // set edges and apply topological sorting to the graph
    g.extend_with_edges(&edges);
    petgraph::algo::toposort(&g).iter()
        .map(|n| *g.node_weight(*n).unwrap())
        .collect()
}

fn match_order(sample: &[char], key: &[char]) -> bool {
    let mut k = 0;
    for &e in sample {
        while k < key.len() && key[k] != e {
            k += 1;
        }
        if k >= key.len() {
            return false;
        }
    }
    true
}

pub fn solve(path: &str) -> String {
    let logs = read_data(path);
    let mut chars = logs.iter().flat_map(|s| s.clone()).collect::<Vec<_>>();
    chars.sort();
    chars.dedup();

    'next_key: for key in Heap::new(&mut chars) {
        for log in &logs {
            if !match_order(&log, &key) { continue 'next_key; }
        }

        return key.into_iter().collect();
    }

    String::new()
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let start = PreciseTime::now();
    let s = solve(&path);
    println!("password: {} ({})", s, start.to(PreciseTime::now()));

    let start = PreciseTime::now();
    let s = solve_graph(&path);
    println!("password (graph): {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_79() {
        let path = "data/p079_keylog.txt";
        let s = solve(&path);
        assert_eq!("73162890", s);
    }

    #[test]
    fn test_graph_79() {
        let path = "data/p079_keylog.txt";
        let s = solve_graph(&path);
        assert_eq!("73162890", s);
    }

    #[bench]
    fn bench_79(b: &mut Bencher) {
        let path = "data/p079_keylog.txt";
        b.iter(|| black_box(solve(&path)));
    }

    #[bench]
    fn bench_graph_79(b: &mut Bencher) {
        let path = "data/p079_keylog.txt";
        b.iter(|| black_box(solve_graph(&path)));
    }
}

