pub fn part_one(input: &str) -> Option<usize> {
    let fully_contains = input
        .lines()
        .map(parse)
        .filter(|v| contains(&v[..]))
        .count();

    Some(fully_contains)
}

pub fn part_two(input: &str) -> Option<usize> {
    let overlaps = input
        .lines()
        .map(parse)
        .filter(|v| overlaps(&v[..]))
        .count();

    Some(overlaps)
}

fn contains(input: &[u32]) -> bool {
    (input[0] <= input[2] && input[1] >= input[3]) || (input[0] >= input[2] && input[1] <= input[3])
}

fn overlaps(input: &[u32]) -> bool {
    (input[0] >= input[2] && input[0] <= input[3])
        || (input[1] >= input[2] && input[1] <= input[3])
        || (input[2] >= input[0] && input[2] <= input[1])
        || (input[3] >= input[0] && input[3] <= input[1])
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split(',')
        .flat_map(|s| s.split('-').map(|v| v.parse().unwrap()))
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
