use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|rucksack| -> u8 {
            let size = rucksack.len() / 2;
            let c1 = &rucksack[..size];
            let c2 = &rucksack[size..];

            c1.bytes().find(|i| c2.as_bytes().contains(i)).unwrap()
        })
        .map(find_priority)
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .tuples()
        .map(|(b1, b2, b3)| -> u8 {
            let s1 = b2.as_bytes();
            let s2 = b3.as_bytes();
            b1.bytes()
                .find(|i| s1.contains(i) && s2.contains(i))
                .unwrap()
        })
        .map(find_priority)
        .sum()
}

fn find_priority(item: u8) -> Option<u32> {
    if item.is_ascii_uppercase() {
        Some(27 + (item - b'A') as u32)
    } else {
        Some(1 + (item - b'a') as u32)
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
