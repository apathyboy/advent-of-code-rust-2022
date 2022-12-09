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
        .flat_map(|(motion, amt)| move_instruction(&mut knots, motion, amt))
        .unique()
        .count();

    Some(unique_visits as u32)
}

fn move_instruction(
    knots: &mut Vec<(i32, i32)>,
    motion: &(i32, i32),
    amt: &i32,
) -> Vec<(i32, i32)> {
    (0..*amt)
        .map(|_| -> (i32, i32) {
            move_toward(&mut knots[0], motion);

            for i in 1..knots.len() {
                let d = diff(&knots[i - 1], &knots[i]);

                if d.0.abs() > 1 || d.1.abs() > 1 {
                    move_toward(&mut knots[i], &(d.0.signum(), d.1.signum()));
                }
            }

            *knots.last().unwrap()
        })
        .collect()
}

fn diff(knot1: &(i32, i32), knot2: &(i32, i32)) -> (i32, i32) {
    (knot1.0 - knot2.0, knot1.1 - knot2.1)
}

fn move_toward(knot: &mut (i32, i32), dir: &(i32, i32)) {
    knot.0 += dir.0;
    knot.1 += dir.1;
}

fn parse_input(input: &str) -> Vec<((i32, i32), i32)> {
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, amt)| (parse_dir(dir).unwrap(), amt.parse::<i32>().unwrap()))
        .collect_vec()
}

fn parse_dir(dir: &str) -> Option<(i32, i32)> {
    match dir {
        "U" => Some((0, 1)),
        "D" => Some((0, -1)),
        "L" => Some((-1, 0)),
        "R" => Some((1, 0)),
        _ => None,
    }
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
