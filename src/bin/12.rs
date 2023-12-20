use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(12);

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
        let cur = visit
            .pop_front()
            .map_or_else(|| panic!("Unable to pop from empty queue"), |i| i);
        let cur_val = *map
            .get(&cur)
            .map_or_else(|| panic!("invalid map key"), |i| i) as i16;

        for step in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (cur.0 + step.0, cur.1 + step.1);

            if map.contains_key(&next)
                && !visited.contains_key(&next)
                && (*map
                    .get(&next)
                    .map_or_else(|| panic!("invalid map key"), |i| i) as i16
                    - cur_val)
                    * (direction as i16)
                    <= 1
            {
                visit.push_back(next);
                visited.insert(
                    next,
                    visited
                        .get(&cur)
                        .map_or_else(|| panic!("invalid map key"), |i| i)
                        + 1,
                );
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
                    i16::try_from(x).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                    i16::try_from(y).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                );
                height = 'a';
            } else if c.eq(&'E') {
                end = (
                    i16::try_from(x).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                    i16::try_from(y).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                );
                height = 'z';
            }

            map.insert(
                (
                    i16::try_from(x).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                    i16::try_from(y).map_or_else(|e| panic!("invalid conversion: {e:?}"), |i| i),
                ),
                height,
            );
        }
    }

    (map, start, end)
}

pub fn part_one(input: &str) -> Option<u16> {
    let (map, start, end) = parse_input(input);

    explore(&map, start, ExploreDirection::StartToEnd)
        .iter()
        .find_map(|(p, steps)| if *p == end { Some(*steps) } else { None })
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(29));
    }
}
