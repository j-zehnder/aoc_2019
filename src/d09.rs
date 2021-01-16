#[aoc_generator(day9)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(initial_memory: &[i64]) -> i64 {
    let mut vm = intcomp::VM::new(&initial_memory);
    vm.input.push(1);
    vm.run();
    vm.output.pop().unwrap()
}

#[aoc(day9, part2)]
pub fn part2(initial_memory: &[i64]) -> i64 {
    let mut vm = intcomp::VM::new(&initial_memory);
    vm.input.push(2);
    vm.run();
    vm.output.pop().unwrap()
}

pub mod intcomp {
    #[derive(Debug, PartialEq)]
    pub enum VMState {
        Ready,
        Paused,
        Halted,
    }

    #[derive(Debug, PartialEq)]
    enum ParameterMode {
        Positional = 0,
        Immediate = 1,
        Relative = 2,
    }

    impl ParameterMode {
        fn from_i64(value: i64) -> ParameterMode {
            match value {
                0 => ParameterMode::Positional,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
                _ => panic!("unknown parameter mode"),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum OpCode {
        ADDI = 1,
        MULI = 2,
        INPT = 3,
        OUTP = 4,
        TJMP = 5,
        FJMP = 6,
        ISLT = 7,
        ISEQ = 8,
        RELB = 9,
        HALT = 99,
    }

    impl OpCode {
        fn from_cmdcode(cmdcode: i64) -> OpCode {
            match cmdcode % 100 {
                1 => OpCode::ADDI,
                2 => OpCode::MULI,
                3 => OpCode::INPT,
                4 => OpCode::OUTP,
                5 => OpCode::TJMP,
                6 => OpCode::FJMP,
                7 => OpCode::ISLT,
                8 => OpCode::ISEQ,
                9 => OpCode::RELB,
                99 => OpCode::HALT,
                _ => panic!("unknown value"),
            }
        }
    }

    pub struct VM {
        memory: Vec<i64>,
        ins_ptr: usize,
        relb_ptr: usize,
        pub state: VMState,
        pub input: Vec<i64>,
        pub output: Vec<i64>,
    }

    impl VM {
        pub fn new(initial_memory: &[i64]) -> Self {
            VM {
                memory: initial_memory.to_vec(),
                ins_ptr: 0,
                relb_ptr: 0,
                state: VMState::Ready,
                input: Vec::new(),
                output: Vec::new(),
            }
        }

        pub fn run(&mut self) -> &VMState {
            if self.state == VMState::Paused && !self.input.is_empty() {
                self.state = VMState::Ready;
            }

            while self.state == VMState::Ready {
                self.step();
            }

            &self.state
        }

        fn get_address_for_param(&mut self, cmdcode: i64, param: usize) -> usize {
            let mode_offset = [0, 100, 1000, 10000][param];
            let mode = ParameterMode::from_i64((cmdcode / mode_offset) % 10);
            let ptr = match mode {
                ParameterMode::Positional => self.memory[self.ins_ptr + param] as usize,
                ParameterMode::Immediate => self.ins_ptr + param,
                ParameterMode::Relative => {
                    (self.relb_ptr as i64 + self.memory[self.ins_ptr + param]) as usize
                }
            };

            if ptr > self.memory.len() {
                let len = ptr - self.memory.len() + 1000;
                let additional_memory = vec![0 as i64; len];
                self.memory.extend_from_slice(&*additional_memory);
            }

            ptr
        }

        fn extend_memory(&mut self) {
            if self.relb_ptr > self.memory.len() {
                let len = self.relb_ptr - self.memory.len() + 1000;
                let additional_memory = vec![0 as i64; len];
                self.memory.extend_from_slice(&*additional_memory);
            }

            if self.ins_ptr > self.memory.len() {
                let len = self.ins_ptr - self.memory.len() + 1000;
                let additional_memory = vec![0 as i64; len];
                self.memory.extend_from_slice(&*additional_memory);
            }
        }

        fn step(&mut self) -> &VMState {
            if self.state == VMState::Halted || self.state == VMState::Paused {
                return &self.state;
            }

            let cmdcode = self.memory[self.ins_ptr];
            let opcode = OpCode::from_cmdcode(cmdcode);

            self.extend_memory();

            match opcode {
                OpCode::ADDI => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    let b_ptr = self.get_address_for_param(cmdcode, 2);
                    let c_ptr = self.get_address_for_param(cmdcode, 3);

                    self.memory[c_ptr] = self.memory[a_ptr] + self.memory[b_ptr];
                    self.ins_ptr += 4;
                }
                OpCode::MULI => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    let b_ptr = self.get_address_for_param(cmdcode, 2);
                    let c_ptr = self.get_address_for_param(cmdcode, 3);
                    self.memory[c_ptr] = self.memory[a_ptr] * self.memory[b_ptr];
                    self.ins_ptr += 4;
                }
                OpCode::INPT => {
                    if !self.input.is_empty() {
                        let a_ptr = self.get_address_for_param(cmdcode, 1);
                        self.memory[a_ptr] = self.input.pop().expect("expected input");
                        self.ins_ptr += 2;
                    } else {
                        self.state = VMState::Paused;
                    }
                }
                OpCode::OUTP => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    self.output.push(self.memory[a_ptr]);
                    self.ins_ptr += 2;
                }
                OpCode::TJMP => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    if self.memory[a_ptr] != 0 {
                        let b_ptr = self.get_address_for_param(cmdcode, 2);
                        self.ins_ptr = self.memory[b_ptr] as usize;
                    } else {
                        self.ins_ptr += 3;
                    }
                }
                OpCode::FJMP => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    if self.memory[a_ptr] == 0 {
                        let b_ptr = self.get_address_for_param(cmdcode, 2);
                        self.ins_ptr = self.memory[b_ptr] as usize;
                    } else {
                        self.ins_ptr += 3;
                    }
                }
                OpCode::ISLT => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    let b_ptr = self.get_address_for_param(cmdcode, 2);
                    let c_ptr = self.get_address_for_param(cmdcode, 3);
                    if self.memory[a_ptr] < self.memory[b_ptr] {
                        self.memory[c_ptr] = 1;
                    } else {
                        self.memory[c_ptr] = 0;
                    }
                    self.ins_ptr += 4;
                }
                OpCode::ISEQ => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    let b_ptr = self.get_address_for_param(cmdcode, 2);
                    let c_ptr = self.get_address_for_param(cmdcode, 3);
                    if self.memory[a_ptr] == self.memory[b_ptr] {
                        self.memory[c_ptr] = 1;
                    } else {
                        self.memory[c_ptr] = 0;
                    }
                    self.ins_ptr += 4;
                }
                OpCode::RELB => {
                    let a_ptr = self.get_address_for_param(cmdcode, 1);
                    self.relb_ptr = (self.relb_ptr as i64 + self.memory[a_ptr]) as usize;
                    self.ins_ptr += 2;
                }
                OpCode::HALT => {
                    self.state = VMState::Halted;
                }
            }

            &self.state
        }
    }
}
