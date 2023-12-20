use itertools::Itertools;

advent_of_code::solution!(3);

fn find_priority(item: Option<&u8>) -> Option<u32> {
    if item?.is_ascii_uppercase() {
        Some(27 + u32::from(item? - b'A'))
    } else {
        Some(1 + u32::from(item? - b'a'))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(str::as_bytes)
        .map(|rucksack| -> Option<&u8> {
            let size = rucksack.len() / 2;
            let c1 = &rucksack[..size];
            let c2 = &rucksack[size..];

            c1.iter().find(|i| c2.contains(i))
        })
        .map(find_priority)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(str::as_bytes)
        .tuples()
        .map(|(b1, b2, b3)| b1.iter().find(|i| b2.contains(i) && b3.contains(i)))
        .map(find_priority)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(70));
    }
}
