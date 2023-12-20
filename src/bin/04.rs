advent_of_code::solution!(4);

const fn contains(input: &[u32]) -> bool {
    (input[0] <= input[2] && input[1] >= input[3]) || (input[0] >= input[2] && input[1] <= input[3])
}

const fn overlaps(input: &[u32]) -> bool {
    !(input[2] > input[1] || input[3] < input[0])
}

fn parse(input: &str) -> Vec<u32> {
    input
        .split(',')
        .flat_map(|s| {
            s.split('-').map(|v| match v.parse() {
                Ok(t) => t,
                Err(e) => panic!("Error parsing input: {e:?}"),
            })
        })
        .collect()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(4));
    }
}
