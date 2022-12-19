use itertools::Itertools;

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .map(|rucksack| -> Option<&u8> {
            let size = rucksack.len() / 2;
            let c1 = &rucksack[..size];
            let c2 = &rucksack[size..];

            c1.iter().find(|i| c2.contains(i))
        })
        .map(find_priority)
        .sum()
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|l| l.as_bytes())
        .tuples()
        .map(|(b1, b2, b3)| b1.iter().find(|i| b2.contains(i) && b3.contains(i)))
        .map(find_priority)
        .sum()
}

fn find_priority(item: Option<&u8>) -> Option<u32> {
    match item?.is_ascii_uppercase() {
        true => Some(27 + (item? - b'A') as u32),
        false => Some(1 + (item? - b'a') as u32),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
