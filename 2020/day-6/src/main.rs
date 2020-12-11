use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn collect_answers(responses: &Vec<String>) -> HashMap<char, i32> {
    let mut result = HashMap::new();
    for response in responses {
        for c in response.chars() {
            *result.entry(c).or_insert(0) += 1;
        }
    }

    result
}

fn count_full_answers(responses: &Vec<String>) -> usize {
    let mut result = HashMap::new();
    for response in responses {
        for c in response.chars() {
            *result.entry(c).or_insert(0) += 1;
        }
    }

    result.iter()
        .filter(|&(_, v)| *v == responses.len())
        .count()
}

fn main() -> io::Result<()> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;

    let groups: Vec<Vec<String>> = buf.split("\n\n")
        .map(|group| {
            group.split("\n")
                .map(|response| response.to_string())
                .collect()
        })
        .collect();

    let total: usize = groups.iter()
        .map(|grp| collect_answers(grp).keys().len())
        .sum();

    let total_full: usize = groups.iter()
        .map(|grp| count_full_answers(&grp))
        .sum();

    println!("total questions answered = {}", total);
    println!("total questions answered by everyone in group = {}", total_full);

    Ok(())
}
