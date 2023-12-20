use itertools::Itertools;

advent_of_code::solution!(5);

pub type Move = (u32, usize, usize);

fn collect_stack_tops(stacks: &[Vec<char>]) -> String {
    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

fn parse_moves(input: &str) -> Vec<Move> {
    let input = input.trim();
    input
        .lines()
        .map(|line| -> (u32, usize, usize) {
            let vals = line.split(' ').skip(1).step_by(2).collect_vec();

            let count = match vals[0].parse::<u32>() {
                Ok(t) => t,
                Err(e) => panic!("Error parsing input: {e:?}"),
            };
            let from = match vals[1].parse::<usize>() {
                Ok(t) => t,
                Err(e) => panic!("Error prasing input: {e:?}"),
            };
            let to = match vals[2].parse::<usize>() {
                Ok(t) => t,
                Err(e) => panic!("Error prasing input: {e:?}"),
            };

            (count, from - 1, to - 1)
        })
        .collect_vec()
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let next_line = input.lines().next().map_or("", |s| s);
    let stack_count = (next_line.len() + 1) / 4;
    let mut stacks = vec![Vec::new(); stack_count];

    for line in input.rsplit('\n').skip(1) {
        for (i, v) in line.chars().skip(1).step_by(4).enumerate() {
            if v != ' ' {
                let stack = stacks
                    .get_mut(i)
                    .map_or_else(|| panic!("Invalid stack id {i}"), |t| t);
                stack.push(v);
            }
        }
    }

    stacks
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let split_point = input
        .replace("\r\n", "\n")
        .find("\n\n")
        .map_or_else(|| panic!("Invalid input"), |t| t);

    let (stacks, moves) = input.split_at(split_point);

    (parse_stacks(stacks), parse_moves(moves))
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (count, from, to) in &moves {
        (0..*count).for_each(|_| {
            let c = stacks[*from].pop().map_or(' ', |s| s);
            stacks[*to].push(c);
        });
    }

    Some(collect_stack_tops(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (count, from, to) in &moves {
        let at = stacks[*from].len() - *count as usize;

        let mut to_move = stacks[*from].split_off(at);
        stacks[*to].append(&mut to_move);
    }

    Some(collect_stack_tops(&stacks))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
