use std::collections::{HashMap, VecDeque};

#[must_use]
pub fn part_one(input: &str) -> Option<u16> {
    let (map, start, end) = parse_input(input);

    explore(&map, start, ExploreDirection::StartToEnd)
        .iter()
        .find_map(|(p, steps)| if *p == end { Some(*steps) } else { None })
}

#[must_use]
pub fn part_two(input: &str) -> Option<u16> {
    let (map, _, end) = parse_input(input);

    explore(&map, end, ExploreDirection::EndToStart)
        .iter()
        .filter_map(|(p, steps)| match map.get(&(p.0, p.1)) {
            Some('a') => Some(*steps),
            _ => None,
        })
        .min()
}

type Point = (i16, i16);

#[derive(Clone, Copy)]
enum ExploreDirection {
    StartToEnd = 1,
    EndToStart = -1,
}

fn explore(
    map: &HashMap<Point, char>,
    start: Point,
    direction: ExploreDirection,
) -> HashMap<Point, u16> {
    let mut visited: HashMap<Point, u16> = HashMap::from([(start, 0)]);
    let mut visit: VecDeque<Point> = VecDeque::from([start]);

    while !visit.is_empty() {
        let cur = visit.pop_front().unwrap();
        let cur_val = *map.get(&cur).unwrap() as i16;

        for step in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (cur.0 + step.0, cur.1 + step.1);

            if map.contains_key(&next)
                && !visited.contains_key(&next)
                && (*map.get(&next).unwrap() as i16 - cur_val) * (direction as i16) <= 1
            {
                visit.push_back(next);
                visited.insert(next, visited.get(&cur).unwrap() + 1);
            }
        }
    }

    visited
}

fn parse_input(input: &str) -> (HashMap<Point, char>, Point, Point) {
    let mut map = HashMap::new();
    let mut start = (255, 255);
    let mut end = (255, 255);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut height = c;
            if c.eq(&'S') {
                start = (
                    i16::try_from(x).ok().unwrap(),
                    i16::try_from(y).ok().unwrap(),
                );
                height = 'a';
            } else if c.eq(&'E') {
                end = (
                    i16::try_from(x).ok().unwrap(),
                    i16::try_from(y).ok().unwrap(),
                );
                height = 'z';
            }

            map.insert(
                (
                    i16::try_from(x).ok().unwrap(),
                    i16::try_from(y).ok().unwrap(),
                ),
                height,
            );
        }
    }

    (map, start, end)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
