use itertools::Itertools;

fn main() {
    let mut s = "1321131112".to_string();
    for _ in 0..40 {
        s = next(&s);
    }
    dbg!(s.len());

    let mut s = "1321131112".to_string();
    for _ in 0..50 {
        s = next(&s);
    }
    dbg!(s.len());
}

fn next(s: &str) -> String {
    let mut out = String::new();
    for (c, chunk) in &s.chars().chunk_by(|&c| c) {
        let n = chunk.count();
        out.push_str(&n.to_string());
        out.push(c);
    }
    out
}
