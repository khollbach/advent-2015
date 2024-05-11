use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::{collections::HashSet, io};

fn _main() -> Result<()> {
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

// ---

fn traverse(directions: &str, visited: &mut HashSet<(i32, i32)>) -> Result<()> {
    let mut x = 0;
    let mut y = 0;

    for c in directions.chars() {
        match c {
            '^' => y += 1,
            'v' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,
            _ => bail!("unexpected character: {c:?}"),
        }

        visited.insert((x, y));
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut even = String::new();
    let mut odd = String::new();
    {
        let (input,) = io::stdin()
            .lines()
            .collect_tuple()
            .context("expected 1 line")?;
        let input = input?;

        for (i, c) in input.chars().enumerate() {
            if i % 2 == 0 {
                even.push(c);
            } else {
                odd.push(c);
            }
        }
    }

    let mut visited = HashSet::new();
    {
        // Initial house.
        visited.insert((0, 0));
        
        traverse(&even, &mut visited)?;
        traverse(&odd, &mut visited)?;
    }

    println!("{}", visited.len());

    Ok(())
}

