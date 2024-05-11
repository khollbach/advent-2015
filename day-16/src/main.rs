use std::{collections::HashMap, io, iter::zip};

use anyhow::{ensure, Context, Result};

/*
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
*/

fn read_input() -> Result<Vec<HashMap<String, u32>>> {
    let mut out = vec![];
    for line in io::stdin().lines() {
        let a = parse_aunt(&line?)?;
        out.push(a);
    }
    Ok(out)
}

/// Example input:
/// ```
/// Sue 1: goldfish: 6, trees: 9, akitas: 0
/// ```
/// Doesn't do proper validation.
fn parse_aunt(line: &str) -> Result<HashMap<String, u32>> {
    let (label, properties) = line.split_once(':').context("expected at least one ':'")?;
    ensure!(label.starts_with("Sue "));

    let mut map = HashMap::new();
    for kv in properties.split(',') {
        let (k, v) = kv
            .split_once(':')
            .context("expected ':' in key-value pair")?;
        let k = k.trim().to_owned();
        let v = v
            .trim()
            .parse()
            .with_context(|| format!("failed to parse value of key-value pair {kv:?}"))?;
        map.insert(k, v);
    }
    Ok(map)
}

const ACTUAL: &str = "Sue 0: children: 3, cats: 7, samoyeds: 2, pomeranians: 3, akitas: 0, vizslas: 0, goldfish: 5, trees: 3, cars: 2, perfumes: 1";

#[allow(unused)]
fn part_1() -> Result<()> {
    let actual = parse_aunt(ACTUAL)?;
    for (aunt, i) in zip(read_input()?, 1..) {
        if is_subset(&aunt, &actual) {
            println!("{i}");
        }
    }

    Ok(())
}

/// We'll say `x subset y` if every kv pair in x is also in y.
fn is_subset(x: &HashMap<String, u32>, y: &HashMap<String, u32>) -> bool {
    x.iter().all(|(k, v)| y.get(k) == Some(v))
}

fn main() -> Result<()> {
    let actual = parse_aunt(ACTUAL)?;
    for (aunt, i) in zip(read_input()?, 1..) {
        if is_possible(&aunt, &actual) {
            println!("{i}");
        }
    }

    Ok(())
}

/// See the rules for part two.
fn is_possible(candidate: &HashMap<String, u32>, actual: &HashMap<String, u32>) -> bool {
    candidate.iter().all(|(k, &v)| {
        if !actual.contains_key(k) {
            return false;
        }
        match k.as_str() {
            "cats" | "trees" => v > actual[k],
            "pomeranians" | "goldfish" => v < actual[k],
            _ => v == actual[k],
        }
    })
}
