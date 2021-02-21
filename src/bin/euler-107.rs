// Minimal network
//
// The following undirected network consists of seven vertices and twelve edges with a total weight
// of 243.
//
// The same network can be represented by the matrix below.
//     	A	B	C	D	E	F	G
// A	-	16	12	21	-	-	-
// B	16	-	-	17	20	-	-
// C	12	-	-	28	-	31	-
// D	21	17	28	-	18	19	23
// E	-	20	-	18	-	-	11
// F	-	-	31	19	-	-	27
// G	-	-	-	23	11	27	-
//
// However, it is possible to optimise the network by removing some edges and still ensure that all
// points on the network remain connected. The network which achieves the maximum saving is shown
// below. It has a weight of 93, representing a saving of 243 − 93 = 150 from the original network.
//
// Using network.txt (right click and 'Save Link/Target As...'), a 6K text file containing a
// network with forty vertices, and given in matrix form, find the maximum saving which can be
// achieved by removing redundant edges whilst ensuring that the network remains connected.

#![feature(test)]
extern crate test;
extern crate time;
extern crate petgraph;

use time::PreciseTime;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type Vertex = usize;
type Weight = usize;
type Edge = (Weight, Vertex, Vertex);

pub fn read_data(path: &str) -> (usize, Vec<Edge>) {
    let path = Path::new(&path);
    let data = BufReader::new(File::open(&path).unwrap()).lines().flat_map(|x| x).collect::<Vec<_>>();

    let vertex_count = data.len();

    let edges = data.iter()
        .enumerate()
        .flat_map(|(i, s)| {
            s.split(',').enumerate()
             .take(i).filter_map(move |(j, x)| x.parse::<usize>().ok().map(|w| (w, i, j)))
        })
        .collect::<Vec<Edge>>();

    (vertex_count, edges)
}

fn count_weight(edges: &[Edge]) -> usize {
    edges.iter().map(|e| e.0).sum()
}

pub fn solve_prim(path: &str) -> usize {
    let (count, mut rem_edges) = read_data(path);
    let init_sum = count_weight(&rem_edges);

    // build list of vertices and edges of minimum spanning tree
    let mut rem_verts = vec![true; count];
    let mut mst_verts = vec![false; count];
    let mut mst_edges = Vec::with_capacity(count);

    // sort in order to have shortest pathes first
    rem_edges.sort();

    rem_verts[0] = false;
    mst_verts[0] = true;

    // build minimum spanning tree
    while !rem_verts.is_empty() {
        // find shortest edges linking from the set of nodes in mst and
        // remaining nodes and add it to the mst
        if let Some(e) = rem_edges.iter().filter_map(|&(w, v1, v2)| {
            if rem_verts[v1] && mst_verts[v2] {
                Some((w, v1, v2))
            }
            else if rem_verts[v2] && mst_verts[v1] {
                Some((w, v2, v1))
            }
            else { None }
        }).nth(0) {
            // transfer vertex
            rem_verts[e.1] = false;
            mst_verts[e.1] = true;
            mst_edges.push(e);

            // remove remaining useless edges --> no need, slows things up
            // rem_edges.retain(|e| rem_verts[e.1] || rem_verts[e.2]);
        }
        else {
            break;  // no new node to add, we have finished
        }
    }

    let final_sum = count_weight(&mst_edges);
    init_sum - final_sum
}

pub fn solve_graph(path: &str) -> usize {
    let (count, edges) = read_data(path);
    let init_sum = count_weight(&edges);

    // build a graph
    let mut g = petgraph::Graph::new_undirected();
    let nodes = (0..count).map(|c| g.add_node(c)).collect::<Vec<_>>();

    // add undirected edges
    for e in &edges {
        g.add_edge(nodes[e.1], nodes[e.2], e.0);
    }

    // calculate minimum spanning tree and sum remaining edges' weights
    let final_sum = petgraph::algo::min_spanning_tree(&g).raw_edges().iter().map(|e| e.weight).sum::<usize>();

    init_sum - final_sum
}

fn main() {
    let path: String = env::args().nth(1).expect("Must supply a file name");

    let start = PreciseTime::now();
    let s = solve_graph(&path);
    println!("tree saving: {} ({})", s, start.to(PreciseTime::now()));
    //
    let start = PreciseTime::now();
    let s = solve_prim(&path);
    println!("tree saving: {} ({})", s, start.to(PreciseTime::now()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn test_graph_107() {
        let path = "data/p107_network.txt";
        let s = solve_graph(path);
        assert_eq!(259679, s);
    }

    #[test]
    fn test_prim_107() {
        let path = "data/p107_network.txt";
        let s = solve_prim(path);
        assert_eq!(259679, s);
    }

    #[bench]
    fn bench_read_107(b: &mut Bencher) {
        let path = "data/p107_network.txt";
        b.iter(|| black_box(read_data(path)));
    }

    #[bench]
    fn bench_graph_107(b: &mut Bencher) {
        let path = "data/p107_network.txt";
        b.iter(|| black_box(solve_graph(path)));
    }

    #[bench]
    fn bench_prim_107(b: &mut Bencher) {
        let path = "data/p107_network.txt";
        b.iter(|| black_box(solve_prim(path)));
    }
}
