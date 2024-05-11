use std::{
    collections::{HashMap, HashSet},
    io,
};

use anyhow::{ensure, Context, Result};
use itertools::Itertools;

// rules: HashMap<Elem, Vec<Seq>>
// initial_seq: Seq

#[derive(Clone, PartialEq, Eq, Hash)]
struct Elem {
    code: String,
}

type Seq = Vec<Elem>;

/*

1. split the input sequence up into elements
2. parse the "rules" map

3. for each pos'n in the input-seq, apply any rules that match that elem.
3.5 build up an output HashSet of Seqs we can get to this way.

*/

fn possible_seqs(init_seq: Seq, rules: &Rules) -> HashSet<Seq> {
    let mut out = HashSet::new();
    for i in 0..init_seq.len() {
        let elem = &init_seq[i];
        let Some(replacements) = rules.get(elem) else {
            continue;
        };
        for r in replacements {
            let mut new_seq = Vec::with_capacity(init_seq.len() + r.len());
            new_seq.extend_from_slice(&init_seq[..i]);
            new_seq.extend_from_slice(&r);
            new_seq.extend_from_slice(&init_seq[i + 1..]);
            out.insert(new_seq);
        }
    }
    out
}

type Rules = HashMap<Elem, Vec<Seq>>;

/// Let's not worry about *validating* the input.
fn read_input() -> Result<(Rules, Seq)> {
    let mut lines = io::stdin().lines();

    let mut rules = Rules::new();
    for l in &mut lines {
        let l = l?;
        let l = l.trim();
        if l.is_empty() {
            break;
        }

        let (elem, seq) = parse_rule(l)?;
        rules.entry(elem).or_default().push(seq);
    }

    let (l,) = lines
        .collect_tuple()
        .context("expected only one line after rules")?;
    let init_seq = parse_seq(&l?)?;

    Ok((rules, init_seq))
}

fn parse_rule(line: &str) -> Result<(Elem, Seq)> {
    let (elem, seq) = line
        .trim()
        .split_once(" => ")
        .context("expected ' => ' in rule")?;
    let elem = Elem::new(elem)?;
    let seq = parse_seq(seq)?;
    Ok((elem, seq))
}

fn parse_seq(s: &str) -> Result<Seq> {
    // 1. split up into strings
    // 2. Elem::new each string

    ensure!(s.is_ascii());
    ensure!(s.chars().all(|c| c.is_ascii_alphabetic()));

    let mut s = s.to_owned();
    s.push('Z'); // sentinal value to make parsing easier

    // Find the indices of the starts of words.
    let mut uppers = vec![];
    for i in 0..s.len() {
        if get_char(&s, i).is_ascii_uppercase() {
            uppers.push(i);
        }
    }

    let mut out = vec![];
    for ij in uppers.windows(2) {
        let &[i, j] = ij else { unreachable!() };
        let elem = Elem::new(&s[i..j])?;
        out.push(elem);
    }
    Ok(out)
}

/// Only works for ascii strings.
fn get_char(s: &str, i: usize) -> char {
    let b = s.as_bytes()[i];
    b as char
}

impl Elem {
    fn new(code: &str) -> Result<Self> {
        ensure!(!code.is_empty());
        ensure!(code.len() <= 2);
        ensure!(code.is_ascii());

        let first = code.chars().next().unwrap();
        ensure!(first.is_ascii_uppercase());

        if code.len() == 2 {
            let second = code.chars().skip(1).next().unwrap();
            ensure!(second.is_ascii_lowercase());
        }

        Ok(Self {
            code: code.to_owned(),
        })
    }
}

fn main() -> Result<()> {
    let (rules, init_seq) = read_input()?;
    let ans = possible_seqs(init_seq, &rules).len();
    println!("{ans}");
    Ok(())
}
