fn main() {
    // I am so lazy :)
    let input = include_str!("../input.txt");

    let result_1 = part_1(input);
    println!("Part 1: {:?}", result_1);

    let result_2 = part_2(input);
    println!("Part 2: {:?}", result_2);
}

fn part_1(input: &str) -> isize {
    input
        .lines()
        .map(|num| (num.parse::<isize>().unwrap() / 3) - 2)
        .sum()
}

fn part_2(input: &str) -> isize {
    input
        .lines()
        .map(|num| num.parse::<isize>().unwrap())
        .map(compute_fuel)
        .sum()
}

fn compute_fuel(mass: isize) -> isize {
    let mut res = 0;
    let mut fuel = mass;
    while fuel > 0 {
        fuel = (fuel / 3) - 2;
        if fuel > 0 {
            res += fuel;
        }
    }
    res
}
