use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    model(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    model(input, 10)
}

fn model(input: &str, knot_count: usize) -> Option<u32> {
    let motions = parse_input(input);
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); knot_count];

    let unique_visits = motions
        .iter()
        .flat_map(|(motion, amt)| -> Vec<(i32, i32)> {
            (0..*amt)
                .map(|_| -> (i32, i32) {
                    knots[0].0 += motion.0;
                    knots[0].1 += motion.1;

                    for i in 1..knots.len() {
                        let x_diff = knots[i - 1].0 - knots[i].0;
                        let y_diff = knots[i - 1].1 - knots[i].1;

                        if (x_diff).abs() > 1 || (y_diff).abs() > 1 {
                            let follow_motion = (x_diff.signum(), y_diff.signum());

                            knots[i].0 += follow_motion.0;
                            knots[i].1 += follow_motion.1;
                        }
                    }

                    knots[knot_count - 1]
                })
                .collect_vec()
        })
        .unique()
        .count();

    Some(unique_visits as u32)
}

fn move_dir(dir: &str) -> Option<(i32, i32)> {
    match dir {
        "U" => Some((0, 1)),
        "D" => Some((0, -1)),
        "L" => Some((-1, 0)),
        "R" => Some((1, 0)),
        _ => None,
    }
}

fn parse_input(input: &str) -> Vec<((i32, i32), i32)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, amt)| (move_dir(dir).unwrap(), amt.parse::<i32>().unwrap()))
        .collect_vec()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}
