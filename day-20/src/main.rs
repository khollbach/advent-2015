fn main() {
    let limit = 10_000_000;
    let mut num_presents = vec![0; limit + 1];

    for elf in 1..=limit {
        for i in 1..=50 {
            let h = elf * i;
            if h > limit {
                break;
            }
            num_presents[h] += elf;
        }
    }

    let target = 36_000_000;
    for h in 1..=limit {
        if num_presents[h] * 11 >= target {
            println!("{h}");
            break;
        }
    }
}

fn _main() {
    // dbg!(num_presents(1_000_000));
    // dbg!(num_presents(10_000_000));
    // panic!();

    let limit = 10_000_000;
    let mut num_presents = vec![0; limit + 1];

    for elf in 1..=limit {
        let mut i = elf;
        while i <= limit {
            num_presents[i] += elf;
            i += elf;
        }
    }

    let target = 36_000_000 / 10;
    for h in 1..=limit {
        if num_presents[h] >= target {
            println!("{h}");
            break;
        }
    }
}

#[allow(dead_code)]
fn naive() {
    let target = 36_000_000;
    for h in 1.. {
        // println!("{h} {}", num_presents(h));
        if num_presents(h) >= target {
            println!("{h}");
            break;
        }
    }
}

fn num_presents(h: u32) -> u32 {
    10 * divisors(h).into_iter().sum::<u32>()
}

fn divisors(x: u32) -> Vec<u32> {
    let mut out = vec![];
    for i in 1..=x {
        if x % i == 0 {
            out.push(i);
        }
    }
    out
}
