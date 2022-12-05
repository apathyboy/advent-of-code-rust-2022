use std::collections::VecDeque;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (move_counter, from_stack, to_stack) in moves {
        for _ in 0..move_counter {
            let move_val = stacks[from_stack].pop_back().unwrap();
            stacks[to_stack].push_back(move_val);
        }
    }

    Some(collect_stack_tops(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (move_counter, from_stack, to_stack) in moves {
        let at = stacks[from_stack].len() - move_counter as usize;

        let mut to_move = stacks[from_stack].split_off(at);
        stacks[to_stack].append(&mut to_move);
    }

    Some(collect_stack_tops(&stacks))
}

fn collect_stack_tops(stacks: &Vec<VecDeque<char>>) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

fn parse_moves(input: &str) -> Vec<(u32, usize, usize)> {
    let input = input.trim();
    input
        .lines()
        .map(|line| -> (u32, usize, usize) {
            let vals = line.split(' ').skip(1).step_by(2).collect_vec();
            (
                vals[0].parse::<u32>().unwrap(),
                vals[1].parse::<usize>().unwrap() - 1,
                vals[2].parse::<usize>().unwrap() - 1,
            )
        })
        .collect_vec()
}

fn parse_stacks(input: &str) -> Vec<VecDeque<char>> {
    let stack_count = (input.lines().next().unwrap().len() + 1) / 4;
    let mut stacks = vec![VecDeque::new(); stack_count];

    for line in input.rsplit('\n').skip(1) {
        for (i, v) in line.chars().skip(1).step_by(4).enumerate() {
            if v != ' ' {
                stacks.get_mut(i).unwrap().push_back(v);
            }
        }
    }

    stacks
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
