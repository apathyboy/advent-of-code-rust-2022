use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let mut cycles = Vec::new();
    let mut x_register = 1;
    let mut cycle = 0;

    for line in input.lines() {
        if line.contains("addx") {
            cycle += 1;
            cycles.push(cycle * x_register);

            cycle += 1;
            cycles.push(cycle * x_register);

            x_register += &line[5..].parse::<i32>().unwrap();
        } else {
            cycle += 1;
            cycles.push(cycle * x_register);
        }
    }

    Some(cycles.iter().skip(19).step_by(40).take(6).sum())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut output = Vec::new();
    let mut x_register = 1;
    let mut cycle = 0;

    for line in input.lines() {
        if line.contains("addx") {
            cycle += 1;
            output.push(crt_value(cycle, x_register));

            cycle += 1;
            output.push(crt_value(cycle, x_register));

            x_register += &line[5..].parse::<i32>().unwrap();
        } else {
            cycle += 1;
            output.push(crt_value(cycle, x_register));
        }
    }

    let display = output
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .take(6)
        .join("\n");

    Some(display)
}

fn crt_value(cycle: i32, x_register: i32) -> char {
    let draw_pos = (cycle - 1) % 40;
    match draw_pos >= x_register - 1 && draw_pos <= x_register + 1 {
        true => '#',
        false => '.',
    }
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
        let string = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_string();

        assert_eq!(part_two(&input), Some(string));
    }
}