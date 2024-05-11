mod input;

use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Elem {
    code: String,
}

type Seq = Vec<Elem>;

type Rules = HashMap<Elem, Vec<Seq>>;
type RevRules = HashMap<Seq, Vec<Elem>>;

fn reverse_rules(rules: &Rules) -> RevRules {
    let mut out = RevRules::new();
    for (elem, seqs) in rules {
        for seq in seqs {
            out.entry(seq.clone()).or_default().push(elem.clone());
        }
    }
    out
}

fn longest_seq(rules: &RevRules) -> usize {
    rules.keys().map(|seq| seq.len()).max().unwrap_or(0)
}

fn main() -> Result<()> {
    let (rules, initial_seq) = input::read_input()?;
    let rules = reverse_rules(&rules);
    let target = vec![Elem::new("Ee")?];

    let longest_seq = longest_seq(&rules);

    let mut seen = HashSet::new();
    seen.insert(initial_seq.clone());

    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_seq, 0));

    while let Some((seq, dist)) = to_visit.pop_front() {
        if seq == target {
            println!("{dist}");
            break;
        }

        for i in 0..seq.len() {
            for len in 1..=longest_seq {
                // (oops -- I got the bounds wrong the first time around)
                if i + len > seq.len() {
                    break;
                }

                let slice = &seq[i..i + len];
                let Some(elems) = rules.get(slice) else {
                    continue;
                };

                for e in elems {
                    let mut new_seq = Vec::with_capacity(seq.len() + 1 - len);
                    new_seq.extend_from_slice(&seq[..i]);
                    new_seq.push(e.clone());
                    new_seq.extend_from_slice(&seq[i + len..]);

                    if !seen.contains(&new_seq) {
                        seen.insert(new_seq.clone());
                        to_visit.push_back((new_seq, dist + 1));
                    }
                }
            }
        }
    }

    Ok(())
}

/*
idea:
BFS through the DAG
stop when you reach "Ee"

DAG?:
* nodes are strings
* edge from long string to shorter one if
    there's a rule that lets us contract a slice of the long string

*/
