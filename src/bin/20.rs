use itertools::Itertools;
use std::convert::TryFrom;
/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    let encrypted = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            (
                l.parse::<i64>()
                    .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i),
                idx,
            )
        })
        .collect_vec();
    let mixed = mix(encrypted);

    let cur_offset = match mixed.iter().find_position(|&(v, _)| *v == 0) {
        Some((idx, _)) => idx,
        None => panic!("Invalid item"),
    };

    let (v1, _) = mixed[(1_000 + cur_offset) % mixed.len()];
    let (v2, _) = mixed[(2_000 + cur_offset) % mixed.len()];
    let (v3, _) = mixed[(3_000 + cur_offset) % mixed.len()];

    Some(v1 + v2 + v3)
}

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    let encrypted = input
        .lines()
        .enumerate()
        .map(|(idx, l)| {
            (
                811_589_153
                    * l.parse::<i64>()
                        .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i),
                idx,
            )
        })
        .collect_vec();
    let mut mixed = mix(encrypted);

    for _ in 0..9 {
        mixed = mix(mixed);
    }

    let cur_offset = match mixed.iter().find_position(|&(v, _)| *v == 0) {
        Some((idx, _)) => idx,
        None => panic!("Invalid item"),
    };

    let (v1, _) = mixed[(1_000 + cur_offset) % mixed.len()];
    let (v2, _) = mixed[(2_000 + cur_offset) % mixed.len()];
    let (v3, _) = mixed[(3_000 + cur_offset) % mixed.len()];

    Some(v1 + v2 + v3)
}

fn mix(encrypted: Vec<(i64, usize)>) -> Vec<(i64, usize)> {
    let mut mixed = encrypted;

    for i in 0..mixed.len() {
        let (cur_offset, cur_val) = match mixed.iter().find_position(|(_, idx)| *idx == i) {
            Some((idx, val)) => (idx, *val),
            None => panic!("Invalid item"),
        };

        let new_offset = (cur_offset as i64 + cur_val.0) % (mixed.len() - 1) as i64;
        let new_offset_adjusted = if new_offset >= 0 {
            new_offset
        } else {
            mixed.len() as i64 - 1 + new_offset
        };

        mixed.remove(cur_offset);
        mixed.insert(
            usize::try_from(new_offset_adjusted)
                .map_or_else(|e| panic!("Invalid conversion: {e:?}"), |i| i),
            cur_val,
        );
    }

    mixed
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1_623_178_306));
    }
}
