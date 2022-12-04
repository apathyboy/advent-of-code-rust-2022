pub fn part_one(input: &str) -> Option<u32> {
    input.lines().map(play_match_rules1).sum::<Option<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().map(play_match_rules2).sum::<Option<u32>>()
}

fn score(throw: char) -> Option<u32> {
    match throw {
        'A' | 'X' => Some(1),
        'B' | 'Y' => Some(2),
        'C' | 'Z' => Some(3),
        _ => None,
    }
}

fn play_match_rules1(game: &str) -> Option<u32> {
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

fn play_match_rules2(game: &str) -> Option<u32> {
    let opponent = game.chars().nth(0).unwrap();
    let player = game.chars().nth(2).unwrap();
    let opponent_score = score(opponent);

    if player == 'Y' {
        Some(3 + opponent_score.unwrap())
    } else if player == 'X' {
        if opponent_score == Some(1) {
            Some(3)
        } else {
            Some(opponent_score.unwrap() - 1)
        }
    } else if opponent_score == Some(3) {
        Some(7)
    } else {
        Some(6 + (opponent_score.unwrap() + 1))
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
