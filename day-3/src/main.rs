use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{collections::HashSet, io};

fn main() -> Result<()> {
    let (input,) = io::stdin()
        .lines()
        .collect_tuple()
        .context("expected 1 line")?;

    let mut visited = HashSet::new();
    let mut x = 0;
    let mut y = 0;

    // Initial house.
    visited.insert((x, y));

    for c in input?.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => bail!("unexpected character: {c:?}"),
        }

        visited.insert((x, y));
    }

    println!("{}", visited.len());

    Ok(())
}
