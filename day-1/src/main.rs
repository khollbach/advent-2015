use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::io;

fn main() -> Result<()> {
    let (input,) = io::stdin()
        .lines()
        .collect_tuple()
        .context("expected 1 line")?;
    let mut floor = 0;
    for c in input?.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => bail!("invalid char: {c:?}"),
        }
    }
    println!("{floor}");
    Ok(())
}
