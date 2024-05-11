use std::{io, ops::Add};

use anyhow::{bail, ensure, Result};

/// Num rows and cols in the grid.
const N: usize = 100;

fn read_input() -> Result<Grid> {
    let mut rows = vec![];
    for line in io::stdin().lines() {
        let mut r = vec![];
        for c in line?.chars() {
            let lit = match c {
                '.' => false,
                '#' => true,
                _ => bail!("unexpected symbol: {c:?}"),
            };
            r.push(lit);
        }
        ensure!(r.len() == N);
        rows.push(r);
    }
    ensure!(rows.len() == N);
    Ok(Grid { rows })
}

#[derive(Clone)]
struct Grid {
    rows: Vec<Vec<bool>>,
}

impl Grid {
    fn step(&mut self) {
        let tmp = self.next();
        *self = tmp;
    }

    fn next(&self) -> Self {
        let mut out = self.clone();
        for row in 0..N as isize {
            for col in 0..N as isize {
                let p = Point { row, col };
                let count = self.live_neighbors(p);
                let new_state = if self.is_lit(p) {
                    count == 2 || count == 3
                } else {
                    count == 3
                };
                out.set(p, new_state);
            }
        }
        out
    }

    fn live_neighbors(&self, p: Point) -> usize {
        neighbors(p).into_iter().filter(|&p2| self.is_lit(p2)).count()
    }

    fn is_lit(&self, p: Point) -> bool {
        in_bounds(p);
        self.rows[p.row as usize][p.col as usize]
    }

    fn set(&mut self, p: Point, lit: bool) {
        in_bounds(p);
        self.rows[p.row as usize][p.col as usize] = lit;
    }

    fn num_lit(&self) -> usize {
        let mut count = 0;
        for row in 0..N as isize {
            for col in 0..N as isize {
                if self.is_lit(Point { row, col }) {
                    count += 1;
                }
            }
        }
        count
    }
}

fn neighbors(p: Point) -> Vec<Point> {
    let mut out = vec![];
    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            let delta = Point::new(dr, dc);

            // oops -- I forgot to include this at first
            if delta == Point::new(0, 0) {
                continue;
            }

            let p2 = p + delta;
            if in_bounds(p2) {
                out.push(p2);
            }
        }
    }
    out
}

fn in_bounds(p: Point) -> bool {
    let row = 0 <= p.row && p.row < N as isize;
    let col = 0 <= p.col && p.col < N as isize;
    row && col
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Point {
    row: isize,
    col: isize,
}

impl Point {
    fn new(row: isize, col: isize) -> Self {
        Self { row, col }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, p2: Self) -> Self {
        Self {
            row: self.row + p2.row,
            col: self.col + p2.col,
        }
    }
}

fn main() -> Result<()> {
    let mut grid = read_input()?;
    for _ in 0..100 {
        grid.step();
    }
    let ans = grid.num_lit();
    println!("{ans}");
    Ok(())
}
