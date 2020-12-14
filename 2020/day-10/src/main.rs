use itertools::Itertools;
use itertools::sorted;
use std::io;
use std::io::prelude::*;
use std::iter;

fn joltage_differences(adapters: &Vec<i32>) -> Result<(i32, i32, i32), ()> {
    let mut by_1 = 0;
    let mut by_2 = 0;
    let mut by_3 = 0;

    let sorted_adapters: Vec<i32> = sorted(adapters).map(|v| *v).collect();
    let outlet_joltage = 0;
    let device_joltage = sorted_adapters.iter().max().unwrap() + 3;
    let devices = iter::once(&outlet_joltage)
        .chain(sorted_adapters.iter())
        .chain(iter::once(&device_joltage));

    for (x, y) in devices.tuple_windows() {
        match y - x {
            1 => by_1 += 1,
            2 => by_2 += 1,
            3 => by_3 += 1,
            _ => return Err(()),
        }
    }

    Ok((by_1, by_2, by_3))
}

fn main() {
    let adapters: Vec<i32> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    if let Ok((by_1, _, by_3)) = joltage_differences(&adapters) {
        println!("{}", by_1 * by_3);
    }
}
