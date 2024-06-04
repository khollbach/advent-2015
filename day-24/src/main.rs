use std::{cmp::Reverse, collections::HashSet, io};

use anyhow::Result;

fn read_input() -> Result<Vec<u32>> {
    io::stdin().lines().map(|l| Ok(l?.parse()?)).collect()
}

fn assert_unique(nums: &[u32]) {
    let set: HashSet<_> = nums.iter().collect();
    assert_eq!(set.len(), nums.len());
}

fn main() -> Result<()> {
    let mut nums = read_input()?;
    assert_unique(&nums);
    nums.sort_by_key(|&x| Reverse(x));

    let sum: u32 = nums.iter().sum();
    assert_eq!(sum % 3, 0);
    let target_sum = sum / 3;

    let group1 = first_group(&nums, target_sum);
    let ans = group1.into_iter().map(product).min().unwrap();
    assert_eq!(ans, 11_846_773_891);
    dbg!(ans);

    Ok(())
}

fn product(nums: Vec<u32>) -> u64 {
    nums.into_iter().map(|x| x as u64).product()
}

/// Return all the possibilities for Group 1.
///
/// They'll each have the same size. All we have to do is take the one with the
/// minimum product.
fn first_group(nums: &[u32], target_sum: u32) -> Vec<Vec<u32>> {
    for size in 0..nums.len() {
        let mut out = vec![];
        for group1 in subsets_of_size(nums, target_sum, size) {
            let rest = difference(nums, &group1);
            if !subsets(&rest, target_sum).is_empty() {
                out.push(group1);
            }
        }
        if !out.is_empty() {
            return out;
        }
    }
    panic!();
}

/// Set difference; naive O(n^2) impl.
fn difference(a: &[u32], b: &[u32]) -> Vec<u32> {
    a.iter().copied().filter(|x| !b.contains(x)).collect()
}

/// Return all subsets whose sum is `value` and whose length is `size`.
fn subsets_of_size(nums: &[u32], value: u32, size: usize) -> Vec<Vec<u32>> {
    // Should be faster if sorted decreasing.
    debug_assert!(nums.windows(2).all(|pair| pair[0] > pair[1]));

    if nums.is_empty() {
        if value == 0 && size == 0 {
            let empty_set = vec![];
            return vec![empty_set];
        } else {
            return vec![];
        }
    }

    let mut out = vec![];

    // Don't use nums[0].
    for s in subsets_of_size(&nums[1..], value, size) {
        out.push(s);
    }

    let x = nums[0];
    let Some(smaller_value) = value.checked_sub(x) else {
        return out;
    };
    let Some(smaller_size) = size.checked_sub(1) else {
        return out;
    };

    // Do use nums[0].
    for s in subsets_of_size(&nums[1..], smaller_value, smaller_size) {
        let mut s2 = vec![];
        s2.extend_from_slice(&s);
        s2.push(x);
        out.push(s2);
    }
    out
}

/// Return all subsets whose sum is the target value.
fn subsets(nums: &[u32], value: u32) -> Vec<Vec<u32>> {
    // Should be faster if sorted decreasing.
    debug_assert!(nums.windows(2).all(|pair| pair[0] > pair[1]));

    // Base case.
    if nums.is_empty() {
        if value == 0 {
            let empty_set = vec![];
            return vec![empty_set];
        } else {
            return vec![];
        }
    }

    let mut out = vec![];

    // Don't use nums[0].
    for s in subsets(&nums[1..], value) {
        out.push(s);
    }

    let x = nums[0];
    let Some(smaller_value) = value.checked_sub(x) else {
        return out;
    };

    // Do use nums[0].
    for s in subsets(&nums[1..], smaller_value) {
        let mut s2 = vec![];
        s2.extend_from_slice(&s);
        s2.push(x);
        out.push(s2);
    }
    out
}
