fn main() {
    let input = include_str!("../input.txt");

    let (min, max) = parse_range(input);
    let result_1 = part_1(min, max);

    println!("Part 1: {:?}", result_1);

    let result_2 = part_2(min, max);
    println!("Part 2: {:?}", result_2);
}

fn part_1(min: usize, max: usize) -> usize {
    let mut correct_count = 0;
    for password in min..=max {
        if check_password_1(password, min, max) {
            correct_count += 1;
        }
    }
    correct_count
}

fn part_2(min: usize, max: usize) -> usize {
    let mut correct_count = 0;
    for password in min..=max {
        if check_password_2(password, min, max) {
            correct_count += 1;
        }
    }
    correct_count
}

fn parse_range(input: &str) -> (usize, usize) {
    let mut nums = input.split("-").map(|num| num.parse::<usize>().unwrap());

    (nums.next().unwrap(), nums.next().unwrap())
}

fn check_password_1(password: usize, min: usize, max: usize) -> bool {
    if password < min || password > max {
        return false;
    }

    let dig1 = password % 10;
    let dig2 = (password / 10) % 10;
    let dig3 = (password / 100) % 10;
    let dig4 = (password / 1000) % 10;
    let dig5 = (password / 10000) % 10;
    let dig6 = (password / 100000) % 10;

    // At least one pair of duplicate adjacent digits
    let list = [dig6, dig5, dig4, dig3, dig2, dig1];
    if !list.windows(2).any(|pair| pair[0] == pair[1]) {
        return false;
    }

    // Never decreasing
    if dig6 > dig5 || dig5 > dig4 || dig4 > dig3 || dig3 > dig2 || dig2 > dig1 {
        return false;
    }

    true
}

fn check_password_2(password: usize, min: usize, max: usize) -> bool {
    if password < min || password > max {
        return false;
    }

    let dig1 = password % 10;
    let dig2 = (password / 10) % 10;
    let dig3 = (password / 100) % 10;
    let dig4 = (password / 1000) % 10;
    let dig5 = (password / 10000) % 10;
    let dig6 = (password / 100000) % 10;

    // At least one pair of duplicate adjacent digits
    let list = [dig6, dig5, dig4, dig3, dig2, dig1];
    let mut repeated_digits = 1;
    for i in 1..6 {
        if list[i - 1] == list[i] {
            repeated_digits += 1;
        } else {
            if repeated_digits == 2 {
                break;
            }
            repeated_digits = 1;
        }
    }
    if repeated_digits != 2 {
        return false;
    }

    // Never decreasing
    if dig6 > dig5 || dig5 > dig4 || dig4 > dig3 || dig3 > dig2 || dig2 > dig1 {
        return false;
    }

    true
}
