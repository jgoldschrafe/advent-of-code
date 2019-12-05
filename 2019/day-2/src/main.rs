use std::io;

fn run(program: &mut Vec<i32>) -> Result<i32, &str> {
    let size = program.len();
    let mut pc = 0;

    loop {
        let opcode = program[pc];
        if opcode == 99 {
            return Ok(program[0]);
        }

        let ax = program[pc + 1] as usize;
        let bx = program[pc + 2] as usize;
        let dest = program[pc + 3] as usize;

        if ax > size || bx > size || dest > size {
            return Err("out of bounds");
        }

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

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut patched = program.clone();
            patched[1] = noun;
            patched[2] = verb;

            if let Ok(result @ 19690720) = run(&mut patched) {
                let total = 100 * noun + verb;
                println!("{}", total);
                return;
            }
        }
    }
}
