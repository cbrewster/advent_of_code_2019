mod computer;

use computer::{parse_program, Computer, ComputerState, Program};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);

    let result1 = part1(program.clone());
    println!("Part 1: {}", result1);

    let result2 = part2(program);
    println!("Part 2: {}", result2);
}

fn part1(program: Program) -> isize {
    let mut queue: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
    let mut computers = vec![Computer::new(program.clone()); 50];

    for (i, computer) in computers.iter_mut().enumerate() {
        computer.push_input(i as isize);
        computer.execute();
    }

    loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            match computer.execute() {
                ComputerState::InputRequired => match queue.entry(i).or_default().pop() {
                    None => computer.push_input(-1),
                    Some((x, y)) => {
                        computer.push_input(x);
                        computer.push_input(y);
                    }
                },
                ComputerState::Output(address) => {
                    let x = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state!"),
                    };
                    let y = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state!"),
                    };
                    if address == 255 {
                        return y;
                    } else {
                        queue.entry(address as usize).or_default().push((x, y));
                    }
                }
                _ => panic!("Unexpected computer state!"),
            }
        }
    }
}

fn part2(program: Program) -> isize {
    let mut queue: HashMap<usize, Vec<(isize, isize)>> = HashMap::new();
    let mut computers = vec![Computer::new(program.clone()); 50];

    for (i, computer) in computers.iter_mut().enumerate() {
        computer.push_input(i as isize);
        computer.execute();
    }

    let mut nat_packet = (0, 0);
    let mut sent_to_zero = HashSet::new();
    let mut waiting_set = HashSet::new();

    loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            match computer.execute() {
                ComputerState::InputRequired => match queue.entry(i).or_default().pop() {
                    None => {
                        computer.push_input(-1);
                        waiting_set.insert(i);
                    }
                    Some((x, y)) => {
                        waiting_set.remove(&i);
                        computer.push_input(x);
                        computer.push_input(y);
                    }
                },
                ComputerState::Output(address) => {
                    let x = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state!"),
                    };
                    let y = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state!"),
                    };
                    if address == 255 {
                        nat_packet = (x, y);
                    } else {
                        queue.entry(address as usize).or_default().push((x, y));
                    }
                }
                _ => panic!("Unexpected computer state!"),
            }
        }

        if waiting_set.len() == 50 && queue.values().all(|v| v.is_empty()) {
            if sent_to_zero.contains(&nat_packet.1) {
                return nat_packet.1;
            }
            queue.entry(0).or_default().push(nat_packet);
            sent_to_zero.insert(nat_packet.1);
        }
    }
}
