use itertools::Itertools;

use super::d05::intcomp::{VMState, VM};

#[aoc_generator(day7)]
pub fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(initial_memory: &[i64]) -> i64 {
    let phase_settings: Vec<i64> = vec![0, 1, 2, 3, 4];

    phase_settings
        .iter()
        .permutations(5)
        .map(|p| {
            let mut vm_a = VM::new(&initial_memory);
            vm_a.input.push(0);
            vm_a.input.push(**p.get(0).unwrap());
            vm_a.run();

            let mut vm_b = VM::new(&initial_memory);
            vm_b.input.push(vm_a.output.pop().unwrap());
            vm_b.input.push(**p.get(1).unwrap());
            vm_b.run();

            let mut vm_c = VM::new(&initial_memory);
            vm_c.input.push(vm_b.output.pop().unwrap());
            vm_c.input.push(**p.get(2).unwrap());
            vm_c.run();

            let mut vm_d = VM::new(&initial_memory);
            vm_d.input.push(vm_c.output.pop().unwrap());
            vm_d.input.push(**p.get(3).unwrap());
            vm_d.run();

            let mut vm_e = VM::new(&initial_memory);
            vm_e.input.push(vm_d.output.pop().unwrap());
            vm_e.input.push(**p.get(4).unwrap());
            vm_e.run();

            vm_e.output.pop().unwrap()
        })
        .max()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(initial_memory: &[i64]) -> i64 {
    let phase_settings: Vec<i64> = vec![5, 6, 7, 8, 9];
    phase_settings
        .iter()
        .permutations(5)
        .map(|p| amplify(&initial_memory, p))
        .max()
        .unwrap()
}

fn amplify(initial_memory: &[i64], phase_settings: Vec<&i64>) -> i64 {
    let mut vm_a = VM::new(&initial_memory);
    vm_a.input.push(*phase_settings[0]);
    vm_a.run();

    let mut vm_b = VM::new(&initial_memory);
    vm_b.input.push(*phase_settings[1]);
    vm_b.run();

    let mut vm_c = VM::new(&initial_memory);
    vm_c.input.push(*phase_settings[2]);
    vm_c.run();

    let mut vm_d = VM::new(&initial_memory);
    vm_d.input.push(*phase_settings[3]);
    vm_d.run();

    let mut vm_e = VM::new(&initial_memory);
    vm_e.input.push(*phase_settings[4]);
    vm_e.run();

    // prime
    vm_e.output.push(0);

    while vm_e.state != VMState::Halted {
        vm_a.input.push(vm_e.output.pop().unwrap());
        vm_a.run();

        vm_b.input.push(vm_a.output.pop().unwrap());
        vm_b.run();

        vm_c.input.push(vm_b.output.pop().unwrap());
        vm_c.run();

        vm_d.input.push(vm_c.output.pop().unwrap());
        vm_d.run();

        vm_e.input.push(vm_d.output.pop().unwrap());
        vm_e.run();
    }
    vm_e.output.pop().unwrap()
}
