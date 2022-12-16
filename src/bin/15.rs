use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let sensor_beacon_pairs = parse(input);
    let n = if sensor_beacon_pairs.len() == 14 {
        10
    } else {
        2000000
    };

    let mut x_min = i32::MAX;
    let mut x_max = i32::MIN;

    let mut ranges: Vec<(i32, i32)> = Vec::new();

    for (sensor, _, dist) in sensor_beacon_pairs {
        let y_diff = (sensor.1 - n).abs();
        let x_remainder = dist - y_diff;

        if x_remainder >= 0 {
            let x_left = sensor.0 - x_remainder;
            let x_right = sensor.0 + x_remainder;
            x_min = x_min.min(x_left);
            x_max = x_max.max(x_right);

            ranges.push((x_left, x_right))
        }
    }

    ranges.sort_by_key(|r| r.0);

    let tmp = x_max - x_min;

    let gap = has_gap(&ranges);

    if gap.is_some() {
        Some(tmp - 1)
    } else {
        Some(tmp)
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let sensor_beacon_pairs = parse(input);
    let max_rowscols = if sensor_beacon_pairs.len() == 14 {
        20
    } else {
        4000000
    };

    for n in 0..max_rowscols {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;

        let mut ranges: Vec<(i32, i32)> = Vec::new();

        for (sensor, _, dist) in &sensor_beacon_pairs {
            let y_diff = (sensor.1 - n).abs();
            let x_remainder = dist - y_diff;

            if x_remainder >= 0 {
                let x_left = sensor.0 - x_remainder;
                let x_right = sensor.0 + x_remainder;
                x_min = x_min.min(x_left);
                x_max = x_max.max(x_right);

                ranges.push((x_left, x_right))
            }
        }

        ranges.sort_by_key(|r| r.0);

        if let Some(gap) = has_gap(&ranges) {
            return Some((gap as i64 * 4000000) + n as i64);
        }
    }

    None
}

type Point = (i32, i32);

fn has_gap(ranges: &[(i32, i32)]) -> Option<i32> {
    let mut x_max = i32::MIN;

    for (a, b) in ranges.iter().tuple_windows() {
        let local_max = a.1.max(x_max);

        if local_max < b.0 {
            return Some(a.1 + 1);
        } else {
            x_max = local_max;
        }
    }

    None
}

fn parse(input: &str) -> Vec<(Point, Point, i32)> {
    let re = Regex::new(
        r"Sensor at x=([-]?\d+), y=([-]?\d+): closest beacon is at x=([-]?\d+), y=([-]?\d+)",
    )
    .unwrap();

    input
        .lines()
        .map(|l| -> (Point, Point, i32) {
            let cap = re.captures(l).unwrap();

            let sensor = (
                cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let beacon = (
                cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            );

            (sensor, beacon, manhattan(&sensor, &beacon))
        })
        .collect_vec()
}

fn manhattan(p1: &Point, p2: &Point) -> i32 {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as i32
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
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56000011));
    }
}
