pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(outcomes_part1).sum::<Option<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(outcomes_part2).sum::<Option<u32>>()
}

fn outcomes_part1(game: &str) -> Option<u32> {
    match game {
        "A X" => Some(4),
        "A Y" => Some(8),
        "A Z" => Some(3),
        "B X" => Some(1),
        "B Y" => Some(5),
        "B Z" => Some(9),
        "C X" => Some(7),
        "C Y" => Some(2),
        "C Z" => Some(6),
        _ => None,
    }
}

fn outcomes_part2(game: &str) -> Option<u32> {
    match game {
        "A X" => Some(3),
        "A Y" => Some(4),
        "A Z" => Some(8),
        "B X" => Some(1),
        "B Y" => Some(5),
        "B Z" => Some(9),
        "C X" => Some(2),
        "C Y" => Some(6),
        "C Z" => Some(7),
        _ => None,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
