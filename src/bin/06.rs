use itertools::Itertools;

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut pos: usize = 4;
    while pos < input.chars().count() {
        let window = input.chars().skip(pos - 4).take(4).collect_vec();
        if window.iter().unique().count() == 4 {
            break;
        }

        pos += 1;
    }

    Some(pos as u32)
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut pos: usize = 14;
    while pos < input.chars().count() {
        let window = input.chars().skip(pos - 14).take(14).collect_vec();
        if window.iter().unique().count() == 14 {
            break;
        }

        pos += 1;
    }

    Some(pos as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
