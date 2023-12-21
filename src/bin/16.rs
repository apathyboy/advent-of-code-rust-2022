use std::collections::HashMap;

advent_of_code::solution!(16);

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: u32,
    neighbors: Vec<String>,
}

impl Valve {
    fn new(id: &str, flow_rate: u32, neighbors: &[&str]) -> Self {
        Self {
            id: id.to_string(),
            flow_rate,
            neighbors: neighbors.iter().map(|n| n.to_string()).collect(),
        }
    }
}

fn parse_valve(input: &str) -> Valve {
    let (valve, neighbors) = input.split_once("; ").unwrap();
    let neighbors = neighbors[22..].trim().split(", ").collect::<Vec<_>>();

    Valve::new(&valve[6..8], valve[23..].parse().unwrap(), &neighbors)
}

fn find_next_valve(
    valves: &HashMap<String, Valve>,
    opened_valves: &[String],
    current_valve: &str,
    remaining_time: u32,
) -> Option<(Valve, u32, u32)> {
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = input
        .lines()
        .map(|line| {
            let valve = parse_valve(line);

            (valve.id.clone(), valve)
        })
        .collect::<HashMap<_, _>>();

    let mut current_valve = "AA".to_string();
    let mut opened_valves: Vec<String> = vec![];
    let mut remaining_time = 30;
    let mut released_pressure = 0;

    while let Some((valve, time_consumed, pressure)) = find_next_valve(
        &valves,
        &opened_valves,
        current_valve.as_str(),
        remaining_time,
    ) {
        current_valve = valve.id.clone();
        remaining_time = time_consumed;
        released_pressure += pressure;
    }

    Some(released_pressure)
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
