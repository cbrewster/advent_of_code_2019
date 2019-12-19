mod computer;

use computer::{parse_program, Computer, ComputerState, Program};

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);

    let result1 = part1(program.clone());
    println!("Part 1: {}", result1);

    let result2 = part2(program);
    println!("Part 2: {}", result2);
}


fn part1(program: Program) -> usize {
    let mut sum = 0;
    for y in 0..50 {
        for x in 0..50 {
            let mut computer = Computer::new(program.clone());
            computer.push_input(x);
            computer.push_input(y);

            if let ComputerState::Output(output) = computer.execute() {
                sum += output as usize;
            }
        }
    }
    sum
}

fn lookup(program: Program, x: isize, y: isize) -> isize {
    if x < 0 || y < 0 {
        return 0;
    }

    let mut computer = Computer::new(program.clone());
    computer.push_input(x);
    computer.push_input(y);
    match computer.execute() {
        ComputerState::Output(output) => output,
        _ => panic!("Unexpected computer output"),
    }
}

fn part2(program: Program) -> usize {
    // This is pretty slow and we skip the first 20 rows since they have no laser output which causes an infinite loop.
    let mut y = 20;
    loop {
        let mut x = 0;
        loop {
            let output = lookup(program.clone(), x, y);
            if output == 1 {
                let corner = lookup(program.clone(), x + 99, y - 99);
                if corner == 1 {
                    return (x * 10000 + y - 99) as usize;
                }
                break;
            }
            x += 1;
        }
        y += 1;
    }
}
