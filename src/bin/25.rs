use itertools::Itertools;

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_one(input: &str) -> Option<String> {
    Some(convert(parse_fuel_requirements(input)))
}

#[must_use]
pub const fn part_two(_input: &str) -> Option<u32> {
    None
}

fn convert(val: i64) -> String {
    let mut remainders: Vec<i64> = Vec::new();
    let pows = create_pows(5, 22);

    let (position, _) = pows
        .iter()
        .find_position(|&p| *p > val)
        .map_or_else(|| panic!("Not enough pows generated"), |i| i);
    let pows = pows.iter().take(position + 1).rev().collect_vec();

    let mut quotient = val;

    for (i, &p) in pows.iter().enumerate() {
        let r = quotient / *p;

        if r <= 2 {
            remainders.push(r);
        } else {
            // handle borrowing from higher power
            // todo: generalize the borrowing
            if i == 0 {
                remainders.push(1);
            } else {
                remainders[i - 1] += 1;
                if remainders[i - 1] == 3 {
                    remainders[i - 1] = -2;

                    if i >= 2 {
                        remainders[i - 2] += 1;
                        if remainders[i - 2] == 3 {
                            remainders[i - 2] = -2;

                            if i >= 3 {
                                remainders[i - 3] += 1;
                            }
                        }
                    }
                }
            }

            quotient += pows[i - 1];

            if r == 3 {
                remainders.push(-2);
            } else {
                remainders.push(-1);
            }
        }

        quotient %= *p;
    }

    remainders
        .iter()
        .skip_while(|&i| *i == 0)
        .map(|i| -> String {
            if *i >= 0 {
                i.to_string()
            } else if *i == -2 {
                "=".to_string()
            } else {
                "-".to_string()
            }
        })
        .collect::<String>()
}

fn create_pows(pow: i64, max_exp: usize) -> Vec<i64> {
    (0..max_exp)
        .map(|exp| pow.pow(u32::try_from(exp).map_or_else(|e| panic!("Invalid: {e:?}"), |i| i)))
        .collect()
}

fn parse_fuel_requirements(input: &str) -> i64 {
    input
        .lines()
        .map(|l| {
            l.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    let i = u32::try_from(i).map_or_else(|e| panic!("Invalid: {e:?}"), |i| i);
                    let place: i64 = 5;
                    let place = place.pow(i);

                    if c.is_numeric() {
                        c.to_digit(10).map_or_else(
                            || panic!("Invalid input"),
                            |t| i64::try_from(t).map_or_else(|e| panic!("Invalid: {e:?}"), |i| i),
                        ) * place
                    } else {
                        match c {
                            '-' => -place,
                            '=' => -2 * place,
                            _ => panic!("Invalid input"),
                        }
                    }
                })
                .sum::<i64>()
        })
        .sum()
}

// 2=-0=1-0012-=-2=0=01
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
        assert_eq!(convert(1), "1".to_string());
        assert_eq!(convert(2), "2".to_string());
        assert_eq!(convert(3), "1=".to_string());
        assert_eq!(convert(4), "1-".to_string());
        assert_eq!(convert(5), "10".to_string());
        assert_eq!(convert(6), "11".to_string());
        assert_eq!(convert(7), "12".to_string());
        assert_eq!(convert(8), "2=".to_string());
        assert_eq!(convert(9), "2-".to_string());
        assert_eq!(convert(10), "20".to_string());
        assert_eq!(convert(15), "1=0".to_string());
        assert_eq!(convert(20), "1-0".to_string());
        assert_eq!(convert(2022), "1=11-2".to_string());
        assert_eq!(convert(12345), "1-0---0".to_string());
        assert_eq!(convert(314_159_265), "1121-1110-1=0".to_string());
    }
}
