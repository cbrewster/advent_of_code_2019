mod computer;

use computer::{parse_program, Computer, ComputerState};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn as_int(&self) -> isize {
        match self {
            Direction::North => 1,
            Direction::South => 2,
            Direction::West => 3,
            Direction::East => 4,
        }
    }

    fn as_delta(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, 1),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
            Direction::East => (1, 0),
        }
    }
}

#[derive(Clone)]
struct RepairDroid {
    location: (isize, isize),
    computer: Computer,
}

// impl RepairDroid {
//     fn find_unexplored_empty_spaces
// }

// struct Maze {
//     map: HashMap<(isize, isize), Tile>,
//     program: Program,
// }

// impl Maze {
//     fn new(program: Program) -> Maze {
//         let mut map = HashMap::new();
//         map.insert((0, 0), Tile::Empty);
//         let computer = Computer::new(program);
//         Maze { map, computer }
//     }

//     fn find_oxygen_system(&mut self) -> (isize, isize) {
//         loop {
//             self.print_map();
//             println!("Move:");
//             let mut line = String::new();
//             std::io::stdin().read_line(&mut line).unwrap();
//             let cur_dir = match line.trim() {
//                 "N" => Direction::North,
//                 "S" => Direction::South,
//                 "W" => Direction::West,
//                 "E" => Direction::East,
//                 _ => panic!("Bad input"),
//             };
//             self.computer.push_input(cur_dir.as_int());
//             let result = match self.computer.execute() {
//                 ComputerState::Output(output) => output,
//                 _ => panic!("Computer program borked"),
//             };

//             match result {
//                 0 => {
//                     let delta = cur_dir.as_delta();
//                     let wall_pos = (self.location.0 + delta.0, self.location.1 + delta.1);
//                     self.map.insert(wall_pos, Tile::Wall);
//                 }
//                 1 => {
//                     let delta = cur_dir.as_delta();
//                     self.location.0 += delta.0;
//                     self.location.1 += delta.1;
//                     self.map.insert(self.location, Tile::Empty);
//                 }
//                 2 => {
//                     let delta = cur_dir.as_delta();
//                     self.location.0 += delta.0;
//                     self.location.1 += delta.1;
//                     self.map.insert(self.location, Tile::OxygenSystem);
//                     return self.location;
//                 }
//                 _ => panic!("Unexpected output from computer"),
//             }
//         }
//     }

//     fn print_map(&self) {
//         let min_x = self.map.keys().map(|(x, _)| *x).min().unwrap();
//         let max_x = self.map.keys().map(|(x, _)| *x).max().unwrap();
//         let min_y = self.map.keys().map(|(_, y)| *y).min().unwrap();
//         let max_y = self.map.keys().map(|(_, y)| *y).max().unwrap();
//         println!("Map:");
//         for y in (min_y..=max_y).rev() {
//             for x in min_x..=max_x {
//                 match self.map.get(&(x, y)) {
//                     None => print!(" "),
//                     Some(Tile::Empty) => print!("."),
//                     Some(Tile::Wall) => print!("#"),
//                     Some(Tile::OxygenSystem) => print!("O"),
//                 }
//             }
//             println!();
//         }
//     }
// }

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);
    let mut map = HashMap::new();
    let computer = Computer::new(program);
    explore_maze(computer, (0, 0), &mut map);
    let result1 = part1(&map);
    println!("Part 1: {}", result1);

    let result2 = part2(&map);
    println!("Part 2: {}", result2);
}

fn part1(map: &HashMap<(isize, isize), Tile>) -> usize {
    // Pathfinding
    let mut open = Vec::new();
    open.push((0, 0));
    let mut closed = HashSet::new();
    let mut steps = 0;
    let mut found = false;

    while !open.is_empty() {
        steps += 1;
        let considering = std::mem::replace(&mut open, Vec::new());
        for position in considering {
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ] {
                let delta = direction.as_delta();
                let new_pos = (position.0 + delta.0, position.1 + delta.1);
                if closed.contains(&new_pos) {
                    continue;
                }

                match map.get(&new_pos) {
                    Some(&Tile::Empty) => open.push(new_pos),
                    Some(&Tile::OxygenSystem) => found = true,
                    _ => {}
                }
            }
            closed.insert(position);
        }
        if found {
            break;
        }
    }

    steps
}

fn part2(map: &HashMap<(isize, isize), Tile>) -> usize {
    let (start, _) = map
        .iter()
        .find(|(_, tile)| **tile == Tile::OxygenSystem)
        .unwrap();

    // Pathfinding
    let mut open = Vec::new();
    open.push(*start);
    let mut closed = HashSet::new();
    let mut steps = 0;

    while !open.is_empty() {
        steps += 1;
        let considering = std::mem::replace(&mut open, Vec::new());
        for position in considering {
            for direction in &[
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ] {
                let delta = direction.as_delta();
                let new_pos = (position.0 + delta.0, position.1 + delta.1);
                if closed.contains(&new_pos) {
                    continue;
                }

                match map.get(&new_pos) {
                    Some(&Tile::Empty) | Some(&Tile::OxygenSystem) => open.push(new_pos),
                    _ => {}
                }
            }
            closed.insert(position);
        }
    }

    steps - 1
}

fn move_direction(computer: &mut Computer, direction: Direction) -> Tile {
    computer.push_input(direction.as_int());
    match computer.execute() {
        ComputerState::Output(0) => Tile::Wall,
        ComputerState::Output(1) => Tile::Empty,
        ComputerState::Output(2) => Tile::OxygenSystem,
        _ => panic!("Unexpected computer state"),
    }
}

fn explore_maze(
    computer: Computer,
    position: (isize, isize),
    map: &mut HashMap<(isize, isize), Tile>,
) {
    for direction in &[
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ] {
        let mut new_computer = computer.clone();
        let delta = direction.as_delta();
        let new_pos = (position.0 + delta.0, position.1 + delta.1);
        if map.contains_key(&new_pos) {
            continue;
        }
        let tile = move_direction(&mut new_computer, *direction);
        map.insert(new_pos, tile);
        match tile {
            Tile::Empty => explore_maze(new_computer, new_pos, map),
            _ => {}
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &HashMap<(isize, isize), Tile>) {
    let min_x = map.keys().map(|(x, _)| *x).min().unwrap();
    let max_x = map.keys().map(|(x, _)| *x).max().unwrap();
    let min_y = map.keys().map(|(_, y)| *y).min().unwrap();
    let max_y = map.keys().map(|(_, y)| *y).max().unwrap();
    println!("Map:");
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                print!("S");
                continue;
            }
            match map.get(&(x, y)) {
                None => print!(" "),
                Some(Tile::Empty) => print!("."),
                Some(Tile::Wall) => print!("#"),
                Some(Tile::OxygenSystem) => print!("O"),
            }
        }
        println!();
    }
}
