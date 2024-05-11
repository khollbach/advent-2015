use anyhow::Result;
use std::{cmp::max, iter::zip};

// 225000000 -- too high
fn _attempt_1() -> Result<()> {
    let inputs = [[3, 0, 0, -3], [-3, 3, 0, 0], [-1, 0, 4, 0], [0, 0, -2, 2]];

    let mut best = 0;

    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                for d in 0..=100 - a - b - c {
                    let mut props = [0; 4];
                    for (ingredient, amount) in zip(inputs, [a, b, c, d]) {
                        for i in 0..4 {
                            props[i] += ingredient[i] * amount;
                        }
                    }

                    let mut score = 1;
                    for i in 0..4 {
                        score *= props[i];
                    }
                    best = max(best, score);
                }
            }
        }
    }

    println!("{best}");

    Ok(())
}

fn _main() -> Result<()> {
    let inputs = [[3, 0, 0, -3], [-3, 3, 0, 0], [-1, 0, 4, 0], [0, 0, -2, 2]];

    let mut best = 0;

    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                let d = 100 - a - b - c;

                let mut props = [0; 4];
                for (ingredient, amount) in zip(inputs, [a, b, c, d]) {
                    for i in 0..4 {
                        props[i] += ingredient[i] * amount;
                    }
                }

                // oops! we needed this.
                for i in 0..4 {
                    if props[i] < 0 {
                        props[i] = 0;
                    }
                }

                let mut score = 1;
                for i in 0..4 {
                    score *= props[i];
                }
                best = max(best, score);
            }
        }
    }

    println!("{best}");

    Ok(())
}

fn main() -> Result<()> {
    let inputs = [
        [3, 0, 0, -3, 2],
        [-3, 3, 0, 0, 9],
        [-1, 0, 4, 0, 1],
        [0, 0, -2, 2, 8],
    ];

    let mut best = 0;

    for a in 0..=100 {
        for b in 0..=100 - a {
            for c in 0..=100 - a - b {
                let d = 100 - a - b - c;

                let mut props = [0; 5];
                for (ingredient, amount) in zip(inputs, [a, b, c, d]) {
                    for i in 0..5 {
                        props[i] += ingredient[i] * amount;
                    }
                }

                let cals = props[4];
                if cals != 500 {
                    continue;
                }

                for i in 0..4 {
                    if props[i] < 0 {
                        props[i] = 0;
                    }
                }

                let mut score = 1;
                for i in 0..4 {
                    score *= props[i];
                }
                best = max(best, score);
            }
        }
    }

    println!("{best}");

    Ok(())
}
