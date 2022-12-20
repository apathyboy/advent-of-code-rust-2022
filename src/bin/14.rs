use itertools::Itertools;
use std::cmp::{max, min};

#[must_use]
pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let max_y = find_max_y(&map);
    let mut sand_counter = 0;

    loop {
        let mut sand = (500, 0);

        while let Some(next_move) = find_next_move(&map, sand, max_y + 2) {
            if next_move.1 == max_y {
                return Some(sand_counter);
            }

            sand = next_move;
        }

        map.push(sand);
        sand_counter += 1;
    }
}

#[must_use]
pub fn part_two(input: &str) -> Option<u32> {
    let mut map = parse(input);
    let floor_y = find_max_y(&map) + 2;
    let mut sand_counter = 0;

    loop {
        let mut sand = (500, 0);

        while let Some(next_move) = find_next_move(&map, sand, floor_y) {
            sand = next_move;
        }

        if sand.0 == 500 && sand.1 == 0 {
            return Some(sand_counter + 1);
        }

        map.push(sand);
        sand_counter += 1;
    }
}

type Point = (i16, i16);

fn find_next_move(map: &[Point], sand: Point, floor: i16) -> Option<Point> {
    let (x_check, y_check) = sand;

    if y_check + 1 == floor {
        return None;
    }

    if !map.contains(&(x_check, y_check + 1)) {
        Some((x_check, y_check + 1))
    } else if !map.contains(&(x_check - 1, y_check + 1)) {
        Some((x_check - 1, y_check + 1))
    } else if !map.contains(&(x_check + 1, y_check + 1)) {
        Some((x_check + 1, y_check + 1))
    } else {
        None
    }
}

fn find_max_y(map: &[Point]) -> i16 {
    map.iter().map(|p| p.1).max().unwrap()
}

fn parse(input: &str) -> Vec<Point> {
    let mut map: Vec<Point> = Vec::new();

    for line in input.lines() {
        let tmp = line
            .split(" -> ")
            .map(|p| {
                let (x_str, y_str) = p.split_once(',').unwrap();
                (x_str.parse::<i16>().unwrap(), y_str.parse::<i16>().unwrap())
            })
            .collect_vec();
        for (p1, p2) in tmp.iter().tuple_windows() {
            if p1.0 == p2.0 {
                let x = p1.0;
                let y_min = min(p1.1, p2.1);
                let y_max = max(p1.1, p2.1);
                for y in y_min..=y_max {
                    if !map.contains(&(x, y)) {
                        map.push((x, y));
                    }
                }
            } else {
                let y = p1.1;
                let x_min = min(p1.0, p2.0);
                let x_max = max(p1.0, p2.0);

                for x in x_min..=x_max {
                    if !map.contains(&(x, y)) {
                        map.push((x, y));
                    }
                }
            }
        }
    }

    map
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
