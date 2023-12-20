use itertools::Itertools;
use std::cmp::Ordering;

advent_of_code::solution!(21);

#[derive(Clone)]
struct Monkey {
    id: String,
    op: String,
    left: String,
    left_val: Option<i64>,
    right: String,
    right_val: Option<i64>,
    val: Option<i64>,
}

fn run_test(mut monkeys: Vec<Monkey>) -> (i64, i64) {
    loop {
        for i in 0..monkeys.len() {
            if monkeys[i].id == "root"
                && monkeys[i].left_val.is_some()
                && monkeys[i].right_val.is_some()
            {
                return (
                    monkeys[i]
                        .left_val
                        .map_or_else(|| panic!("Expected value"), |v| v),
                    monkeys[i]
                        .right_val
                        .map_or_else(|| panic!("Expected value"), |v| v),
                );
            } else if monkeys[i].op == "VAL" {
                for j in 0..monkeys.len() {
                    if monkeys[j].left == monkeys[i].id && monkeys[j].left_val.is_none() {
                        monkeys[j].left_val = Some(
                            monkeys[i]
                                .val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                        );
                    }
                    if monkeys[j].right == monkeys[i].id && monkeys[j].right_val.is_none() {
                        monkeys[j].right_val = Some(
                            monkeys[i]
                                .val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                        );
                    }
                }
            } else if monkeys[i].left_val.is_some() && monkeys[i].right_val.is_some() {
                monkeys[i].val = if monkeys[i].op == "*" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            * monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "/" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            / monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "+" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            + monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "-" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            - monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else {
                    None
                };
                monkeys[i].op = "VAL".to_string();
            }
        }
    }
}

fn parse_monkey(line: &str) -> Monkey {
    let id = line[0..4].to_string();

    if line
        .chars()
        .nth(6)
        .map_or_else(|| panic!("Invalid format"), |c| c)
        .is_numeric()
    {
        Monkey {
            id,
            op: "VAL".to_string(),
            left: String::new(),
            left_val: None,
            right: String::new(),
            right_val: None,
            val: line[6..].parse().ok(),
        }
    } else {
        Monkey {
            id,
            op: line[11..12].to_string(),
            left: line[6..10].to_string(),
            left_val: None,
            right: line[13..].to_string(),
            right_val: None,
            val: None,
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = input.lines().map(parse_monkey).collect_vec();

    loop {
        for i in 0..monkeys.len() {
            if monkeys[i].id == "root" && monkeys[i].op == "VAL" {
                return monkeys[i].val;
            } else if monkeys[i].op == "VAL" {
                for j in 0..monkeys.len() {
                    if monkeys[j].left == monkeys[i].id && monkeys[j].left_val.is_none() {
                        monkeys[j].left_val = Some(
                            monkeys[i]
                                .val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                        );
                    }
                    if monkeys[j].right == monkeys[i].id && monkeys[j].right_val.is_none() {
                        monkeys[j].right_val = Some(
                            monkeys[i]
                                .val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                        );
                    }
                }
            } else if monkeys[i].left_val.is_some() && monkeys[i].right_val.is_some() {
                monkeys[i].val = if monkeys[i].op == "*" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            * monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "/" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            / monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "+" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            + monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else if monkeys[i].op == "-" {
                    Some(
                        monkeys[i]
                            .left_val
                            .map_or_else(|| panic!("Expected value"), |v| v)
                            - monkeys[i]
                                .right_val
                                .map_or_else(|| panic!("Expected value"), |v| v),
                    )
                } else {
                    None
                };
                monkeys[i].op = "VAL".to_string();
            }
        }
    }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = input.lines().map(parse_monkey).collect_vec();

    let mut humn_idx = 0;
    let mut lo_val = 0;

    for (i, monkey) in monkeys.iter_mut().enumerate() {
        if monkey.id == "root" {
            monkey.op = "=".to_string();
        } else if monkey.id == "humn" {
            humn_idx = i;
        }
    }

    let mut high_val = 10_000_000_000_000;

    monkeys[humn_idx].val = Some(lo_val);
    let (t1, _) = run_test(monkeys.clone());
    monkeys[humn_idx].val = Some(high_val);
    let (t2, _) = run_test(monkeys.clone());

    while lo_val != high_val {
        let mid = (lo_val + high_val) / 2;

        monkeys[humn_idx].val = Some(mid);

        let (mut left_val, mut right_val) = run_test(monkeys.clone());

        if t1 < t2 {
            (left_val, right_val) = (right_val, left_val);
        }

        match left_val.cmp(&right_val) {
            Ordering::Greater => lo_val = mid + 1,
            Ordering::Less => high_val = mid - 1,
            Ordering::Equal => high_val = mid,
        }

        //if left_val > right_val {
        //    lo_val = mid + 1;
        //} else if left_val < right_val {
        //    high_val = mid - 1;
        //} else {
        //    high_val = mid;
        //}

        //if left_val < 0 || left_val > right_val {
        //    humn_val /= 2;
        //} else {
        //    humn_val += 1;
        //}
    }

    Some(lo_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(301));
    }
}
