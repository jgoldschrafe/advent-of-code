use std::io;
use std::io::prelude::*;

fn parse_tree_mask(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| c == '#')
        .collect()
}

fn trees_encountered(tree_coords: &Vec<Vec<bool>>) -> i32 {
    let mut result = 0;
    let mut x = 0;

    for row in tree_coords.iter() {
        if row[x] {
            result += 1;
        }

        x = (x + 3) % row.len();
    }

    result
}

fn main() {
    let tree_coords: Vec<Vec<bool>> = io::stdin().lock()
        .lines()
        .map(|line| parse_tree_mask(&line.unwrap()))
        .collect();

    let result = trees_encountered(&tree_coords);
    println!("{}", result);
}