use itertools::Itertools;
use regex::Regex;

pub fn part_one(input: &str) -> Option<usize> {
    let _sensor_beacon_pairs = parse(input);
    let beacons = _sensor_beacon_pairs.iter().map(|p| p.1).collect_vec();

    let mut non_beacon_locations = Vec::new();
    let n = 2000000;

    for (sensor, beacon) in _sensor_beacon_pairs {
        let manhattan = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        let y_diff = (sensor.1 - n).abs();

        if y_diff > manhattan {
            continue;
        }

        let x_remainder = manhattan - y_diff;

        for x in sensor.0 - x_remainder..=sensor.0 + x_remainder {
            if !beacons.contains(&(x, n)) {
                non_beacon_locations.push((x, n));
            }
        }
    }

    Some(non_beacon_locations.iter().unique().count())

    // loop over all pairs
    // get manhattan from sensor to beacon
    // find y distance from sensor to y=n (10 for example 2000000 for puzzle)
    // diff y dist and manhattan distance <-> are the x positions intersected by sensor coverage and y of interest
    // cache all non-beacon/sensor points of intersection
    // count of all points of intersection
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
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
