/*

- 8 nodes
- complete undirected graph
- weighted edges
- find the shortest path visiting each node exactly once

idea
- parse the input
- generate the graph: ~ m a t r i x ~
    for each pair of indices, there should be an edge-weight
    (diagonal can be +infty)
- generate all permutations (of indices)
- for each, calculate its path-length:
    - sum over all windows of size 2 the corresponding edge weight
- output the overall minimum path-length

*/

use std::{
    collections::{HashMap, HashSet},
    io,
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let lines: Vec<_> = io::stdin().lines().try_collect()?;

    let mut edges = vec![];
    for l in &lines {
        let (x, y, w) = parse_line(l)?;
        edges.push((x, y, w));
    }

    let g = Graph::new(&edges);

    let shortest_path = perms(g.len())
        .into_iter()
        .map(|p| g.path_len(&p))
        .min()
        .unwrap();
    dbg!(shortest_path);

    Ok(())
}

/// Return all n! permutations of 0..n
fn perms(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        let empty_sequence = vec![];
        return vec![empty_sequence];
    }

    let n_factorial = (1..=n).product();
    let mut out = Vec::with_capacity(n_factorial);

    // Recursively compute permutations *without* the largest value.
    for p in perms(n - 1) {
        // Insert the largest value at each possible position in p.
        for i in 0..=n - 1 {
            let mut p2 = Vec::with_capacity(n);
            p2.extend_from_slice(&p[..i]);
            p2.push(n - 1);
            p2.extend_from_slice(&p[i..]);
            out.push(p2);
        }
    }

    out
}

struct Graph {
    matrix: Vec<Vec<u32>>,
}

impl Graph {
    fn path_len(&self, path: &[usize]) -> u32 {
        let mut total = 0;
        for ij in path.windows(2) {
            let &[i, j] = ij else { unreachable!() };
            total += self.matrix[i][j];
        }
        total
    }

    fn new(edges: &[(&str, &str, u32)]) -> Graph {
        let mut labels = HashSet::new();
        for &(x, y, _) in edges {
            labels.insert(x);
            labels.insert(y);
        }
        let n = labels.len();

        let mut label_to_index = HashMap::with_capacity(n);
        for (i, l) in labels.into_iter().enumerate() {
            label_to_index.insert(l, i);
        }

        let mut matrix = vec![vec![u32::MAX; n]; n];
        for &(x, y, w) in edges {
            let x = label_to_index[x];
            let y = label_to_index[y];
            assert_eq!(matrix[x][y], u32::MAX);
            assert_eq!(matrix[y][x], u32::MAX);
            matrix[x][y] = w;
            matrix[y][x] = w;
        }

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    debug_assert_eq!(matrix[i][j], u32::MAX);
                } else {
                    debug_assert_ne!(matrix[i][j], u32::MAX);
                }
            }
        }

        Graph { matrix }
    }

    fn len(&self) -> usize {
        self.matrix.len()
    }
}

fn parse_line(l: &str) -> Result<(&str, &str, u32)> {
    let (x, to, y, eq, w) = l
        .split_whitespace()
        .collect_tuple()
        .context("expected 5 words")?;

    ensure!(to == "to");
    ensure!(eq == "=");

    ensure!(x.chars().all(|c| c.is_ascii_alphabetic()));
    ensure!(y.chars().all(|c| c.is_ascii_alphabetic()));
    let w: u32 = w.parse()?;

    Ok((x, y, w))
}
