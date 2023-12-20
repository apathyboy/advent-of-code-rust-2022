use itertools::Itertools;

advent_of_code::solution!(8);

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
        .map(|s| {
            s.chars()
                .map(|c| {
                    c.to_digit(10)
                        .map_or_else(|| panic!("Invalid input"), |t| t)
                })
                .collect_vec()
        })
        .collect_vec();

    let x_max = map[0].len();
    let y_max = map.len();

    (map, x_max, y_max)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map, x_max, y_max) = parse_map(input);

    let visible = (0..y_max)
        .cartesian_product(0..x_max)
        .filter(|(x, y)| visibility_check(&map, *x, x_max, *y, y_max))
        .count();

    Some(visible)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, x_max, y_max) = parse_map(input);

    (1..y_max - 1)
        .cartesian_product(1..x_max - 1)
        .map(|(x, y)| scenic_score(&map, x, x_max, y, y_max))
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(8));
    }
}
