use std::io;

use anyhow::{bail, Context, Result};
use itertools::Itertools;

const PART2: bool = true;

fn main() -> Result<()> {
    // let lines: Vec<_> = io::stdin().lines().try_collect()?;
    let lines = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .lines()
        .collect_vec();

    let mut regs = if PART2 { [1, 0] } else { [0u64; 2] };
    let mut curr_instr = 0isize;

    while 0 <= curr_instr && curr_instr < lines.len() as isize {
        let l = &lines[curr_instr as usize];

        // // "single-step debugger"
        // eprint!("[{curr_instr}: {l}]");
        // io::stdin().lines().next();

        let Instr { op, ab, offset } = parse_instr(l)?;

        let mut jump_taken = false;
        match op {
            "hlf" => {
                debug_assert_eq!(regs[ab] % 2, 0);
                regs[ab] /= 2;
            }
            "tpl" => regs[ab] *= 3,
            "inc" => regs[ab] += 1,

            "jmp" => {
                curr_instr += offset;
                jump_taken = true;
            }
            "jie" => {
                if regs[ab] % 2 == 0 {
                    curr_instr += offset;
                    jump_taken = true;
                }
            }
            "jio" => {
                if regs[ab] == 1 {
                    curr_instr += offset;
                    jump_taken = true;
                }
            }

            _ => bail!("invalid opcode: {:?}", op),
        }
        if !jump_taken {
            curr_instr += 1;
        }
    }

    dbg!(regs);

    Ok(())
}

struct Instr<'a> {
    op: &'a str,
    ab: usize,
    offset: isize,
}

fn parse_instr(l: &str) -> Result<Instr> {
    let (op, args) = l.split_once(' ').context("expected space")?;

    let (ab, offset) = match op {
        "jie" | "jio" => args.split_once(", ").context(r#"expected ", ""#)?,
        "jmp" => ("a", args),
        _ => (args, "0"),
    };

    let ab = match ab {
        "a" => 0,
        "b" => 1,
        _ => bail!("expected a or b, got {:?}", ab),
    };
    let offset: isize = offset.parse()?;

    Ok(Instr { op, ab, offset })
}
