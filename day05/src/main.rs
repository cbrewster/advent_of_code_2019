use std::io::{stdin, stdout, Write};

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Immediate,
    Position,
}

#[derive(Debug, Copy, Clone)]
struct Parameter {
    mode: ParameterMode,
    value: isize,
}

struct Computer {
    memory: Vec<isize>,
    ip: usize,
}

fn main() {
    let input = include_str!("../input.txt");
    let program = parse_program(input);

    println!("Starting part 1...");
    let mut computer = Computer::new(program);
    computer.execute();
}

fn parse_program(input: &str) -> Vec<isize> {
    input
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

impl Computer {
    fn new(program: Vec<isize>) -> Computer {
        Computer {
            memory: program,
            ip: 0,
        }
    }

    fn execute(&mut self) {
        loop {
            if self.step() {
                break;
            }
        }
    }

    fn parse_parameter(&self, position: usize) -> Parameter {
        let mode = (self.memory[self.ip] as usize) / (10 * 10usize.pow(position as u32)) % 10;
        let mode = if mode == 1 {
            ParameterMode::Immediate
        } else {
            ParameterMode::Position
        };
        let value = self.memory[self.ip + position];
        Parameter { mode, value }
    }

    fn read_parameter(&self, parameter: Parameter) -> isize {
        match parameter.mode {
            ParameterMode::Immediate => parameter.value,
            ParameterMode::Position => self.memory[parameter.value as usize],
        }
    }

    fn write_parameter(&mut self, parameter: Parameter, value: isize) {
        match parameter.mode {
            ParameterMode::Immediate => panic!("Writing to immediate mode!"),
            ParameterMode::Position => self.memory[parameter.value as usize] = value,
        }
    }

    fn step(&mut self) -> bool {
        match self.memory[self.ip] % 100 {
            99 => return true,
            // Add
            1 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                let param3 = self.parse_parameter(3);

                self.write_parameter(
                    param3,
                    self.read_parameter(param1) + self.read_parameter(param2),
                );
                self.ip += 4;
            }
            // Mul
            2 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                let param3 = self.parse_parameter(3);

                self.write_parameter(
                    param3,
                    self.read_parameter(param1) * self.read_parameter(param2),
                );
                self.ip += 4;
            }
            // Store
            3 => {
                let input = get_user_input("Computer input");
                let addr = self.parse_parameter(1);
                self.write_parameter(addr, input);
                self.ip += 2;
            }
            // Output
            4 => {
                let output = self.parse_parameter(1);
                println!("Output: {}", self.read_parameter(output));
                self.ip += 2;
            }
            // Jump if true
            5 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                if self.read_parameter(param1) != 0 {
                    self.ip = self.read_parameter(param2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            // Jump if false
            6 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                if self.read_parameter(param1) == 0 {
                    self.ip = self.read_parameter(param2) as usize;
                } else {
                    self.ip += 3;
                }
            }
            // Less than
            7 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                let param3 = self.parse_parameter(3);

                let result = if self.read_parameter(param1) < self.read_parameter(param2) {
                    1
                } else {
                    0
                };

                self.write_parameter(param3, result);

                self.ip += 4;
            }
            // Equals
            8 => {
                let param1 = self.parse_parameter(1);
                let param2 = self.parse_parameter(2);
                let param3 = self.parse_parameter(3);

                let result = if self.read_parameter(param1) == self.read_parameter(param2) {
                    1
                } else {
                    0
                };

                self.write_parameter(param3, result);

                self.ip += 4;
            }

            _ => panic!("Unexpected opcode: {}", self.memory[self.ip]),
        }
        false
    }
}

fn get_user_input(prompt: &str) -> isize {
    print!("{}: ", prompt);
    stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Could not get input");
    input.trim().parse().expect("invalid user input")
}
