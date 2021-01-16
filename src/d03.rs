use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'U' => Direction::Up,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'D' => Direction::Down,
            _ => panic!("invalid character"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    direction: Direction,
    amount: usize,
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if let Ok((c, amount)) = scan_fmt!(s, "{[URLD]}{d}", char, usize) {
            let direction = Direction::from_char(c);
            return Instruction { direction, amount };
        }
        panic!("invalid format")
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[aoc_generator(day3)]
pub fn parse_input(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|l| l.split(',').map(Instruction::from_str).collect())
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(instructions: &[Vec<Instruction>]) -> usize {
    let mut current_coord = Coord { x: 0, y: 0 };

    let mut visited: HashSet<Coord> = HashSet::new();
    let wire1_instructions = &instructions[0];

    for ins in wire1_instructions {
        match ins.direction {
            Direction::Up => {
                for _ in 1..=ins.amount {
                    current_coord.y += 1 as i32;
                    visited.insert(current_coord);
                }
            }
            Direction::Down => {
                for _ in 1..=ins.amount {
                    current_coord.y -= 1 as i32;
                    visited.insert(current_coord);
                }
            }
            Direction::Right => {
                for _ in 1..=ins.amount {
                    current_coord.x += 1 as i32;
                    visited.insert(current_coord);
                }
            }
            Direction::Left => {
                for _ in 1..=ins.amount {
                    current_coord.x -= 1 as i32;
                    visited.insert(current_coord);
                }
            }
        }
    }

    current_coord = Coord { x: 0, y: 0 };
    let mut crossed: HashSet<Coord> = HashSet::new();
    let wire2_instructions = &instructions[1];

    for ins in wire2_instructions {
        match ins.direction {
            Direction::Up => {
                for _ in 1..=ins.amount {
                    current_coord.y += 1 as i32;
                    if visited.contains(&current_coord) {
                        crossed.insert(current_coord);
                    }
                }
            }
            Direction::Down => {
                for _ in 1..=ins.amount {
                    current_coord.y -= 1 as i32;
                    if visited.contains(&current_coord) {
                        crossed.insert(current_coord);
                    }
                }
            }
            Direction::Right => {
                for _ in 1..=ins.amount {
                    current_coord.x += 1 as i32;
                    if visited.contains(&current_coord) {
                        crossed.insert(current_coord);
                    }
                }
            }
            Direction::Left => {
                for _ in 1..=ins.amount {
                    current_coord.x -= 1 as i32;
                    if visited.contains(&current_coord) {
                        crossed.insert(current_coord);
                    }
                }
            }
        }
    }
    crossed.iter().map(|c| c.x.abs() + c.y.abs()).min().unwrap() as usize
}

#[aoc(day3, part2)]
pub fn part2(instructions: &[Vec<Instruction>]) -> usize {
    let mut current_coord = Coord { x: 0, y: 0 };
    let mut steps: usize = 0;

    let mut visited: HashMap<Coord, usize> = HashMap::new();
    let wire1_instructions = &instructions[0];
    for ins in wire1_instructions {
        match ins.direction {
            Direction::Up => {
                for _ in 1..=ins.amount {
                    current_coord.y += 1 as i32;
                    steps += 1;
                    visited.insert(current_coord, steps);
                }
            }
            Direction::Down => {
                for _ in 1..=ins.amount {
                    current_coord.y -= 1 as i32;
                    steps += 1;
                    visited.insert(current_coord, steps);
                }
            }
            Direction::Right => {
                for _ in 1..=ins.amount {
                    current_coord.x += 1 as i32;
                    steps += 1;
                    visited.insert(current_coord, steps);
                }
            }
            Direction::Left => {
                for _ in 1..=ins.amount {
                    current_coord.x -= 1 as i32;
                    steps += 1;
                    visited.insert(current_coord, steps);
                }
            }
        }
    }

    current_coord = Coord { x: 0, y: 0 };
    steps = 0;
    let mut crossed: Vec<usize> = Vec::new();
    let wire2_instructions = &instructions[1];

    for ins in wire2_instructions {
        match ins.direction {
            Direction::Up => {
                for _ in 1..=ins.amount {
                    current_coord.y += 1 as i32;
                    steps += 1;
                    if visited.contains_key(&current_coord) {
                        crossed.push(visited.get(&current_coord).unwrap() + steps);
                    }
                }
            }
            Direction::Down => {
                for _ in 1..=ins.amount {
                    current_coord.y -= 1 as i32;
                    steps += 1;
                    if visited.contains_key(&current_coord) {
                        crossed.push(visited.get(&current_coord).unwrap() + steps);
                    }
                }
            }
            Direction::Right => {
                for _ in 1..=ins.amount {
                    current_coord.x += 1 as i32;
                    steps += 1;
                    if visited.contains_key(&current_coord) {
                        crossed.push(visited.get(&current_coord).unwrap() + steps);
                    }
                }
            }
            Direction::Left => {
                for _ in 1..=ins.amount {
                    current_coord.x -= 1 as i32;
                    steps += 1;
                    if visited.contains_key(&current_coord) {
                        crossed.push(visited.get(&current_coord).unwrap() + steps);
                    }
                }
            }
        }
    }
    *crossed.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d03_p1_t1() {
        assert_eq!(6, part1(&parse_input(SAMPLE1)));
        assert_eq!(159, part1(&parse_input(SAMPLE2)));
        assert_eq!(135, part1(&parse_input(SAMPLE3)));
    }

    const SAMPLE1: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    const SAMPLE2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";

    const SAMPLE3: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
}
