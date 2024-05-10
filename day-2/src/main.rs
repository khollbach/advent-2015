use anyhow::{Context, Result};
use itertools::Itertools;
use std::io;

fn _main() -> Result<()> {
    let mut total = 0;
    for line in io::stdin().lines() {
        let (x, y, z) = line?
            .split('x')
            .map(|s| s.parse::<u32>().expect("each dimension must be a u32"))
            .collect_tuple()
            .context("expected 3 dimensions")?;
        let (a, b, c) = (x * y, y * z, x * z);
        let area = 2 * (a + b + c);
        let extra = [a, b, c].into_iter().min().unwrap();
        total += area + extra;
    }
    println!("{total}");
    Ok(())
}

fn main() -> Result<()> {
    let mut total = 0;
    for line in io::stdin().lines() {
        let (x, y, z) = line?
            .split('x')
            .map(|s| s.parse::<u32>().expect("each dimension must be a u32"))
            .collect_tuple()
            .context("expected 3 dimensions")?;
        let (a, b, c) = (x + y, y + z, x + z);
        let min_perim = 2 * [a, b, c].into_iter().min().unwrap();
        let volume = x * y * z;
        total += min_perim + volume;
    }
    println!("{total}");
    Ok(())
}
