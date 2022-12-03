use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let sum_item_priorities = input.lines().map(|s| search_rucksack(s).unwrap()).sum();

    Some(sum_item_priorities)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum_badge_priorities = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| search_group(g.to_vec()).unwrap())
        .sum();

    Some(sum_badge_priorities)
}

fn find_priority(_item: char) -> Option<u32> {
    if _item as u32 >= 97 {
        Some(_item as u32 - 97 + 1)
    } else {
        Some(_item as u32 - 65 + 27)
    }
}

fn search_group(group: Vec<&str>) -> Option<u32> {
    let mut priority = 0;

    for c in group[0].chars() {
        if group[1].contains(c) && group[2].contains(c) {
            priority = find_priority(c).unwrap();
        }
    }

    Some(priority)
}

fn search_rucksack(rucksack: &str) -> Option<u32> {
    let mut priority = 0;
    let c2 = rucksack
        .chars()
        .rev()
        .take(rucksack.len() / 2)
        .collect_vec();

    for c in rucksack.chars().take(rucksack.len() / 2) {
        if c2.contains(&c) {
            priority = find_priority(c).unwrap();
        }
    }

    Some(priority)
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
