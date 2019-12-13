mod computer;

use computer::{parse_program, Computer, ComputerState, Program};
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum TileId {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileId {
    fn from_int(input: isize) -> TileId {
        match input {
            0 => TileId::Empty,
            1 => TileId::Wall,
            2 => TileId::Block,
            3 => TileId::Paddle,
            4 => TileId::Ball,
            _ => panic!("Invalid tile input"),
        }
    }
}

struct ArcadeBox {
    game: Program,
}

impl ArcadeBox {
    fn new(game: Program) -> ArcadeBox {
        ArcadeBox { game }
    }

    fn play(&mut self) -> isize {
        let mut program = self.game.clone();
        // enter 2 coins
        program[0] = 2;
        let mut computer = Computer::new(program);
        let mut score = 0;
        let mut tiles = HashMap::new();
        let mut ball_location = (0, 0);
        let mut paddle_location = (0, 0);
        loop {
            match computer.execute() {
                ComputerState::Halt => return score,
                ComputerState::InputRequired => {
                    let block_count = count_blocks(&tiles);
                    if block_count == 0 {
                        return score;
                    }

                    let input = match ball_location.0.cmp(&paddle_location.0) {
                        Ordering::Equal => 0,
                        Ordering::Greater => 1,
                        Ordering::Less => -1,
                    };

                    computer.push_input(input);
                }
                ComputerState::Output(x) => {
                    let y = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state."),
                    };
                    let id = match computer.execute() {
                        ComputerState::Output(output) => output,
                        _ => panic!("Unexpected computer state."),
                    };
                    if x == -1 && y == 0 {
                        score = id;
                    } else {
                        let id = TileId::from_int(id);
                        tiles.insert((x, y), id);
                        match id {
                            TileId::Ball => ball_location = (x, y),
                            TileId::Paddle => paddle_location = (x, y),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

fn count_blocks(tiles: &HashMap<(isize, isize), TileId>) -> usize {
    tiles.values().filter(|id| **id == TileId::Block).count()
}

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);
    let result1 = part1(program.clone());
    println!("Part 1: {}", result1);

    let result2 = part2(program.clone());
    println!("Part 2: {}", result2);
}

fn part1(program: Program) -> usize {
    let mut computer = Computer::new(program);
    let mut blocks = 0;
    while let ComputerState::Output(_x) = computer.execute() {
        let _y = match computer.execute() {
            ComputerState::Output(output) => output,
            _ => panic!("Unexpected computer state."),
        };
        let id = match computer.execute() {
            ComputerState::Output(output) => output,
            _ => panic!("Unexpected computer state."),
        };
        let id = TileId::from_int(id);
        if id == TileId::Block {
            blocks += 1;
        }
    }
    blocks
}

fn part2(program: Program) -> isize {
    let mut arcade = ArcadeBox::new(program);
    arcade.play()
}
