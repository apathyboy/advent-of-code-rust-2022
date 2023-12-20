use itertools::Itertools;

advent_of_code::solution!(9);

fn model(input: &str, knot_count: usize) -> usize {
    let motions = parse_input(input);
    let mut knots: Vec<(i32, i32)> = vec![(0, 0); knot_count];

    motions
        .iter()
        .flat_map(|(motion, amt)| move_instruction(&mut knots, *motion, *amt))
        .unique()
        .count()
}

fn move_instruction(knots: &mut Vec<(i32, i32)>, motion: (i32, i32), amt: i32) -> Vec<(i32, i32)> {
    (0..amt)
        .map(|_| -> (i32, i32) {
            move_toward(&mut knots[0], motion);

            for i in 1..knots.len() {
                let d = diff(knots[i - 1], knots[i]);

                if d.0.abs() > 1 || d.1.abs() > 1 {
                    move_toward(&mut knots[i], (d.0.signum(), d.1.signum()));
                }
            }

            *knots
                .last()
                .map_or_else(|| panic!("empty knot list"), |k| k)
        })
        .collect()
}

const fn diff(knot1: (i32, i32), knot2: (i32, i32)) -> (i32, i32) {
    (knot1.0 - knot2.0, knot1.1 - knot2.1)
}

fn move_toward(knot: &mut (i32, i32), dir: (i32, i32)) {
    knot.0 += dir.0;
    knot.1 += dir.1;
}

fn parse_input(input: &str) -> Vec<((i32, i32), i32)> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map_or_else(|| panic!("Invalid format"), |s| s)
        })
        .map(|(dir, amt)| {
            (
                parse_dir(dir),
                amt.parse::<i32>()
                    .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i),
            )
        })
        .collect_vec()
}

fn parse_dir(dir: &str) -> (i32, i32) {
    match dir {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => panic!("Invalid format"),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(model(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(model(input, 10))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1));
    }
}
