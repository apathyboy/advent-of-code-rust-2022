use itertools::Itertools;

pub type Move = (u32, usize, usize);

#[must_use]
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

#[must_use]
pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, moves) = parse_input(input);

    for (count, from, to) in &moves {
        let at = stacks[*from].len() - *count as usize;

        let mut to_move = stacks[*from].split_off(at);
        stacks[*to].append(&mut to_move);
    }

    Some(collect_stack_tops(&stacks))
}

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
                let stack = match stacks.get_mut(i) {
                    Some(t) => t,
                    None => panic!("Attempt to access invalid stack: {i}"),
                };
                stack.push(v);
            }
        }
    }

    stacks
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
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
