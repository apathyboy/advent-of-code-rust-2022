use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<u16> {
    let (map, start, end) = parse_input(input);

    get_distance(&map, start, end)
}

pub fn part_two(input: &str) -> Option<u16> {
    let (map, _, end) = parse_input(input);

    map.iter()
        .filter_map(|(p, c)| match c {
            'a' => get_distance(&map, *p, end),
            _ => None,
        })
        .min()
}

type Point = (i16, i16);

fn get_distance(map: &HashMap<Point, char>, start: Point, end: Point) -> Option<u16> {
    let mut visited: HashMap<Point, u16> = HashMap::new();
    let mut visit: VecDeque<Point> = VecDeque::new();

    visited.insert(start, 0);
    visit.push_back(start);

    while !visit.is_empty() {
        let (cur_x, cur_y) = visit.pop_front().unwrap();

        for (dest_x, dest_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (nx, ny) = (cur_x + dest_x, cur_y + dest_y);

            if map.contains_key(&(nx, ny))
                && !visited.contains_key(&(nx, ny))
                && *map.get(&(nx, ny)).unwrap() as i16 - *map.get(&(cur_x, cur_y)).unwrap() as i16
                    <= 1
            {
                let nv = visited.get(&(cur_x, cur_y)).unwrap() + 1;
                visit.push_back((nx, ny));
                visited.insert((nx, ny), nv);

                if (nx, ny) == end {
                    return Some(nv);
                }
            }
        }
    }

    None
}

fn parse_input(input: &str) -> (HashMap<Point, char>, Point, Point) {
    let mut map = HashMap::new();
    let mut start = (255, 255);
    let mut end = (255, 255);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut height = c;
            if c.eq(&'S') {
                start = (x as i16, y as i16);
                height = 'a'
            } else if c.eq(&'E') {
                end = (x as i16, y as i16);
                height = 'z';
            }

            map.insert((x as i16, y as i16), height);
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
