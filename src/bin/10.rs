pub fn part_one(input: &str) -> Option<i32> {
    let mut cycles = Vec::new();
    let mut x_register = 1;
    let mut cycle = 1;

    cycles.push(x_register);
    cycle += 1;

    for line in input.lines() {
        let command = &line[..4];

        cycles.push(cycle * x_register);
        cycle += 1;

        if command == "addx" {
            x_register += &line[5..].parse::<i32>().unwrap();
            cycles.push(cycle * x_register);
            cycle += 1;
        }
    }

    Some(cycles.iter().skip(19).step_by(40).take(6).sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
