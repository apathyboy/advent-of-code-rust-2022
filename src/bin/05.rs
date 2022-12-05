use std::collections::VecDeque;

use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for m in moves.lines() {
        let caps = re.captures(m).unwrap();

        let move_counter = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let from_stack = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to_stack = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        for _ in 0..move_counter {
            let move_val = stacks[from_stack].pop_back().unwrap();
            stacks[to_stack].push_back(move_val);
        }
    }

    let s = stacks.iter().map(|v| v.back().unwrap()).collect::<String>();

    Some(s)
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();

    for m in moves.lines() {
        let caps = re.captures(m).unwrap();

        let move_counter = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let from_stack = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
        let to_stack = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;

        let mut tmp: Vec<char> = Vec::new();

        for _ in 0..move_counter {
            let move_val = stacks[from_stack].pop_back().unwrap();
            tmp.push(move_val);
        }

        for _ in 0..tmp.len() {
            stacks[to_stack].push_back(tmp.pop().unwrap());
        }
    }

    let s = stacks.iter().map(|v| v.back().unwrap()).collect::<String>();

    Some(s)
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, &str) {
    let (stacks, moves) = input.split_at(input.find("\n\n").unwrap());

    let stacks = stacks.lines().collect::<Vec<&str>>();

    let stack_count = (stacks[0].len() + 1) / 4;

    let mut stack_containers: Vec<VecDeque<char>> = vec![VecDeque::new(); stack_count];

    for row in stacks {
        if row.chars().nth(1).unwrap() == '1' {
            continue;
        }

        let mut stack_id = 0;

        while stack_id < stack_count {
            let contents = row.chars().nth((stack_id * 4) + 1).unwrap();

            if contents != ' ' {
                stack_containers[stack_id].push_front(contents);
            }

            stack_id += 1;
        }
    }

    (stack_containers, moves.trim())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
