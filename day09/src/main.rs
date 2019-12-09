mod computer;

use computer::Computer;

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);
    let mut computer = Computer::new(program.clone());
    computer.push_input(1);

    println!("Part 1:");
    while let Some(output) = computer.execute() {
        println!("Output: {}", output);
    }

    let mut computer = Computer::new(program);
    computer.push_input(2);

    println!("Part 2:");
    while let Some(output) = computer.execute() {
        println!("Output: {}", output);
    }
}

fn parse_program(input: &str) -> Vec<isize> {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}
