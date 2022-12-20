use std::collections::HashMap;

#[must_use]
pub fn part_one(input: &str) -> Option<u64> {
    let total = read_directory_sizes(input)
        .iter()
        .filter(|&size| *size < 100_000)
        .sum();

    Some(total)
}

/// # Panics
///
/// Will panic if `read_directory_sizes` result is empty
#[must_use]
pub fn part_two(input: &str) -> Option<u64> {
    let dir_sizes = read_directory_sizes(input);

    let cur_usage = *dir_sizes.iter().max().unwrap();
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

fn read_directory_sizes(input: &str) -> Vec<u64> {
    let mut current_path: Vec<&str> = Vec::new();
    let mut dir_structure: HashMap<String, u64> = HashMap::new();

    for chunk in input.split('$') {
        if chunk.is_empty() {
            continue;
        }

        let mut lines = chunk.lines();
        let command = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .collect::<Vec<&str>>();

        if command[0] == "cd" {
            if command[1] == ".." {
                _ = current_path.pop();
            } else {
                current_path.push(command[1]);
                dir_structure.insert(current_path.join("/"), 0);
            }
        } else if command[0] == "ls" {
            let dir_size = lines
                .map(|s| s.split(' ').next().unwrap())
                .filter_map(|s| {
                    if s.chars().next().unwrap().is_numeric() {
                        Some(s.parse::<u64>().unwrap())
                    } else {
                        None
                    }
                })
                .sum::<u64>();

            let mut dir = String::new();

            for d in current_path.clone() {
                dir.push_str(d);
                *dir_structure.get_mut(dir.as_str()).unwrap() += dir_size;
                dir.push('/');
            }
        }
    }

    dir_structure.values().copied().collect::<Vec<u64>>()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
