use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input).unwrap().last().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let top_three = parse_input(input).unwrap().iter().rev().take(3).sum();

    Some(top_three)
}

fn parse_input(input: &str) -> Option<Vec<u32>> {
    input
        .split("\n\n")
        .map(|s| Some(s.lines().map(|s| s.parse::<u32>().unwrap()).sum()))
        .sorted()
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
