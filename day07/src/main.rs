mod computer;

use computer::Computer;

struct Amplifier {
    computer: Computer,
}

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);

    let result = part_1(program.clone());
    println!("Part 1: {}", result);

    let result = part_2(program.clone());
    println!("Part 2: {}", result);
}

fn part_1(program: Vec<isize>) -> isize {
    let mut max_out = 0;

    let input = [0, 1, 2, 3, 4];
    let permutations = generate_permutations(&input);

    for permutation in permutations {
        let mut amp_a = Amplifier::new(program.clone(), permutation[0]);
        let mut amp_b = Amplifier::new(program.clone(), permutation[1]);
        let mut amp_c = Amplifier::new(program.clone(), permutation[2]);
        let mut amp_d = Amplifier::new(program.clone(), permutation[3]);
        let mut amp_e = Amplifier::new(program.clone(), permutation[4]);

        let a_out = amp_a.execute(0).unwrap();
        let b_out = amp_b.execute(a_out).unwrap();
        let c_out = amp_c.execute(b_out).unwrap();
        let d_out = amp_d.execute(c_out).unwrap();
        let e_out = amp_e.execute(d_out).unwrap();

        if e_out > max_out {
            max_out = e_out;
        }
    }

    max_out
}

fn part_2(program: Vec<isize>) -> isize {
    let mut max_out = 0;

    let input = [5, 6, 7, 8, 9];
    let permutations = generate_permutations(&input);

    for permutation in permutations {
        let mut amp_a = Amplifier::new(program.clone(), permutation[0]);
        let mut amp_b = Amplifier::new(program.clone(), permutation[1]);
        let mut amp_c = Amplifier::new(program.clone(), permutation[2]);
        let mut amp_d = Amplifier::new(program.clone(), permutation[3]);
        let mut amp_e = Amplifier::new(program.clone(), permutation[4]);

        let mut feedback = 0;
        loop {
            let a_out = match amp_a.execute(feedback) {
                Some(val) => val,
                None => break,
            };
            let b_out = amp_b.execute(a_out).expect("No output for b");
            let c_out = amp_c.execute(b_out).expect("No output for c");
            let d_out = amp_d.execute(c_out).expect("No output for d");
            feedback = amp_e.execute(d_out).expect("No output for e");
        }

        if feedback > max_out {
            max_out = feedback;
        }
    }

    max_out
}

fn generate_permutations(input: &[isize]) -> Vec<Vec<isize>> {
    let mut permutations = vec![];
    heap_permutation(&mut input.to_owned(), input.len(), &mut permutations);
    permutations
}

fn heap_permutation(current: &mut Vec<isize>, size: usize, permutations: &mut Vec<Vec<isize>>) {
    if size == 1 {
        permutations.push(current.clone());
        return;
    }

    for i in 0..size {
        heap_permutation(current, size - 1, permutations);

        if size % 2 == 1 {
            current.swap(0, size - 1);
        } else {
            current.swap(i, size - 1)
        }
    }
}

fn parse_program(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

impl Amplifier {
    fn new(program: Vec<isize>, phase_setting: isize) -> Amplifier {
        let mut computer = Computer::new(program);
        computer.push_input(phase_setting);
        Amplifier { computer }
    }

    fn execute(&mut self, input: isize) -> Option<isize> {
        self.computer.push_input(input);
        self.computer.execute();
        self.computer.pop_output()
    }
}
