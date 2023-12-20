use std::collections::HashMap;

advent_of_code::solution!(7);

fn read_directory_sizes(input: &str) -> Vec<u64> {
    let mut current_path: Vec<&str> = Vec::new();
    let mut dir_structure: HashMap<String, u64> = HashMap::new();

    for chunk in input.split('$') {
        if chunk.is_empty() {
            continue;
        }

        let mut lines = chunk.lines();
        let next_line = lines.next().map_or_else(|| panic!("Invalid input"), |t| t);
        let command = next_line.trim().split(' ').collect::<Vec<&str>>();

        if command[0] == "cd" {
            if command[1] == ".." {
                _ = current_path.pop();
            } else {
                current_path.push(command[1]);
                dir_structure.insert(current_path.join("/"), 0);
            }
        } else if command[0] == "ls" {
            let dir_size = lines
                .filter_map(|s| s.split(' ').next())
                .filter_map(|s| {
                    if s.as_bytes()[0].is_ascii_digit() {
                        match s.parse::<u64>() {
                            Ok(t) => Some(t),
                            Err(e) => panic!("Invalid input: {e:?}"),
                        }
                    } else {
                        None
                    }
                })
                .sum::<u64>();

            let mut dir = String::new();

            for d in current_path.clone() {
                dir.push_str(d);
                let tmp = dir_structure
                    .get_mut(dir.as_str())
                    .map_or_else(|| panic!("Invalid key"), |t| t);
                *tmp += dir_size;
                dir.push('/');
            }
        }
    }

    dir_structure.values().copied().collect::<Vec<u64>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let total = read_directory_sizes(input)
        .iter()
        .filter(|&size| *size < 100_000)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let dir_sizes = read_directory_sizes(input);

    let cur_usage = dir_sizes.iter().max().map_or(0, |m| *m);
    let required_space = cur_usage - 40_000_000;

    dir_sizes
        .iter()
        .filter_map(|size| {
            if *size >= required_space {
                Some(*size)
            } else {
                None
            }
        })
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(24_933_642));
    }
}
