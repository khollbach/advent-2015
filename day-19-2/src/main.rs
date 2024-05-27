/*

uhh. 

I mean, it seems to work! :)

*/

use std::{collections::HashMap, io};

use itertools::Itertools;

fn main() {
    let (contractions, mut s) = read_input();
    dbg_contractions(&contractions);

    for num_iters in 0.. {
        eprintln!("{:?}", std::str::from_utf8(&s).unwrap());
        s = shrink(&s, &contractions, num_iters);
    }

    // note: the answer is num_iters + 1, since there's a final "-> e" step.
}

fn dbg_contractions(contractions: &Contractions) {
    eprintln!("{{");
    for (k, v) in contractions {
        eprintln!("    {:?} -> {:?}",
        std::str::from_utf8(k).unwrap(),
        std::str::from_utf8(v).unwrap());
    }
    eprintln!("}}");
}

fn shrink(s: &[u8], contractions: &Contractions, iter: usize) -> Vec<u8> {
    let n = s.len();

    // for i in 0..n {
    //     for j in i..=n {

    for len in (0..n).rev() {
        for i in 0..n - len {
            let j = i + len;

            if let Some(sym) = contractions.get(&s[i..j]) {
                eprintln!("shrinking: {:?} -> {:?} at [{i}..{j}]",
                std::str::from_utf8(&s[i..j]).unwrap(),
                std::str::from_utf8(sym).unwrap());

                let mut shorter = Vec::new();
                shorter.extend_from_slice(&s[..i]);
                shorter.extend_from_slice(sym);
                shorter.extend_from_slice(&s[j..]);
                return shorter;
            }
        }
    }
    panic!("failed to shrink. s: {:?}. iter: {iter}", std::str::from_utf8(s).unwrap());
}

type Contractions = HashMap<Vec<u8>, Vec<u8>>;

fn read_input() -> (Contractions, Vec<u8>) {
    let mut lines = io::stdin().lines().map(Result::unwrap);

    let mut contractions = Contractions::new();
    for l in &mut lines {
        let l = l.trim();
        if l.is_empty() {
            break;
        }

        let (short, long) = l.split_once(" => ").unwrap();
        let exists = contractions.insert(long.as_bytes().to_vec(), short.as_bytes().to_vec());
        assert!(exists.is_none());
    }

    let (long_string,) = lines.collect_tuple().unwrap();
    (contractions, long_string.into())
}
