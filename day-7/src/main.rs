use std::{collections::HashMap, io};

use anyhow::{Context, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let circuit = parse_input()?;
    let values = circuit.simulate();
    let ans = values["a"];
    dbg!(ans);
    Ok(())
}

fn parse_input() -> Result<Circuit> {
    let mut c = Circuit::default();

    for l in io::stdin().lines() {
        let l = parse_line(&l?)?;

        // Special case: constant values.
        for i in &l.inputs {
            if let Ok(value) = i.parse() {
                c.gates.insert(i.clone(), Gate::Constant(value));
            }
        }

        for i in &l.inputs {
            c.forward_edges
                .entry(i.clone())
                .or_default()
                .push(l.output.clone());
        }
        for i in l.inputs {
            c.back_edges.entry(l.output.clone()).or_default().push(i);
        }
        c.gates.insert(l.output, l.op);
    }

    for g in c.gates.keys() {
        c.forward_edges.entry(g.clone()).or_default();
        c.back_edges.entry(g.clone()).or_default();
    }

    Ok(c)
}

struct Line {
    op: Gate,
    inputs: Vec<String>,
    output: String,
}

fn parse_line(l: &str) -> Result<Line> {
    let (lhs, rhs) = l.split_once(" -> ").context("expected ' -> '")?;
    let output = rhs.to_owned();
    let (op, inputs) = parse_lhs(lhs)?;
    Ok(Line { op, inputs, output })
}

fn parse_lhs(lhs: &str) -> Result<(Gate, Vec<String>)> {
    if lhs.starts_with("NOT ") {
        let input = lhs.strip_prefix("NOT ").unwrap().to_owned();
        return Ok((Gate::Not, vec![input]));
    }

    let ops: HashMap<_, _> = [
        (" AND ", Gate::And),
        (" OR ", Gate::Or),
        (" LSHIFT ", Gate::LeftShift),
        (" RSHIFT ", Gate::RightShift),
    ]
    .into_iter()
    .collect();

    for word in ops.keys() {
        if lhs.contains(word) {
            let (in1, in2) = lhs.split_once(word).unwrap();
            return Ok((ops[word], vec![in1.to_owned(), in2.to_owned()]));
        }
    }

    Ok((Gate::Identity, vec![lhs.to_owned()]))
}

#[derive(Default)]
struct Circuit {
    gates: HashMap<String, Gate>,
    /// There is an edge from x to y, if x is one of the inputs to y.
    forward_edges: HashMap<String, Vec<String>>,
    /// There is an edge from y to x, if x is one of the inputs to y.
    back_edges: HashMap<String, Vec<String>>,
}

impl Circuit {
    fn simulate(&self) -> HashMap<&str, u16> {
        let mut values = HashMap::new();

        let mut in_degree = HashMap::new();
        let mut exposed = vec![];
        for (g, op) in &self.gates {
            let d = op.num_inputs();
            in_degree.insert(g, d);
            if d == 0 {
                exposed.push(g);
            }
        }

        while let Some(x) = exposed.pop() {
            debug_assert_eq!(in_degree[x], 0);

            let inputs = self.back_edges[x]
                .iter()
                .map(|s| values[s.as_str()])
                .collect_vec();
            values.insert(x.as_str(), self.gates[x].eval(&inputs));

            for y in &self.forward_edges[x] {
                let d = in_degree.get_mut(y).unwrap();
                *d -= 1;
                if *d == 0 {
                    exposed.push(y);
                }
            }
        }

        values
    }
}

#[derive(Debug, Clone, Copy)]
enum Gate {
    Constant(u16),
    Identity,
    Not,
    Or,
    And,
    LeftShift,
    RightShift,
}

impl Gate {
    fn eval(self, inputs: &[u16]) -> u16 {
        assert_eq!(inputs.len(), self.num_inputs());
        let x = *inputs.get(0).unwrap_or(&0);
        let y = *inputs.get(1).unwrap_or(&0);
        match self {
            Gate::Constant(c) => c,
            Gate::Identity => x,
            Gate::Not => !x,
            Gate::Or => x | y,
            Gate::And => x & y,
            Gate::LeftShift => x << y,
            Gate::RightShift => x >> y,
        }
    }

    fn num_inputs(self) -> usize {
        match self {
            Gate::Constant(_) => 0,
            Gate::Identity => 1,
            Gate::Not => 1,
            Gate::Or => 2,
            Gate::And => 2,
            Gate::LeftShift => 2,
            Gate::RightShift => 2,
        }
    }
}
