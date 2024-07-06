use std::io::{self, BufReader, Read};

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let stdin = BufReader::new(io::stdin());
    let json: serde_json::Value = serde_json::from_reader(stdin)?;

    dbg!(part_2(&json));

    Ok(())
}

fn part_2(json: &serde_json::Value) -> i32 {
    let mut out = 0_i32;
    match json {
        serde_json::Value::Number(x) => {
            let x: i32 = x.as_i64().unwrap().try_into().unwrap();
            out += x;
        }
        serde_json::Value::Array(a) => {
            for elem in a {
                out += part_2(elem);
            }
        }
        serde_json::Value::Object(o) => {
            let skip = o.values().any(|v| v.as_str() == Some("red"));

            if !skip {
                for elem in o.values() {
                    out += part_2(elem);
                }
            }
        }
        _ => (),
    }
    out
}

#[allow(dead_code)]
fn part_1() -> Result<()> {
    let mut json = String::new();
    io::stdin().read_to_string(&mut json)?;
    dbg!(number_sum(&json)?);
    Ok(())
}

fn number_sum(json: &str) -> Result<i32> {
    let mut sum = 0;

    for s in json.split(|c| !is_numeric(c)) {
        if s.is_empty() {
            continue;
        }

        let x: i32 = s.parse().with_context(|| s.to_string())?;
        sum += x;
    }

    Ok(sum)
}

fn is_numeric(c: char) -> bool {
    c == '-' || ('0'..='9').contains(&c)
}

fn _main() {
    for (s, expected) in [
        ("[1,2,3]", 6),
        (r#"{"a":2,"b":4}"#, 6),
        ("[[[3]]]", 3),
        (r#"{"a":{"b":4},"c":-1}"#, 3),
        (r#"{"a":[-1,1]}"#, 0),
        (r#"[-1,{"a":1}]"#, 0),
        ("[]", 0),
        ("{}", 0),
    ] {
        let actual = number_sum(s).unwrap();
        assert_eq!(expected, actual, "{s}");
    }
}
