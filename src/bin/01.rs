use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(parse_group)
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let top3 = input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(parse_group)
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(3)
        .sum();

    Some(top3)
}

fn parse_group(input: &str) -> u32 {
    input.lines().filter_map(|s| s.parse::<u32>().ok()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(45000));
    }
}
