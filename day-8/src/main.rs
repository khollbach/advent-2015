use std::io;

use anyhow::Result;

fn main() -> Result<()> {
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
