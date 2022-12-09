use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let motions = parse_input(input);

    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for (dir, amt) in motions {
        let motion = move_dir(dir).unwrap();

        for _ in 0..amt {
            head.0 += motion.0;
            head.1 += motion.1;

            if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
                tail.0 += motion.0;
                tail.1 += motion.1;

                if motion.0 != 0 && tail.1 != head.1 {
                    if tail.1 > head.1 {
                        tail.1 -= 1;
                    } else {
                        tail.1 += 1;
                    }
                }

                if motion.1 != 0 && tail.0 != head.0 {
                    if tail.0 > head.0 {
                        tail.0 -= 1;
                    } else {
                        tail.0 += 1;
                    }
                }
            }

            visited.push(tail);
        }
    }

    Some(visited.iter().unique().count() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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

fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .lines()
        .map(|line| -> (&str, i32) {
            let (dir, amt) = line.split_once(' ').unwrap();

            (dir, amt.parse::<i32>().unwrap())
        })
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
        assert_eq!(part_two(&input), None);
    }
}
