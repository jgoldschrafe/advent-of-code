use itertools::Itertools;
use std::io;
use std::io::prelude::*;

fn calculate_seat_id(ops: &String) -> Option<i32> {
    let binary_str = ops
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");

    i32::from_str_radix(&binary_str, 2).ok()
}

fn find_missing_seat(seat_ids: &Vec<i32>) -> Option<i32> {
    for (a, b) in seat_ids.iter().sorted().tuple_windows() {
        if b - a > 1 {
            return Some(b - 1);
        }
    }

    None
}

fn main() {
    let seat_ids: Vec<i32> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap())
        .filter_map(|ops| calculate_seat_id(&ops))
        .collect();

    let gapped_seat_id = find_missing_seat(&seat_ids).unwrap();
    
    println!("max seat ID = {}", seat_ids.iter().max().unwrap());
    println!("gapped seat ID = {}", gapped_seat_id)
}