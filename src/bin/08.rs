use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let map_transposed = transpose(&map);

    let mut visible = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if x == 0 || y == 0 || x + 1 == map[0].len() || y + 1 == map.len() {
                visible += 1;
                continue;
            }

            let c = map[y][x];

            // check left
            let visible_left = !map[y].iter().take(x).any(|&i| i >= c);

            // check right
            let visible_right = !map[y].iter().skip(x + 1).any(|&i| i >= c);

            // check up
            let visible_up = !map_transposed[x].iter().take(y).any(|&i| i >= c);

            // check down
            let visible_down = !map_transposed[x].iter().skip(y + 1).any(|&i| i >= c);

            if visible_left || visible_right || visible_up || visible_down {
                visible += 1;
            }
        }
    }

    Some(visible)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec();

    let map_transposed = transpose(&map);

    let mut max_scenic_score = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if x == 0 || y == 0 || x + 1 == map[0].len() || y + 1 == map.len() {
                continue;
            }

            let c = map[y][x];

            let mut visible_left =
                map[y].iter().take(x).rev().take_while(|&i| *i < c).count() as u32;
            if visible_left < x as u32 {
                visible_left += 1;
            }

            let mut visible_right =
                map[y].iter().skip(x + 1).take_while(|&i| *i < c).count() as u32;
            if visible_right < (map.len() - (x + 1)) as u32 {
                visible_right += 1;
            }

            let mut visible_up = map_transposed[x]
                .iter()
                .take(y)
                .rev()
                .take_while(|&i| *i < c)
                .count() as u32;
            if visible_up < y as u32 {
                visible_up += 1;
            }

            let mut visible_down = map_transposed[x]
                .iter()
                .skip(y + 1)
                .take_while(|&i| *i < c)
                .count() as u32;
            if visible_down < (map[0].len() - (y + 1)) as u32 {
                visible_down += 1;
            }

            let scenic_score = visible_left * visible_right * visible_up * visible_down;
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }

    Some(max_scenic_score)
}

fn transpose(map: &Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    (0..map[0].len())
        .map(|i| map.iter().map(|c| c[i]).collect_vec())
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
