fn main() {
    // let mut pw = *b"hxbxwxba";
    let mut pw = *b"hxbxxyzz";
    loop {
        pw = incr(pw);
        if is_valid(pw) {
            let s = String::from_utf8(pw.to_vec()).unwrap();
            dbg!(s);
            return;
        }
    }
}

fn is_valid(s: Pw) -> bool {
    has_straight(s) && avoids_iol(s) && two_pairs(s)
}

fn has_straight(s: Pw) -> bool {
    s.windows(3).any(|w| {
        let &[a, b, c] = w else { unreachable!() };
        b == a + 1 && c == a + 2
    })
}

// todo: should we allow aast_xyaa ?
// It's not so clear from the problem statement...
// I think we'll go with allowing it, for now.
fn two_pairs(s: Pw) -> bool {
    let Some((leftmost, _)) = s.windows(2).enumerate().find(|(_, w)| w[0] == w[1]) else {
        return false;
    };
    let Some((rightmost, _)) = s.windows(2).enumerate().rfind(|(_, w)| w[0] == w[1]) else {
        return false;
    };

    // We win if they're non-overlapping.
    leftmost + 1 < rightmost
}

fn avoids_iol(s: Pw) -> bool {
    let iol = b"iol";
    s.into_iter().all(|c| !iol.contains(&c))
}

type Pw = [u8; 8];

fn incr(mut s: Pw) -> Pw {
    for c in s {
        let ascii_lower = b'a'..=b'z';
        assert!(ascii_lower.contains(&c));
    }

    for i in (0..8).rev() {
        s[i] += 1;
        if s[i] <= b'z' {
            return s;
        } else {
            // Wrap around, and carry on to the next position.
            s[i] = b'a';
        }
    }

    // Edge-case: all positions wrapped, meaning input was all 'z's.
    // We could output all 'a's, but let's panic instead.
    panic!("would wrap");
}
