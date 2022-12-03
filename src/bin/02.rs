pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|s| play_match(s.chars().next().unwrap(), s.chars().nth(2).unwrap()))
        .sum::<Option<u32>>()
}

pub fn part_two(input: &str) -> Option<u32> {
    input
        .lines()
        .map(|s| play_match2(s.chars().next().unwrap(), s.chars().nth(2).unwrap()))
        .sum::<Option<u32>>()
}

fn score(throw: char) -> Option<u32> {
    match throw {
        'A' | 'X' => Some(1),
        'B' | 'Y' => Some(2),
        'C' | 'Z' => Some(3),
        _ => None,
    }
}

fn play_match2(opponent: char, player: char) -> Option<u32> {
    let opponent_score = score(opponent);

    if player == 'Y' {
        Some(3 + opponent_score.unwrap())
    } else if player == 'X' {
        if opponent_score == Some(1) {
            Some(3)
        } else {
            Some(opponent_score.unwrap() - 1)
        }
    } else {
        if opponent_score == Some(3) {
            Some(7)
        } else {
            Some(6 + (opponent_score.unwrap() + 1))
        }
    }
}

fn play_match(opponent: char, player: char) -> Option<u32> {
    let opponent_score = score(opponent);
    let player_score = score(player);

    let final_score;

    if opponent_score == player_score {
        final_score = 3 + player_score.unwrap();
    } else if (opponent_score == Some(1) && player_score == Some(3))
        || ((opponent_score > Some(1) && opponent_score > player_score)
            && !(player_score == Some(1) && opponent_score == Some(3)))
    {
        final_score = player_score.unwrap();
    } else {
        final_score = 6 + player_score.unwrap();
    }

    Some(final_score)
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
