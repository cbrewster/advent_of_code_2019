fn main() {
    let input = include_str!("../input.txt");

    let program = parse_program(input);

    let mut part_1_input = program.clone();
    // Program correct
    part_1_input[1] = 12;
    part_1_input[2] = 2;
    let result_1 = execute_program(part_1_input);

    println!("Part 1: {:?}", result_1[0]);

    // Part 2, need to determine [1], [2] values
    let (noun, verb) = part_2(&program);
    println!("Part 2: {:?}", noun * 100 + verb);
}

fn parse_program(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|num| num.parse::<usize>().unwrap())
        .collect()
}

fn part_2(program: &[usize]) -> (usize, usize) {
    // Try brute force first... it works :)
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.to_owned();
            program[1] = noun;
            program[2] = verb;
            if execute_program(program)[0] == 19690720 {
                return (noun, verb);
            }
        }
    }
    panic!("No result found!");
}

fn execute_program(mut program: Vec<usize>) -> Vec<usize> {
    let mut ip = 0;
    loop {
        match program[ip] {
            99 => break,
            1 => {
                let in1 = program[ip + 1];
                let in2 = program[ip + 2];
                let out = program[ip + 3];
                program[out] = program[in1] + program[in2];
                ip += 4;
            }
            2 => {
                let in1 = program[ip + 1];
                let in2 = program[ip + 2];
                let out = program[ip + 3];
                program[out] = program[in1] * program[in2];
                ip += 4;
            }
            _ => panic!("Unexpected opcode!"),
        }
    }
    program
}
