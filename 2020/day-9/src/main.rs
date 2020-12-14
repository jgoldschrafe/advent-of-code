use itertools::sorted;
use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub};

fn sums_to(nums: &[i64], wanted_total: i64) -> Option<(i64, i64)>
{
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

fn main() {
    let nums: Vec<i64> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    for i in 25..nums.len() {
        let num = nums[i];
        let slice = &nums[(i - 25)..i];
        let copy: Vec<i64> = sorted(slice).map(|v| *v).collect();
        if let None = sums_to(&copy, nums[i]) {
            println!("no 2 numbers in previous 25 sum to: {}", num);
            return
        }
    }
}