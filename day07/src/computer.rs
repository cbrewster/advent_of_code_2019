use std::collections::VecDeque;

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

pub struct Computer {
    memory: Vec<isize>,
    ip: usize,
    inputs: VecDeque<isize>,
    outputs: VecDeque<isize>,
}

impl Computer {
    pub fn new(program: Vec<isize>) -> Computer {
        Computer {
            memory: program,
            ip: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, value: isize) {
        self.inputs.push_front(value);
    }

    pub fn pop_output(&mut self) -> Option<isize> {
        self.outputs.pop_back()
    }

    pub fn execute(&mut self) {
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
                let input = self.inputs.pop_back().unwrap();
                let addr = self.parse_parameter(1);
                self.write_parameter(addr, input);
                self.ip += 2;
            }
            // Output
            4 => {
                let output = self.parse_parameter(1);
                self.outputs.push_front(self.read_parameter(output));
                self.ip += 2;
                // Return execution back to operator
                return true;
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
