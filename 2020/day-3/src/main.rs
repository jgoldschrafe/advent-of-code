use std::io;
use std::io::prelude::*;

fn parse_tree_mask(s: &str) -> Vec<bool> {
    s.chars()
        .map(|c| c == '#')
        .collect()
}

fn trees_encountered(tree_coords: &Vec<Vec<bool>>, step: usize, slope: usize) -> i64 {
    let mut result: i64 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    while y < tree_coords.len() {
        let row = &tree_coords[y];

        if row[x as usize] {
            result += 1;
        }

        x = (x + slope) % row.len();
        y += step;
    }

    result
}

fn main() {
    let tree_coords: Vec<Vec<bool>> = io::stdin().lock()
        .lines()
        .map(|line| parse_tree_mask(&line.unwrap()))
        .collect();

    let slope_reciprocals = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let result: i64 = slope_reciprocals.iter()
        .fold(1, |acc, (step, slope)| acc * trees_encountered(&tree_coords, *step, *slope));

    println!("{}", result);
}