use std::collections::HashMap;

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_one(input: &str) -> Option<String> {
    let fuel_requirements = parse_fuel_requirements(input);
    let snafu_val = to_snafu(fuel_requirements);

    Some(snafu_val)
}

#[must_use]
pub const fn part_two(_input: &str) -> Option<u32> {
    None
}

fn to_snafu(val: i64) -> String {
    let mut snafu = String::new();
    let mut n = val;
    let s: HashMap<i64, char> = HashMap::from([(3, '='), (4, '-'), (0, '0'), (1, '1'), (2, '2')]);

    while n != 0 {
        let rem = n % 5;
        let digit = s[&rem];

        snafu.push(digit);

        // snafu is base 5, shifted by 2
        n = (n + 2) / 5;
    }

    snafu.chars().rev().collect::<String>()
}

fn from_snafu(input: &str) -> i64 {
    let d: HashMap<char, i64> = HashMap::from([('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)]);

    input
        .chars()
        .rev()
        .enumerate()
        .map(|(i, c)| -> i64 {
            let exp = u32::try_from(i).map_or_else(|e| panic!("Invalid: {e:?}"), |i| i);

            d[&c] * 5i64.pow(exp)
        })
        .sum::<i64>()
}

fn parse_fuel_requirements(input: &str) -> i64 {
    input.lines().map(from_snafu).sum()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some("2=-1=0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_conversions() {
        assert_eq!(to_snafu(1), "1".to_string());
        assert_eq!(to_snafu(2), "2".to_string());
        assert_eq!(to_snafu(3), "1=".to_string());
        assert_eq!(to_snafu(4), "1-".to_string());
        assert_eq!(to_snafu(5), "10".to_string());
        assert_eq!(to_snafu(6), "11".to_string());
        assert_eq!(to_snafu(7), "12".to_string());
        assert_eq!(to_snafu(8), "2=".to_string());
        assert_eq!(to_snafu(9), "2-".to_string());
        assert_eq!(to_snafu(10), "20".to_string());
        assert_eq!(to_snafu(15), "1=0".to_string());
        assert_eq!(to_snafu(20), "1-0".to_string());
        assert_eq!(to_snafu(2022), "1=11-2".to_string());
        assert_eq!(to_snafu(12345), "1-0---0".to_string());
        assert_eq!(to_snafu(314_159_265), "1121-1110-1=0".to_string());
    }
}
