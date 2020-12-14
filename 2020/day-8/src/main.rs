use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opcode {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Opcode, Self::Err> {
        match s {
            "acc" => Ok(Opcode::Acc),
            "jmp" => Ok(Opcode::Jmp),
            "nop" => Ok(Opcode::Nop),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Opcode::Acc => "acc",
            Opcode::Jmp => "jmp",
            Opcode::Nop => "nop",
            _ => "<?>",
        })
    }
}

struct OpParseError {
}

impl From<ParseIntError> for OpParseError {
    fn from(_: ParseIntError) -> OpParseError {
        OpParseError{}
    }
}

#[derive(Clone, Copy, Debug)]
struct Op {
    opcode: Opcode,
    value: i32,
}

impl FromStr for Op {
    type Err = OpParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut splits = line.split(" ");
        Ok(Op {
            opcode: splits.next()
                .ok_or(Self::Err{})?
                .parse()
                .or_else(|_| Err(Self::Err{}))?,
            value: splits.next()
                .ok_or(Self::Err{})?
                .parse()
                .or_else(|_| Err(Self::Err{}))?,
        })
    }
}

fn run(ops: &Vec<Op>) -> Result<i32, i32> {
    let mut acc: i32 = 0;
    let mut pc: usize = 0;
    let mut visited: HashSet<usize> = HashSet::new();

    while pc < ops.len() {
        if visited.contains(&pc) {
            return Err(acc);
        } else {
            visited.insert(pc);
        }

        let op = &ops[pc];
        match op.opcode {
            Opcode::Acc => {
                acc += op.value;
                pc += 1;
            },
            Opcode::Jmp => {
                pc = (pc as i32 + op.value) as usize;
            },
            Opcode::Nop => {
                pc += 1;
            },
        }
    }

    Ok(acc)
}

fn toggle_opcode(op: &Op) -> Op {
    Op {
        opcode: match op.opcode {
            Opcode::Jmp => Opcode::Nop,
            Opcode::Nop => Opcode::Jmp,
            _ => op.opcode,
        },
        value: op.value,
    }
}

fn main() {
    let mut ops: Vec<Op> = io::stdin().lock()
        .lines()
        .map(|line| line.unwrap().parse().ok().unwrap())
        .collect();

    for i in 0..ops.len() {
        ops[i] = toggle_opcode(&ops[i]);
        match run(&ops) {
            Ok(result) => {
                println!("Got result: {}", result);
                return;
            },
            Err(last_value) => {
                println!("Infinite loop detected, last accumulator value: {}", last_value);
            }
        }
        ops[i] = toggle_opcode(&ops[i]);
    }
}