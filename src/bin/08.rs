use itertools::Itertools;

#[must_use]
pub fn part_one(input: &str) -> Option<usize> {
    let (map, x_max, y_max) = parse_map(input);

    let visible = (0..y_max)
        .cartesian_product(0..x_max)
        .filter(|(x, y)| visibility_check(&map, *x, x_max, *y, y_max))
        .count();

    Some(visible)
}

#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let (map, x_max, y_max) = parse_map(input);

    (1..y_max - 1)
        .cartesian_product(1..x_max - 1)
        .map(|(x, y)| scenic_score(&map, x, x_max, y, y_max))
        .max()
}

fn visibility_check(map: &[Vec<u32>], x: usize, x_max: usize, y: usize, y_max: usize) -> bool {
    let c = map[y][x];

    (0..x).all(|i| map[y][i] < c)
        || (x + 1..x_max).all(|i| map[y][i] < c)
        || (0..y).all(|j| map[j][x] < c)
        || (y + 1..y_max).all(|j| map[j][x] < c)
}

fn scenic_score(map: &[Vec<u32>], x: usize, x_max: usize, y: usize, y_max: usize) -> usize {
    let c = map[y][x];

    (1 + (1..x).rev().take_while(|&i| map[y][i] < c).count())
        * (1 + (x + 1..x_max - 1).take_while(|&i| map[y][i] < c).count())
        * (1 + (1..y).rev().take_while(|&i| map[i][x] < c).count())
        * (1 + (y + 1..y_max - 1).take_while(|&i| map[i][x] < c).count())
}

fn parse_map(input: &str) -> (Vec<Vec<u32>>, usize, usize) {
    let map = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let x_max = map[0].len();
    let y_max = map.len();

    (map, x_max, y_max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
