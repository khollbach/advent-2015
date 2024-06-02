use std::{cmp::Reverse, io};

use itertools::Itertools;

fn main() {
    let mut nums = io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u32>().unwrap())
        .collect_vec();
    let sum: u32 = nums.iter().sum();
    dbg!(nums.len());
    dbg!(sum);
    dbg!(sum / 3, sum % 3);
    // note: nums are unique

    nums.sort_by_key(|&x| Reverse(x));

    // Group A
    let mut ss1 = subsets(&nums, sum / 3);
    dbg!(ss1.len());
    ss1.sort_by_key(|s| s.len());
    // dbg!(&ss1[..10]);

    let mut short = vec![];

    for s1 in ss1 {
        if s1.len() > 6 {
            break;
        }
        let rest = nums
            .iter()
            .copied()
            .filter(|x| !s1.contains(x))
            .collect_vec();
        let ok = !subsets(&rest, sum / 3).is_empty();
        if ok {
            short.push(s1);
        }
    }

    dbg!(short.len());

    // short.sort_by_key(|s| qe(s));
    // dbg!(&short[0], qe(&short[0]));

    let min = short.iter().map(|s| qe(s)).min().unwrap();
    dbg!(min);

    // guess: 25060087
    // * wrong answer, too low
    // * maybe integer overflow?

    // yep! The product was overflowing u32; but u64 is fine.
    // ans: 11846773891

    // todo: part 2
}

fn qe(set: &[u32]) -> u64 {
    set.iter().map(|&x| x as u64).product()
}

fn subsets(nums: &[u32], target: u32) -> Vec<Vec<u32>> {
    debug_assert!(nums.windows(2).all(|pair| pair[0] > pair[1]));

    // Base case.
    if nums.is_empty() {
        if target == 0 {
            let empty_set = vec![];
            return vec![empty_set];
        } else {
            return vec![];
        }
    }

    let mut out = vec![];

    // Don't use nums[0].
    for s in subsets(&nums[1..], target) {
        out.push(s);
    }

    let x = nums[0];
    let Some(smaller_target) = target.checked_sub(x) else {
        return out;
    };

    // Do use nums[0].
    for s in subsets(&nums[1..], smaller_target) {
        let mut s2 = vec![];
        s2.extend_from_slice(&s);
        s2.push(x);
        out.push(s2);
    }
    out
}

/*

#pkgs in group A:  5..

groups of pkgs of that size (e.g. 29 choose 5)
* filter for sum==520

of remaining pkgs, partition in two, (s.t. sum == 520)

*/
