use std::io;
use std::io::prelude::*;

fn patch(program: &mut Vec<i32>) {
    program[1] = 12;
    program[2] = 2;
}

fn run(program: &mut Vec<i32>) {
    let mut pc = 0;

    loop {
        // We can have an opcode 99 as the last location in the program,
        // with no data after it. Exit early if we see it.
        let opcode = program[pc];
        if opcode == 99 {
            return;
        }

        let ax = program[pc + 1] as usize;
        let bx = program[pc + 2] as usize;
        let dest = program[pc + 3] as usize;

        program[dest] = match opcode {
            1 => program[ax] + program[bx],
            2 => program[ax] * program[bx],
            _ => program[dest],
        };

        pc += 4;
    }
}

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let mut program: Vec<i32> = buf.trim_end()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    patch(&mut program);
    run(&mut program);

    println!("{:?}", program[0]);
}
