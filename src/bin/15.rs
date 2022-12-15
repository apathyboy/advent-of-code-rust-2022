use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let _sensor_beacon_pairs = parse(input);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

type Point = (i64, i64);

fn parse(input: &str) -> Vec<(Point, Point)> {
    let re = Regex::new(
        r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)",
    )
    .unwrap();

    input
        .lines()
        .map(|l| -> (Point, Point) {
            let cap = re.captures(l).unwrap();

            (
                (
                    cap.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                ),
                (
                    cap.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    cap.get(4).unwrap().as_str().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect_vec()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
