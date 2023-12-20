advent_of_code::solution!(2);

fn outcomes_part1(game: &str) -> u32 {
    match game {
        "A X" => 4,
        "A Y" => 8,
        "A Z" => 3,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 7,
        "C Y" => 2,
        "C Z" => 6,
        _ => panic!(),
    }
}

fn outcomes_part2(game: &str) -> u32 {
    match game {
        "A X" => 3,
        "A Y" => 4,
        "A Z" => 8,
        "B X" => 1,
        "B Y" => 5,
        "B Z" => 9,
        "C X" => 2,
        "C Y" => 6,
        "C Z" => 7,
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(outcomes_part1).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(outcomes_part2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(12));
    }
}
