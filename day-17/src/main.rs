use std::io;

use anyhow::Result;

/*
input: Vec<u32>
output: u32 --
    count how many ways to split up 150 exactly into some subset of those buckets

idea:
"brute force" -- iterate over all subsets; check whether each adds to 150
(there's probably ways to speed this up in practice, but idk if we need to)

let's try a recursive thing? idk if it'll be too slow somehow, but let's find out
*/

/// Each element has its own unique indentity, so there are always 2^n subsets.
fn subsets(xs: &[u32]) -> Vec<Vec<u32>> {
    if xs.is_empty() {
        let empty = vec![];
        return vec![empty];
    }

    let n = xs.len();
    let mut out = Vec::with_capacity(2_usize.pow(n.try_into().unwrap()));
    for set in subsets(&xs[1..]) {
        let mut with = Vec::with_capacity(n);
        with.push(xs[0]);
        with.extend_from_slice(&set);
        out.push(with);

        let without = set;
        out.push(without);
    }
    out
}

fn _main() -> Result<()> {
    let mut nums = vec![];
    for line in io::stdin().lines() {
        let x: u32 = line?.parse()?;
        nums.push(x);
    }

    let mut num_ways = 0;
    for set in subsets(&nums) {
        let sum: u32 = set.iter().copied().sum();
        if sum == 150 {
            num_ways += 1;
        }
    }
    println!("{num_ways}");

    Ok(())
}

fn main() -> Result<()> {
    let mut nums = vec![];
    for line in io::stdin().lines() {
        let x: u32 = line?.parse()?;
        nums.push(x);
    }

    let subsets = subsets(&nums);

    let mut min_size = usize::MAX;
    let mut num_ways = 0; // (using only min_size buckets)
    for set in &subsets {
        let sum: u32 = set.iter().copied().sum();
        if sum == 150 {
            if set.len() < min_size {
                min_size = set.len();
                num_ways = 1;
            } else if set.len() == min_size {
                num_ways += 1;
            }
        }
    }

    println!("{num_ways}");

    Ok(())
}
