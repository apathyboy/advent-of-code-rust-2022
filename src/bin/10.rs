use itertools::Itertools;

/// # Panics
///
/// Will panic if line is invalid format
#[must_use]
pub fn part_one(input: &str) -> Option<i32> {
    let mut cycles = Vec::new();
    let mut x_register = 1;
    let mut cycle = 0;

    for line in input.lines() {
        cycle += 1;
        cycles.push(cycle * x_register);

        if line.contains("addx") {
            cycle += 1;
            cycles.push(cycle * x_register);

            x_register += &line[5..]
                .parse::<i32>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i);
        }
    }

    Some(cycles.iter().skip(19).step_by(40).take(6).sum())
}

/// # Panics
///
/// Will panic if line is invalid format
#[must_use]
pub fn part_two(input: &str) -> Option<String> {
    let mut output = Vec::new();
    let mut x_register = 1;
    let mut cycle = 0;

    for line in input.lines() {
        cycle += 1;
        output.push(crt_value(cycle, x_register));

        if line.contains("addx") {
            cycle += 1;
            output.push(crt_value(cycle, x_register));

            x_register += &line[5..]
                .parse::<i32>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i);
        }
    }

    let display = output
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .take(6)
        .join("\n");

    Some(display)
}

const fn crt_value(cycle: i32, x_register: i32) -> char {
    let draw_pos = (cycle - 1) % 40;
    if draw_pos >= x_register - 1 && draw_pos <= x_register + 1 {
        '#'
    } else {
        '.'
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
