use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    R,
    L,
    U,
    D,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Move {
    direction: Direction,
    distance: isize,
}

impl FromStr for Move {
    // Sad errors
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distance = s[1..].parse::<isize>().unwrap();
        let direction = match s.chars().next().unwrap() {
            'R' => Direction::R,
            'L' => Direction::L,
            'U' => Direction::U,
            'D' => Direction::D,
            _ => return Err(()),
        };
        Ok(Move {
            direction,
            distance,
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let line_1 = lines.next().expect("No first line");
    let line_2 = lines.next().expect("No second line");

    let (line_1_map, line_1_set) = compute_path_set(parse_moves(line_1));
    let (line_2_map, line_2_set) = compute_path_set(parse_moves(line_2));
    let intersections = line_1_set.intersection(&line_2_set);

    let min_distance = intersections.map(|coord| coord.distance()).min().unwrap();
    println!("Part 1: {}", min_distance);

    let intersections = line_1_set.intersection(&line_2_set);
    let min_distance = intersections
        .map(|coord| line_1_map.get(&coord).unwrap() + line_2_map.get(&coord).unwrap())
        .min()
        .unwrap();
    println!("Part 2: {}", min_distance);
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .split(",")
        .map(|line| line.parse::<Move>().unwrap())
        .collect()
}

fn compute_path_set(moves: Vec<Move>) -> (HashMap<Coordinate, usize>, HashSet<Coordinate>) {
    let mut path = HashSet::new();
    let mut dists = HashMap::new();
    let mut position = Coordinate { x: 0, y: 0 };
    let mut dist = 0;
    for m in moves {
        for _ in 0..m.distance {
            match m.direction {
                Direction::R => position.x += 1,
                Direction::L => position.x -= 1,
                Direction::U => position.y += 1,
                Direction::D => position.y -= 1,
            }
            path.insert(position);
            dist += 1;
            dists.entry(position).or_insert(dist);
        }
    }
    (dists, path)
}
