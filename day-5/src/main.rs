use std::io;

use anyhow::Result;
use itertools::Itertools;

fn _main1() -> Result<()> {
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

fn main() -> Result<()> {
    let mut num_nice = 0;
    for l in io::stdin().lines() {
        let l = l?;
        let is_nice = first(&l) && second(&l);
        if is_nice {
            num_nice += 1;
        }
    }
    dbg!(num_nice);
    Ok(())
}

fn first(l: &str) -> bool {
    assert!(l.is_ascii());
    let l = l.as_bytes();
    let n = l.len();
    for i in 0..=n - 2 {
        for j in i + 2..=n - 2 {
            if l[i..][..2] == l[j..][..2] {
                return true;
            }
        }
    }
    false
}

fn second(l: &str) -> bool {
    assert!(l.is_ascii());
    let l = l.as_bytes();
    let n = l.len();
    for i in 0..=n - 3 {
        if l[i] == l[i + 2] {
            return true;
        }
    }
    false
}
