use std::io;
use std::io::prelude::*;

fn sums_to(nums: &[i32], wanted_total: i32) -> Option<(i32, i32)> {
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

fn sums_to_three_way(nums: &[i32], wanted_total: i32) -> Option<(i32, i32, i32)> {
    for tail in (2..nums.len()).rev() {
        let z = nums[tail];
        if z > wanted_total {
            continue;
        }

        let remainder = wanted_total - z;
        if let Some((x, y)) = sums_to(&nums[0..tail], remainder) {
            return Some((x, y, z));
        }
    }

    None
}

fn main() {
    let mut nums: Vec<i32> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    nums.sort();

    /*if let Some((x, y)) = sums_to(&nums[..], 2020) {
        println!("{}", x * y);
    }*/

    if let Some((x, y, z)) = sums_to_three_way(&nums[..], 2020) {
        println!("{}", x * y * z);
    }
}
