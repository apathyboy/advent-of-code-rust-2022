use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input.split("\n\n").map(parse_group).max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let top3 = input
        .split("\n\n")
        .map(parse_group)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    Some(top3)
}

fn parse_group(input: &str) -> u32 {
    input
        .split('\n')
        .filter_map(|s| s.parse::<u32>().ok())
        .sum()
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
