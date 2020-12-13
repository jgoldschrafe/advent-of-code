use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::io;
use std::io::prelude::*;

use petgraph::prelude::*;

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
                "0" | "no" => None,
                _ => Some(Adjacency {
                    parent_name: parent_name.clone(),
                    child_name: child_name.to_string(),
                    quantity: qty_str.parse().unwrap(),
                })
            }
        })
        .collect()
}

fn create_graph(adjacencies: &Vec<Adjacency>) -> DiGraphMap<i32, i32> {
    let mut g = DiGraphMap::new();
    let mut name_index: HashMap<String, i32> = HashMap::new();
    // hacky shortcut: let's just always have shiny gold be ID 0
    name_index.insert("shiny gold".to_string(), 0);

    let mut idx = 1;
    for adj in adjacencies {
        let parent_id = match name_index.entry(adj.parent_name.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(idx);
                idx += 1;
                idx - 1
            },
        };

        let child_id = match name_index.entry(adj.child_name.clone()) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                entry.insert(idx);
                idx += 1;
                idx - 1
            }
        };

        g.add_edge(parent_id, child_id, adj.quantity);
    }

    g
}

fn bag_ways(graph: &DiGraphMap<i32, i32>, node: i32) -> i32 {
    graph.neighbors(node)
        .map(|neigh| {
            let weight = graph[(node, neigh)];
            weight + weight * bag_ways(graph, neigh)
        })
        .sum()
}

fn main() {
    let adjacencies: Vec<Adjacency> = io::stdin().lock()
        .lines()
        .map(|line| parse_line(&line.unwrap()))
        .flatten()
        .collect();

    let graph = create_graph(&adjacencies);
    let result = bag_ways(&graph, 0);

    println!("{}", result);
}