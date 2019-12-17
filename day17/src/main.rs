mod computer;

use computer::{parse_program, Computer, ComputerState, Program};
use std::collections::HashMap;

type Map = HashMap<(isize, isize), Node>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Node {
    Scaffold,
    Empty,
    Robot(Direction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, pos: (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::North => (pos.0, pos.1 - 1),
            Direction::South => (pos.0, pos.1 + 1),
            Direction::West => (pos.0 - 1, pos.1),
            Direction::East => (pos.0 + 1, pos.1),
        }
    }

    fn turn_left(&self) -> Direction {
        match *self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::East => Direction::North,
        }
    }

    fn turn_right(&self) -> Direction {
        match *self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::East => Direction::South,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
    Forward(usize),
}

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);

    let map = build_map(program.clone());
    let result1 = part1(&map);
    println!("Part 1: {}", result1);

    let result2 = part2(&map, program);
    println!("Part 2: {}", result2);
}

fn part1(map: &Map) -> isize {
    map.iter()
        .filter(|(_, node)| **node == Node::Scaffold)
        .filter(|(&(x, y), _)| {
            map.get(&(x + 1, y)) == Some(&Node::Scaffold)
                && map.get(&(x - 1, y)) == Some(&Node::Scaffold)
                && map.get(&(x, y + 1)) == Some(&Node::Scaffold)
                && map.get(&(x, y - 1)) == Some(&Node::Scaffold)
        })
        .map(|((x, y), _)| x * y)
        .sum()
}

fn part2(map: &Map, mut program: Program) -> isize {
    let moves = find_moves(map);
    let mut moves_str = moves_to_string(&moves);
    moves_str.push(',');
    moves.len();
    &moves_str;
    moves_str.chars().count();

    let (main_seq, a, b, c) = find_main_program(&moves_str).unwrap();

    // Enable manual control mode
    program[0] = 2;
    let mut computer = Computer::new(program);
    send_string(&mut computer, &main_seq);
    send_string(&mut computer, &a);
    send_string(&mut computer, &b);
    send_string(&mut computer, &c);
    send_string(&mut computer, "n\n");

    let mut last_output = 0;
    while let ComputerState::Output(output) = computer.execute() {
        last_output = output;
    }
    last_output
}

fn send_string(computer: &mut Computer, string: &str) {
    for c in string.chars() {
        computer.push_input(c as isize);
    }
    computer.push_input(10);
}

fn find_main_program(moves_str: &str) -> Option<(String, String, String, String)> {
    for a in 2..20 {
        for b in 2..20 {
            for c in 2..20 {
                let a = &moves_str[0..a];
                if a.chars().last() != Some(',') {
                    continue;
                }
                let new_str = moves_str.replace(a, "");
                let b = &new_str[0..b];
                if b.chars().last() != Some(',') {
                    continue;
                }

                let new_str = new_str.replace(b, "");
                let c = &new_str[0..c];
                if c.chars().last() != Some(',') {
                    continue;
                }

                let new_str = new_str.replace(c, "");
                let final_str = new_str.replace(",", "");

                if final_str.len() == 0 {
                    let mut main_procedure = moves_str.to_owned();
                    main_procedure = main_procedure.replace(a, "A,");
                    main_procedure = main_procedure.replace(b, "B,");
                    main_procedure = main_procedure.replace(c, "C,");
                    return Some((
                        main_procedure[0..main_procedure.len() - 1].into(),
                        a[0..a.len() - 1].into(),
                        b[0..b.len() - 1].into(),
                        c[0..c.len() - 1].into(),
                    ));
                }
            }
        }
    }
    None
}

fn find_moves(map: &Map) -> Vec<Move> {
    let (start_pos, start_dir) = map
        .iter()
        .find_map(|(pos, node)| {
            if let Node::Robot(dir) = *node {
                Some((pos, dir))
            } else {
                None
            }
        })
        .unwrap();

    let mut cur_pos = *start_pos;
    let mut cur_dir = start_dir;
    let mut moves = vec![];

    let mut forward_count = 0;
    loop {
        let new_pos = cur_dir.apply(cur_pos);
        if let Some(&Node::Scaffold) = map.get(&new_pos) {
            cur_pos = new_pos;
            forward_count += 1;
            continue;
        }

        if forward_count > 0 {
            moves.push(Move::Forward(forward_count));
            forward_count = 0;
        }

        if let Some(&Node::Scaffold) = map.get(&cur_dir.turn_left().apply(cur_pos)) {
            cur_dir = cur_dir.turn_left();
            moves.push(Move::Left);
        } else if let Some(&Node::Scaffold) = map.get(&cur_dir.turn_right().apply(cur_pos)) {
            cur_dir = cur_dir.turn_right();
            moves.push(Move::Right);
        } else {
            break;
        }
    }

    moves
}

fn moves_to_string(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| match m {
            Move::Forward(num) => num.to_string(),
            Move::Left => "L".to_owned(),
            Move::Right => "R".to_owned(),
        })
        .collect::<Vec<_>>()
        .join(",")
}

fn build_map(program: Program) -> Map {
    let mut computer = Computer::new(program);

    let mut map = HashMap::new();
    let mut y = 0;
    let mut x = 0;

    while let ComputerState::Output(output) = computer.execute() {
        let node = match output as u8 as char {
            '.' => Node::Empty,
            '#' => Node::Scaffold,
            '^' => Node::Robot(Direction::North),
            'v' => Node::Robot(Direction::South),
            '<' => Node::Robot(Direction::West),
            '>' => Node::Robot(Direction::East),
            '\n' => {
                y += 1;
                x = 0;
                continue;
            }
            _ => panic!("Invalid computer output"),
        };

        map.insert((x, y), node);

        x += 1;
    }
    map
}
