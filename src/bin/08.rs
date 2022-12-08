use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);

    let visible = (0..map.len())
        .cartesian_product(0..map[0].len())
        .filter(|(x, y)| {
            if *x == 0 || *y == 0 || *x + 1 == map[0].len() || *y + 1 == map.len() {
                return true;
            }

            let c = map[*y][*x];

            if !map[*y].iter().take(*x).any(|&i| i >= c)
                || !map[*y].iter().skip(*x + 1).any(|&i| i >= c)
                || !map.iter().take(*y).any(|row| row[*x] as u32 >= c)
                || !map.iter().skip(*y + 1).any(|row| row[*x] as u32 >= c)
            {
                return true;
            }

            false
        })
        .count() as u32;

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_map(input);

    (0..map.len())
        .cartesian_product(0..map[0].len())
        .map(|(x, y)| -> u32 {
            let c = map[y][x];

            let visible_left = x - map[y]
                .iter()
                .take(x)
                .rev()
                .skip_while(|&i| *i < c)
                .skip(1)
                .count();

            let visible_right = map.len()
                - (x + 1)
                - map[y]
                    .iter()
                    .skip(x + 1)
                    .skip_while(|&i| *i < c)
                    .skip(1)
                    .count();

            let visible_up = y - map
                .iter()
                .take(y)
                .rev()
                .skip_while(|row| row[x] < c)
                .skip(1)
                .count();

            let visible_down = map[0].len()
                - (y + 1)
                - map
                    .iter()
                    .skip(y + 1)
                    .skip_while(|row| row[x] < c)
                    .skip(1)
                    .count();

            (visible_left * visible_right * visible_up * visible_down) as u32
        })
        .max()
}

fn parse_map(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
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
