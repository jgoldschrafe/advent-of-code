use itertools::sorted;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

fn sums_to(nums: &[i64], wanted_total: i64) -> Option<(i64, i64)> {
    let mut head = 0;
    let mut tail = nums.len() - 1;
    while head < tail {
        let (x, y) = (nums[head], nums[tail]);
        let sum = x + y;

        if sum == wanted_total {
            return Some((x, y));
        } else if sum < wanted_total {
            head += 1;
        } else {
            tail -= 1;
        }
    }

    None
}

fn window_sums_to(nums: &[i64], wanted_total: i64) -> Option<Vec<i64>> {
    let mut head = 0;
    let mut tail = 0;
    let mut sum = nums[0];

    while tail < nums.len() {
        if sum == wanted_total {
            let result = nums[head..=tail].iter().map(|v| *v).collect();
            return Some(result);
        } else if sum < wanted_total {
            tail += 1;
            sum += nums[tail];
        } else {
            sum -= nums[head];
            head += 1;
        }
    }

    None
}

fn main() {
    let nums: Vec<i64> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    // For each number in the sequence, find 2 numbers in the previous 25 that
    // sum to the number
    let mut misfit: i64 = 0;
    for i in 25..nums.len() {
        let num = nums[i];
        let slice = &nums[(i - 25)..i];
        let copy: Vec<i64> = sorted(slice).map(|v| *v).collect();
        if let None = sums_to(&copy, nums[i]) {
            println!("no 2 numbers in previous 25 sum to: {}", num);
            misfit = num;
            break
        }
    }

    // Find a range of consecutive numbers summing to the misfit number above.
    // Then, output the sum of the smallest and largest numbers in that range.
    if let Some(summed) = window_sums_to(&nums, misfit) {
        let min = summed.iter().min().unwrap();
        let max = summed.iter().max().unwrap();
        println!("found range between {} and {}; {} + {} = {}", min, max, min, max, min + max);
    }
}