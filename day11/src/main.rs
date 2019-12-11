mod computer;

use computer::{parse_program, Computer};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Black,
}

impl Direction {
    fn turn(&self, input: usize) -> Direction {
        match input {
            // left
            0 => match *self {
                Direction::Up => Direction::Left,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
                Direction::Left => Direction::Down,
            },

            // right
            1 => match *self {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Right => Direction::Down,
                Direction::Left => Direction::Up,
            },

            _ => panic!("Bad turn direction"),
        }
    }

    fn move_dir(&self, current: (isize, isize)) -> (isize, isize) {
        match *self {
            Direction::Up => (current.0, current.1 + 1),
            Direction::Down => (current.0, current.1 - 1),
            Direction::Right => (current.0 + 1, current.1),
            Direction::Left => (current.0 - 1, current.1),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);
    let result = part1(program.clone());
    println!("Part 1: {}", result);

    println!("Part 2:");
    part2(program);
}

fn part1(program: Vec<isize>) -> usize {
    let mut computer = Computer::new(program);
    let mut painted_tiles = HashMap::new();
    let mut position = (0, 0);
    let mut direction = Direction::Up;
    computer.push_input(0);
    while let Some(paint) = computer.execute() {
        let color = match paint {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color"),
        };
        painted_tiles.insert(position, color);
        direction = direction.turn(computer.execute().expect("No second output") as usize);
        position = direction.move_dir(position);
        match painted_tiles.get(&position).unwrap_or(&Color::Black) {
            Color::White => computer.push_input(1),
            Color::Black => computer.push_input(0),
        }
    }
    painted_tiles.len()
}

fn part2(program: Vec<isize>) {
    let mut computer = Computer::new(program);
    let mut painted_tiles = HashMap::new();
    let mut position = (0, 0);
    painted_tiles.insert(position, Color::White);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut min_x = 0;
    let mut min_y = 0;
    let mut direction = Direction::Up;
    computer.push_input(1);
    while let Some(paint) = computer.execute() {
        let color = match paint {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("Invalid color"),
        };
        if color == Color::White {
            max_x = max_x.max(position.0);
            max_y = max_y.max(position.1);
            min_x = min_x.min(position.0);
            min_y = min_y.min(position.1);
        }
        painted_tiles.insert(position, color);
        direction = direction.turn(computer.execute().expect("No second output") as usize);
        position = direction.move_dir(position);
        match painted_tiles.get(&position).unwrap_or(&Color::Black) {
            Color::White => computer.push_input(1),
            Color::Black => computer.push_input(0),
        }
    }

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if painted_tiles.get(&(x, y)).unwrap_or(&Color::Black) == &Color::White {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
