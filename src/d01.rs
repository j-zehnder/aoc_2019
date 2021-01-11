#[aoc_generator(day1)]
pub fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(module_masses: &[u32]) -> u32 {
    module_masses.iter().map(calculate_fuel).sum()
}

#[aoc(day1, part2)]
pub fn part2(module_masses: &[u32]) -> u32 {
    module_masses.iter().map(calculate_total_fuel).sum()
}

fn calculate_fuel(mass: &u32) -> u32 {
    let fuel: i32 = *mass as i32 / 3;
    let fuel = fuel - 2;
    if fuel < 0 { 0 } else { fuel as u32 }
}

fn calculate_total_fuel(mass: &u32) -> u32 {
    let mut total_fuel = 0;
    let initial_fuel = calculate_fuel(mass);
    total_fuel += initial_fuel;

    let mut fuel_fuel_cost = calculate_fuel(&initial_fuel);

    while fuel_fuel_cost > 0 {
        total_fuel += fuel_fuel_cost;
        fuel_fuel_cost = calculate_fuel(&fuel_fuel_cost);
    }

    total_fuel
}
