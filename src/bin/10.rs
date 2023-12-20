use itertools::Itertools;

advent_of_code::solution!(10);

const fn crt_value(cycle: i32, x_register: i32) -> char {
    let draw_pos = (cycle - 1) % 40;
    if draw_pos >= x_register - 1 && draw_pos <= x_register + 1 {
        '#'
    } else {
        '.'
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
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
