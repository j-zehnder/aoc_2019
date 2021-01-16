#[aoc_generator(day2)]
pub fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(initial_memory: &[i32]) -> i32 {
    let mut vm = VM::new(initial_memory);

    vm.memory[1] = 12;
    vm.memory[2] = 2;

    vm.run();
    vm.memory[0]
}

const P2_TARGET: i32 = 19690720;

#[aoc(day2, part2)]
pub fn part2(initial_memory: &[i32]) -> i32 {
    for i in 0..100 {
        for ii in 0..100 {
            let mut vm = VM::new(initial_memory);
            vm.memory[1] = i;
            vm.memory[2] = ii;
            vm.run();

            if vm.memory[0] == P2_TARGET {
                return 100 * i + ii;
            }
        }
    }
    panic!("not found")
}

// -------------------------------------------------------------------------------------------------
#[derive(Debug, PartialEq)]
pub enum VMState {
    Ready,
    Paused,
    Halted,
}

#[derive(Debug, PartialEq)]
enum OpCode {
    ADDI = 1,
    MULI = 2,
    HALT = 99,
}

impl OpCode {
    fn from_cmdcode(value: i32) -> OpCode {
        let value = value % 100;
        match value {
            1 => OpCode::ADDI,
            2 => OpCode::MULI,
            99 => OpCode::HALT,
            _ => panic!("unknown value: {}", value),
        }
    }
}

struct VM {
    memory: Vec<i32>,
    ins_ptr: usize,
    state: VMState,
}

impl VM {
    fn new(initial_memory: &[i32]) -> Self {
        VM {
            memory: initial_memory.to_vec(),
            ins_ptr: 0,
            state: VMState::Ready,
        }
    }

    /// run until paused or halted
    fn run(&mut self) -> &VMState {
        while self.state == VMState::Ready {
            self.step();
        }
        &self.state
    }

    fn step(&mut self) {
        let cmdcode = self.memory[self.ins_ptr];
        let opcode = OpCode::from_cmdcode(cmdcode);
        match opcode {
            OpCode::HALT => self.state = VMState::Halted,
            OpCode::ADDI => {
                let a_ptr = self.memory[self.ins_ptr + 1] as usize;
                let b_ptr = self.memory[self.ins_ptr + 2] as usize;
                let c_ptr = self.memory[self.ins_ptr + 3] as usize;
                self.memory[c_ptr] = self.memory[a_ptr] + self.memory[b_ptr];
                self.ins_ptr += 4;
            }
            OpCode::MULI => {
                let a_ptr = self.memory[self.ins_ptr + 1] as usize;
                let b_ptr = self.memory[self.ins_ptr + 2] as usize;
                let c_ptr = self.memory[self.ins_ptr + 3] as usize;
                self.memory[c_ptr] = self.memory[a_ptr] * self.memory[b_ptr];
                self.ins_ptr += 4;
            }
        }
    }
}
// -------------------------------------------------------------------------------------------------
