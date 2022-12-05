use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (move_counter, from_stack, to_stack) in moves {
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

    for (move_counter, from_stack, to_stack) in moves {
        let mut tmp: VecDeque<char> = VecDeque::new();

        for _ in 0..move_counter {
            tmp.push_front(stacks[from_stack].pop_back().unwrap());
        }

        for _ in 0..tmp.len() {
            stacks[to_stack].append(&mut tmp);
        }
    }

    let s = stacks.iter().map(|v| v.back().unwrap()).collect::<String>();

    Some(s)
}

fn parse_moves(input: &str) -> Vec<(u32, usize, usize)> {
    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let input = input.trim();

    input
        .lines()
        .map(|s| -> (u32, usize, usize) {
            let caps = re.captures(s).unwrap();

            (
                caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            )
        })
        .collect_vec()
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
    let mut stack_containers: Vec<VecDeque<char>> = vec![VecDeque::new(); stack_count];

    let mut rows = input
        .lines()
        .map(|s| s.chars().skip(1).step_by(4).collect_vec())
        .peekable();

    while let Some(row) = rows.next() {
        if rows.peek().is_some() {
            for (idx, item) in row.iter().enumerate() {
                if item != &' ' {
                    stack_containers[idx].push_front(item.clone());
                }
            }
        }
    }

    stack_containers
}

fn parse_input(input: &str) -> (Vec<VecDeque<char>>, Vec<(u32, usize, usize)>) {
    let (stacks, moves) = input.split_at(input.find("\n\n").unwrap());

    (parse_stacks(stacks), parse_moves(moves))
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
