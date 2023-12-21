use std::collections::HashMap;

advent_of_code::solution!(16);

#[derive(Debug)]
struct Valve<'a> {
    id: &'a str,
    flow_rate: u32,
    neighbors: Vec<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(id: &'a str, flow_rate: u32, neighbors: Vec<&'a str>) -> Self {
        Self {
            id,
            flow_rate,
            neighbors,
        }
    }
}

fn parse_valve(input: &str) -> Valve {
    let (valve, neighbors) = input.split_once("; ").unwrap();

    Valve::new(
        &valve[6..8],
        valve[23..].parse().unwrap(),
        neighbors[22..].trim().split(", ").collect::<Vec<_>>(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = input.lines().map(parse_valve).collect::<Vec<_>>();

    let mut pressure_released = 0;
    let mut remaining_time = 30;

    let mut current_valve = &valves[0];
    let mut opened_valves: Vec<&str> = Vec::new();

    // take the current valve and find the max potential release for all other valves not currently open and pick the largest
    // when determining the potential, consider it takes 1 sec to move between valves and 1 second to open. Find the potential
    // for the remaining time factoring in movement and opening times.

    let mut potential: HashMap<&str, u32> = HashMap::new();

    let mut to_check = current_valve.neighbors.clone();

    let mut distance = 1;
    while !to_check.is_empty() {
        let next_valve = to_check.pop().unwrap();

        println!("next_valve: {}", next_valve);
    }

    Some(pressure_released)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), None);
    }
}
