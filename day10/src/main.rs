use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

fn main() {
    let input = include_str!("../input.txt");
    let asteroids = parse_input(input);

    let (best, count) = asteroids
        .iter()
        .map(|asteroid| (*asteroid, compute_line_of_sight(*asteroid, &asteroids)))
        .max_by_key(|(_, count)| *count)
        .unwrap();

    println!("Part 1: {}", count);

    let order = compute_asteroid_destruction_order(best, &asteroids);
    let (x, y) = order[199];
    println!("Part 2: {:?}", x * 100 + y);
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (row_index, row) in input.trim().lines().enumerate() {
        for (col_index, col) in row.chars().enumerate() {
            if col == '#' {
                output.push((col_index, row_index));
            }
        }
    }
    output
}

fn compute_line_of_sight(asteroid: (usize, usize), asteroids: &[(usize, usize)]) -> usize {
    let mut found_angles = HashSet::new();

    for other in asteroids {
        if asteroid == *other {
            continue;
        }
        let x = (other.0 as isize - asteroid.0 as isize) as f64;
        let y = (other.1 as isize - asteroid.1 as isize) as f64;
        let angle = y.atan2(x);
        // This is super hacky :)
        // f64 does not implement Hash so we convert to a usize and multiply by a large number to
        // reduce precision loss. Possibly this could be replaced with a rational number type or something.
        found_angles.insert((angle * 10000.0) as usize);
    }

    found_angles.len()
}

fn compute_asteroid_destruction_order(
    center: (usize, usize),
    asteroids: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let mut buckets: HashMap<usize, Vec<(usize, (usize, usize))>> = HashMap::new();

    for other in asteroids {
        if center == *other {
            continue;
        }
        let x = (other.0 as isize - center.0 as isize) as f64;
        let y = (other.1 as isize - center.1 as isize) as f64;
        // dbg!(x, y);
        let angle = y.atan2(x) + PI / 2.0;
        let dist = ((x * x + y * y) * 1000.0) as usize;
        // This is super hacky :)
        // f64 does not implement Hash so we convert to a usize and multiply by a large number to
        // reduce precision loss. Possibly this could be replaced with a rational number type or something.
        buckets
            .entry((angle * 10000.0) as usize)
            .and_modify(|v| v.push((dist, *other)))
            .or_insert(vec![(dist, *other)]);
    }

    let mut ordered_buckets: Vec<(usize, Vec<(usize, (usize, usize))>)> =
        buckets.into_iter().collect();
    ordered_buckets.sort_by_key(|bucket| bucket.0);
    for (_, bucket) in ordered_buckets.iter_mut() {
        bucket.sort_by_key(|(dist, _)| *dist);
        bucket.reverse();
    }

    let mut output = Vec::new();

    loop {
        let mut popped = false;
        for (angle, bucket) in ordered_buckets.iter_mut() {
            if let Some((_, asteroid)) = bucket.pop() {
                // println!("Popping at {}º", (*angle as f64 / 10000.0) * (180.0 / 3.14));
                output.push(asteroid);
                popped = true;
            }
        }
        if !popped {
            break;
        }
    }

    output
}
