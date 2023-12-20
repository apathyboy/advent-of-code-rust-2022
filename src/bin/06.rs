use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let mut pos: usize = 4;
    while pos < input.chars().count() {
        let window = input.chars().skip(pos - 4).take(4).collect_vec();
        if window.iter().unique().count() == 4 {
            break;
        }

        pos += 1;
    }

    Some(pos)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pos: usize = 14;
    while pos < input.chars().count() {
        let window = input.chars().skip(pos - 14).take(14).collect_vec();
        if window.iter().unique().count() == 14 {
            break;
        }

        pos += 1;
    }

    Some(pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(19));
    }
}
