mod input;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    mem,
};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Elem {
    code: String,
}

type Seq = Vec<Elem>;

type Rules = HashMap<Elem, Vec<Seq>>;
type RevRules = HashMap<Seq, Vec<Elem>>;

/*
left off here:
* write down the data types involved and get an understanding of the graph
    (how we'll represent it, etc) and the edges.
* I think we'll be generating the edge-sets on-the-fly, so make sure to
    get a clearer picture of that
* from there I *think* it'll be easier to proceed? Should be fun in any case!
    This does feel like the edge of my abilities...!!

*/

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

/*
new idea:
proceed in two phases,
ph1. go a few steps out from Ee;
        call this the new target set
        (really it's a map from String to Distance)
    * note: we can hard-code a number of steps based on
            how long it takes to run.
ph2. do the current thing of stepping back from initial_seq
        and treat winning as finding the target

Ok this isn't quite right ...
    because if we exit the reverse-search immediately upon finding
    a target, we might not've found the optimal path yet; since the
    targets aren't equal distance from Ee

----

Hmmmm... let's try the classical version of this algorithm:
* two BFSs interleaved
Should be fun to figure out the implementation details!

*/

fn main() -> Result<()> {
    let (rules, initial_seq) = input::read_input()?;
    let rules = reverse_rules(&rules);
    let target = vec![Elem::new("Ee")?];
    todo!()
}

fn shortest_path(s: Seq, t: Seq, rules: Rules) -> Option<usize> {
    let rev_rules = reverse_rules(&rules);
    let longest_seq = longest_seq(&rev_rules);

    let mut seen = [HashSet::new(), HashSet::new()];
    let mut to_visit = [VecDeque::new(), VecDeque::new()];

    // todo: init collections

    // todo: figure out dist calc; incl obo

    for i in [0, 1].into_iter().cycle() {
        // visit this "level"

        for seq in mem::take(&mut to_visit[i]) {
            if seen[1 - i].contains(&seq) {
                return Some(todo!());
            }

            let edges = if i == 0 {
                forward_edges(&seq, &rules)
            } else {
                backward_edges(&seq, &rev_rules)
            };

            for e in edges {
                if seen[i].contains(&e) {
                    continue;
                }
                seen[i].insert(e.clone());
                to_visit[i].push_back(e);
            }
        }
    }

    None
}

fn forward_edges(s: &Seq, rules: &Rules) -> Vec<Seq> {
    todo!()
}

fn backward_edges(s: &Seq, rev_rules: &RevRules) -> Vec<Seq> {
    todo!()
}

fn forward_search() -> HashMap<Seq, usize> {
    todo!()
}

fn reverse_search(
    initial_seq: Seq,
    rules: &RevRules,
    targets: &HashMap<Seq, usize>,
) -> Option<usize> {
    let longest_seq = longest_seq(&rules);

    let mut seen = HashSet::new();
    seen.insert(initial_seq.clone());

    let mut to_visit = VecDeque::new();
    to_visit.push_back((initial_seq, 0));

    while let Some((seq, dist)) = to_visit.pop_front() {
        if let Some(target_dist) = targets.get(&seq) {
            return Some(dist + target_dist);
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

    None
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
