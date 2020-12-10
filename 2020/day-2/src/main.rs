#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

//#[derive(Debug)]
struct CheckedPassword {
    range_start: i32,
    range_end: i32,
    c: char,
    password: String,
}

impl CheckedPassword {
    fn is_valid_by_occurrences(&self) -> bool {
        let occurrences: i32 = self.password.chars()
            .map(|c| (c == self.c) as i32)
            .sum();

        occurrences >= self.range_start && occurrences <= self.range_end
    }

    fn is_valid_by_positions(&self) -> bool {
        let mut at_first = false;
        let mut at_last = false;

        for (i, c) in self.password.chars().enumerate() {
            let pos = i as i32 + 1;
            if c == self.c && pos == self.range_start {
                at_first = true;
            }

            if c == self.c && pos == self.range_end {
                at_last = true;
                break;
            }
        }

        at_first ^ at_last
    }
}

impl FromStr for CheckedPassword {
    type Err = ParseIntError;

    fn from_str(pwd: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        }
        
        let caps = RE.captures(pwd).unwrap();
        Ok(CheckedPassword {
            range_start: caps.get(1).unwrap().as_str().parse().unwrap(),
            range_end: caps.get(2).unwrap().as_str().parse().unwrap(),
            c: caps.get(3).unwrap().as_str().chars().next().unwrap(),
            password: caps.get(4).unwrap().as_str().to_string(),
        })
    }
}

fn main() {
    let passwords: Vec<CheckedPassword> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let num_valid_passwords: i32 = passwords.iter()
        //.map(|pwd| pwd.is_valid_by_occurrences() as i32)
        .map(|pwd| pwd.is_valid_by_positions() as i32)
        .sum();

    println!("{}", num_valid_passwords);
}
