use std::collections::HashMap;

use itertools::Itertools;

// when deciding where to move next find the "benefit" of opening each other unopened valve
// (remaining time - (1 minute * distance to valve + 1 minute to open)) * target valve pressure
// select the highest

pub fn part_one(input: &str) -> Option<i32> {
    let valve_map = parse(input);

    let mut open_valves: Vec<String> = Vec::new();

    let mut arr: [i32; 30] = [0; 30];

    let mut cur_valve_id = "AA";
    let mut prev_valve_id = "";
    let time_limit = 30;

    for minute in 1..=time_limit {
        let cur_valve = valve_map.get(cur_valve_id).unwrap();
        let remaining = time_limit - minute;

        // can or should open the current valve
        if cur_valve.flow_rate > 0 && !open_valves.contains(&cur_valve.id) {}
        // if not move
    }

    Some(
        arr.iter()
            .scan(0, |acc, x| {
                *acc += x;
                Some(*acc)
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: i16,
    connected: Vec<String>,
}

fn parse_valve(line: &str) -> Valve {
    let (valve_str, connected_str) = line.split_once("; ").unwrap();

    let valve = &valve_str[6..8];
    let flow_rate = valve_str[23..].parse::<i16>().unwrap();

    let connected = if connected_str.starts_with("tunnels ") {
        connected_str[23..]
            .split(',')
            .map(|s| s.to_string())
            .collect_vec()
    } else {
        vec![connected_str[22..].to_string()]
    };

    Valve {
        id: valve.to_string(),
        flow_rate,
        connected,
    }
}

fn parse(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|l| {
            let valve = parse_valve(l);
            (valve.id.clone(), valve)
        })
        .collect::<HashMap<String, Valve>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
