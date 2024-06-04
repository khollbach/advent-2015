use std::io;

use anyhow::{bail, ensure, Context, Result};

fn main() -> Result<()> {
    let instrs = read_input()?;

    let mut grid = Grid2::new();
    for i in instrs {
        let r = i.rect;
        match i.op {
            Op::TurnOff => grid.turn_off(r),
            Op::TurnOn => grid.turn_on(r),
            Op::Toggle => grid.toggle(r),
        }
    }
    let ans = grid.total_brightness();
    dbg!(ans);

    Ok(())
}

struct Grid2 {
    rows: Vec<Vec<u32>>,
}

impl Grid2 {
    fn new() -> Self {
        Self {
            rows: vec![vec![0; 1_000]; 1_000],
        }
    }

    fn turn_on(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] += 1;
            }
        }
    }

    fn turn_off(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] = self.rows[row][col].saturating_sub(1);
            }
        }
    }

    fn toggle(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] += 2;
            }
        }
    }

    fn total_brightness(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                count += self.rows[i][j];
            }
        }
        count
    }
}

#[allow(dead_code)]
struct Grid1 {
    rows: Vec<Vec<bool>>,
}

#[allow(dead_code)]
impl Grid1 {
    fn new() -> Self {
        Self {
            rows: vec![vec![false; 1_000]; 1_000],
        }
    }

    fn turn_on(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] = true;
            }
        }
    }

    fn turn_off(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] = false;
            }
        }
    }

    fn toggle(&mut self, r: Rect) {
        for row in r.top_left.row..=r.bot_right.row {
            for col in r.top_left.col..=r.bot_right.col {
                self.rows[row][col] ^= true;
            }
        }
    }

    fn num_lit(&self) -> usize {
        let mut count = 0;
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if self.rows[i][j] {
                    count += 1;
                }
            }
        }
        count
    }
}

struct Point {
    row: usize,
    col: usize,
}

struct Rect {
    top_left: Point,
    /// Inclusive.
    bot_right: Point,
}

struct Instr {
    op: Op,
    rect: Rect,
}

enum Op {
    TurnOff,
    TurnOn,
    Toggle,
}

fn read_input() -> Result<Vec<Instr>> {
    let mut out = vec![];
    for l in io::stdin().lines() {
        let l = l?;
        let (op_and_p1, p2) = l.split_once(" through ").context("expected ' through '")?;
        let (op, p1) = op_and_p1
            .rsplit_once(' ')
            .context("expected space b/w op and p1")?;

        let op = Op::new(op)?;

        let top_left = Point::new(p1)?;
        let bot_right = Point::new(p2)?;
        let rect = Rect::new(top_left, bot_right)?;

        out.push(Instr { op, rect });
    }
    Ok(out)
}

impl Op {
    fn new(s: &str) -> Result<Self> {
        let out = match s {
            "turn off" => Self::TurnOff,
            "turn on" => Self::TurnOn,
            "toggle" => Self::Toggle,
            _ => bail!("not an op: {s:?}"),
        };
        Ok(out)
    }
}

impl Point {
    fn new(s: &str) -> Result<Self> {
        let (row, col) = s.split_once(',').context("expected ','")?;
        let row = row.parse()?;
        let col = col.parse()?;
        Ok(Self { row, col })
    }
}

impl Rect {
    fn new(top_left: Point, bot_right: Point) -> Result<Self> {
        ensure!(top_left.row <= bot_right.row);
        ensure!(top_left.col <= bot_right.col);
        Ok(Self {
            top_left,
            bot_right,
        })
    }
}
