use std::collections::VecDeque;

pub type Program = Vec<isize>;

pub fn parse_program(input: &str) -> Program {
    input
        .trim()
        .split(",")
        .map(|num| num.parse::<isize>().unwrap())
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum ParameterMode {
    Immediate,
    Position,
    Relative,
}

#[derive(Debug, Copy, Clone)]
struct Parameter {
    mode: ParameterMode,
    value: isize,
}

#[derive(Clone)]
pub struct Computer {
    memory: Program,
    ip: usize,
    relative_base_offset: usize,
    inputs: VecDeque<isize>,
}

enum StepResult {
    Continue,
    Stop,
    Output(isize),
    InputRequired,
}

pub enum ComputerState {
    Halt,
    Output(isize),
    InputRequired,
}

impl Computer {
    pub fn new(mut program: Program) -> Computer {
        // cheap hack :)
        program.extend_from_slice(&[0; 1000]);
        Computer {
            memory: program,
            ip: 0,
            relative_base_offset: 0,
            inputs: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, value: isize) {
        self.inputs.push_front(value);
    }

    pub fn execute(&mut self) -> ComputerState {
        loop {
            match self.step() {
                StepResult::Stop => break,
                StepResult::Output(output) => return ComputerState::Output(output),
                StepResult::InputRequired => return ComputerState::InputRequired,
                StepResult::Continue => {}
            }
        }
        ComputerState::Halt
    }

    fn parse_parameter(&self, position: usize) -> Parameter {
        let mode = (self.memory[self.ip] as usize) / (10 * 10usize.pow(position as u32)) % 10;
        let mode = if mode == 1 {
            ParameterMode::Immediate
        } else if mode == 2 {
            ParameterMode::Relative
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
            ParameterMode::Relative => {
                self.memory[(self.relative_base_offset as isize + parameter.value) as usize]
            }
        }
    }

    fn write_parameter(&mut self, parameter: Parameter, value: isize) {
        match parameter.mode {
            ParameterMode::Immediate => panic!("Writing to immediate mode!"),
            ParameterMode::Position => self.memory[parameter.value as usize] = value,
            ParameterMode::Relative => {
                self.memory[(self.relative_base_offset as isize + parameter.value) as usize] =
                    value;
            }
        }
    }

    fn step(&mut self) -> StepResult {
        match self.memory[self.ip] % 100 {
            99 => return StepResult::Stop,
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
            // Input
            3 => {
                if let Some(input) = self.inputs.pop_back() {
                    let addr = self.parse_parameter(1);
                    self.write_parameter(addr, input);
                    self.ip += 2;
                } else {
                    return StepResult::InputRequired;
                }
            }
            // Output
            4 => {
                let param = self.parse_parameter(1);
                let output = self.read_parameter(param);
                self.ip += 2;
                // Return execution back to operator
                return StepResult::Output(output);
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
            // Set relative base offset
            9 => {
                let param = self.parse_parameter(1);
                self.relative_base_offset =
                    (self.relative_base_offset as isize + self.read_parameter(param)) as usize;
                self.ip += 2;
            }
            _ => panic!("Unexpected opcode: {}", self.memory[self.ip]),
        }
        StepResult::Continue
    }
}
