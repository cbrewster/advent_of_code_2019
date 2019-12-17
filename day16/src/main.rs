fn main() {
    let input = include_str!("../input.txt");
    let signal = parse_input(input);
    let result1 = part1(&signal);
    println!("Part 1: {}", result1);
    let result2 = part2(&signal);
    println!("Part 2: {}", result2);
}

fn parse_input(input: &str) -> Vec<isize> {
    input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as isize)
        .collect()
}

// This runs a bit slow in debug-mode, there is probably some optimization that could be done here
fn part1(signal: &[isize]) -> isize {
    let mut cur_signal = signal.to_owned();

    for _ in 0..100 {
        cur_signal = (0..cur_signal.len())
            .map(|i| {
                cur_signal
                    .iter()
                    .zip(pattern(i as usize))
                    .map(|(x, y)| x * y)
                    .sum::<isize>()
                    .abs()
                    % 10
            })
            .collect();
    }

    cur_signal[0..8]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| num * 10_isize.pow(i as u32))
        .sum()
}

fn part2(signal: &[isize]) -> isize {
    let offset: isize = signal[0..7]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| num * 10_isize.pow(i as u32))
        .sum();

    let mut cur_signal = signal
        .repeat(10_000)
        .into_iter()
        .skip(offset as usize)
        .collect::<Vec<_>>();

    for _ in 0..100 {
        // The offset is generally past the half-way point so all coefficients after the place are 1
        // We can start at the end and work up to the front and alter the signal in place while tracking the sum
        let mut sum = 0;
        for i in (0..cur_signal.len()).rev() {
            let val = cur_signal[i];
            cur_signal[i] = (val + sum).abs() % 10;
            sum += val;
        }
    }

    cur_signal[0..8]
        .iter()
        .rev()
        .enumerate()
        .map(|(i, num)| num * 10_isize.pow(i as u32))
        .sum()
}

fn pattern(position: usize) -> impl Iterator<Item = isize> {
    let a = vec![0isize; position + 1];
    let b = vec![1isize; position + 1];
    let c = vec![0isize; position + 1];
    let d = vec![-1isize; position + 1];
    a.into_iter()
        .chain(b.into_iter())
        .chain(c.into_iter())
        .chain(d.into_iter())
        .cycle()
        .skip(1)
}
