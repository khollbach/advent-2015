use std::io;

use anyhow::Result;

fn main() -> Result<()> {
    let mut code_total = 0;
    let mut encoded_total = 0;
    for l in io::stdin().lines() {
        let l = l?;
        code_total += l.len();
        encoded_total += encoded_len(&l);
    }
    dbg!(encoded_total - code_total);
    Ok(())
}

fn encoded_len(string_literal: &str) -> usize {
    let s = string_literal;
    assert!(s.is_ascii());

    // Each quote and backslash must be escaped.
    let special_chars = s.chars().filter(|&c| c == '"' || c == '\\').count();

    // We wrap the final string in a pair of quotes, hence the +2.
    s.len() + special_chars + 2
}

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut code_total = 0;
    let mut mem_total = 0;
    for l in io::stdin().lines() {
        let l = l?;
        code_total += l.len();
        mem_total += mem_len(&l);
    }
    dbg!(code_total - mem_total);
    Ok(())
}

fn mem_len(string_literal: &str) -> usize {
    let s = string_literal;
    assert!(s.starts_with('"'));
    assert!(s.ends_with('"'));
    let s = &s[1..s.len() - 1];

    let mut count = 0;

    assert!(s.is_ascii());
    let mut i = 0;
    while i < s.len() {
        if s.as_bytes()[i] == b'\\' {
            if s.as_bytes()[i + 1] == b'x' {
                i += 4;
            } else {
                i += 2;
            }
        } else {
            i += 1;
        }

        count += 1;
    }

    count
}
