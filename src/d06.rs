use std::collections::HashMap;

pub struct Orbits {
    orbits: HashMap<String, String>,
}

impl Orbits {
    fn count_orbits(&self, start_planet: &str, count: i32) -> i32 {
        if "COM" == start_planet {
            return count;
        }
        self.count_orbits(self.orbits.get(start_planet).unwrap(), count + 1)
    }

    fn find_chain_to_com(&self, start_planet: &str, chain: &mut Vec<String>) {
        if "COM" == start_planet {
            return;
        }
        let next_planet = self.orbits.get(start_planet).unwrap();
        chain.push(next_planet.clone());
        self.find_chain_to_com(next_planet, chain);
    }
}

#[aoc_generator(day6)]
pub fn parse_input(input: &str) -> Orbits {
    let orbits = input
        .lines()
        .map(|l| {
            let mut o = l.split(')');
            let o0 = o.next().unwrap();
            (o.next().unwrap().to_string(), o0.to_string())
        })
        .collect();

    Orbits { orbits }
}

#[aoc(day6, part1)]
pub fn part1(orbits: &Orbits) -> i32 {
    orbits
        .orbits
        .keys()
        .into_iter()
        .map(|k| orbits.count_orbits(k, 0))
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(orbits: &Orbits) -> usize {
    let mut you_chain: Vec<String> = Vec::new();
    orbits.find_chain_to_com("YOU", &mut you_chain);

    let mut san_chain: Vec<String> = Vec::new();
    orbits.find_chain_to_com("SAN", &mut san_chain);

    loop {
        let next_you = you_chain.pop().unwrap();
        let next_san = san_chain.pop().unwrap();

        if next_san != next_you {
            you_chain.push(next_you);
            san_chain.push(next_san);
            break;
        }
    }

    you_chain.len() + san_chain.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d06_p1_t1() {
        let orbits = parse_input(SAMPLE1);
        assert_eq!(42, part1(&orbits))
    }

    #[test]
    fn d06_p2_t1() {
        let orbits = parse_input(SAMPLE2);
        assert_eq!(4, part2(&orbits))
    }

    const SAMPLE1: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const SAMPLE2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
}
