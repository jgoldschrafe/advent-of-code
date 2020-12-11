/*
 * INCOMPLETE
 */

use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Adjacency {
    parent_name: String,
    child_name: String,
    quantity: i32,
}

fn parse_line(line: &str) -> Vec<Adjacency> {
    let mut splits = line.split(" bags contain ");
    let parent_name = splits.next().unwrap().to_string();
    splits.next().unwrap()
        .replace(" bags", "")
        .replace(" bag", "")
        .replace(".", "")
        .split(", ")
        .map(|child_def| {
            let mut sn = child_def.splitn(2, ' ');
            (sn.next().unwrap(), sn.next().unwrap())
        })
        .filter_map(|(qty_str, child_name)| {
            match qty_str {
                "no" => None,
                _ => Some(Adjacency {
                    parent_name: parent_name.clone(),
                    child_name: child_name.to_string(),
                    quantity: qty_str.parse().unwrap(),
                })
            }
        })
        .collect()
}

fn create_adjacency_matrix(adjacencies: &Vec<Adjacency>) -> HashMap<(String, String), i32> {
    let mut mat = HashMap::new();

    for adj in adjacencies {
        mat.insert((adj.parent_name.clone(), adj.child_name.clone()), adj.quantity);
    }

    mat
}

fn main() {
    let adjacencies: Vec<Adjacency> = io::stdin().lock()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .flatten()
        .collect();

    let mat = create_adjacency_matrix(&adjacencies);
    println!("{:#?}", mat);
}