use std::io;

use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut num_nice = 0;
    for l in io::stdin().lines() {
        let l = l?;
        let first = l.chars().filter(|&c| "aeiou".contains(c)).count() >= 3;
        let second = l.chars().tuple_windows().any(|(x, y)| x == y);
        let third = !["ab", "cd", "pq", "xy"].into_iter().any(|s| l.contains(s));
        let is_nice = first && second && third;
        if is_nice {
            num_nice += 1;
        }
    }
    dbg!(num_nice);
    Ok(())
}
