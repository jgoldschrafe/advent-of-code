use std::io;
use std::io::prelude::*;

fn required_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    match fuel {
        std::i32::MIN..=0 => 0,
        _ => fuel + required_fuel(fuel),
    }
}

fn main() {
    let module_mass: Vec<i32> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let module_fuel: Vec<i32> = module_mass.into_iter()
        .map(required_fuel)
        .collect();

    let total_fuel: i32 = module_fuel.into_iter().sum();
    println!("{}", total_fuel);
}
