fn main() {
    let input = b"iwrupvqb";

    for i in 1.. {
        let mut concat = vec![];
        concat.extend_from_slice(input);
        concat.extend_from_slice(i.to_string().as_bytes());

        let h = md5_hash(&concat);
        if h.starts_with(&"0".repeat(6)) {
            dbg!(i);
            break;
        }
    }
}

fn md5_hash(s: &[u8]) -> String {
    let digest = md5::compute(s);
    format!("{:x}", digest)
}
