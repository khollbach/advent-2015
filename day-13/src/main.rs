/*

notes
* 8 nodes
* complete directed weighted graph (small integer weights)
    > can simplify to an undirected graph simply by adding the two weights

want to find a ring with maximum weight

simplest thing would be brute force
* could probably copy paste a bunch of code from the travelling-santa problem

ok I guess that's the plan

---

1. parse input; get a undirected graph (can do as a matrix again)
2. for each permutation (can re-use code)
    3. get weight of that ring (note: make sure to close the loop)
4. take the overall maximum

*/

use std::{
    collections::{HashMap, HashSet},
    io,
};

use anyhow::{Context, Result};
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

fn main() -> Result<()> {
    let g = read_input()?;

    dbg_g(&g);

    let (weight, ring) = perms(g.len())
        .into_iter()
        .map(|p| (g.ring_weight(&p), p))
        .max()
        .unwrap();
    dbg!(ring, weight);

    Ok(())
}

fn dbg_g(g: &Graph) {
    let n = g.len();
    for i in 0..n {
        for j in 0..n {
            eprint!("{}\t", g.matrix[i][j]);
        }
        eprintln!();
    }
}

fn read_input() -> Result<Graph> {
    let lines: Vec<_> = io::stdin().lines().try_collect()?;

    let mut edges = vec![];
    for l in &lines {
        let (x, w, y) = parse_line(l)?;

        // Direction doesn't matter, as long as we're consistent.
        edges.push((x, y, w));
    }

    let g = Graph::new(&edges);
    Ok(g)
}

fn parse_line(l: &str) -> Result<(&str, i32, &str)> {
    static RE: Lazy<Regex> = Lazy::new(|| {
        let re = r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$";
        Regex::new(re).unwrap()
    });

    let caps = RE.captures(l).with_context(|| l.to_string())?;
    let x = caps.get(1).unwrap().as_str();
    let sign = if &caps[2] == "gain" { 1 } else { -1 };
    let w = sign * caps[3].parse::<i32>()?;
    let y = caps.get(4).unwrap().as_str();

    Ok((x, w, y))
}

/// Directed.
struct Graph {
    matrix: Vec<Vec<i32>>,
}

impl Graph {
    fn new(edges: &[(&str, &str, i32)]) -> Graph {
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

        let mut matrix = vec![vec![i32::MAX; n]; n];
        for &(x, y, w) in edges {
            let x = label_to_index[x];
            let y = label_to_index[y];
            assert_eq!(matrix[x][y], i32::MAX);
            matrix[x][y] = w;
        }

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    debug_assert_eq!(matrix[i][j], i32::MAX);
                } else {
                    debug_assert_ne!(matrix[i][j], i32::MAX);
                }
            }
        }

        Graph { matrix }
    }

    fn ring_weight(&self, ring: &[usize]) -> i32 {
        let n = self.len();
        assert_eq!(ring.iter().collect::<HashSet<_>>().len(), n);

        let mut total = 0;
        for ij in ring.windows(2) {
            let &[i, j] = ij else { unreachable!() };
            total += self.matrix[i][j] + self.matrix[j][i];
        }

        // Close the loop.
        let i = ring[0]; // BUG: originally was using 0 and n-1 instead of ring[0] and ring[n-1]
        let j = ring[n - 1];
        total += self.matrix[i][j] + self.matrix[j][i];

        total
    }

    fn len(&self) -> usize {
        self.matrix.len()
    }
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
