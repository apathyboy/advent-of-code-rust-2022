use itertools::Itertools;

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys = input.lines().map(parse_monkey).collect_vec();

    loop {
        for i in 0..monkeys.len() {
            if monkeys[i].id == "root" && monkeys[i].op == "VAL" {
                return monkeys[i].val;
            } else if monkeys[i].op == "VAL" {
                for j in 0..monkeys.len() {
                    if monkeys[j].left == monkeys[i].id && monkeys[j].left_val.is_none() {
                        monkeys[j].left_val = Some(monkeys[i].val.unwrap());
                    }
                    if monkeys[j].right == monkeys[i].id && monkeys[j].right_val.is_none() {
                        monkeys[j].right_val = Some(monkeys[i].val.unwrap());
                    }
                }
            } else if monkeys[i].left_val.is_some() && monkeys[i].right_val.is_some() {
                monkeys[i].val = if monkeys[i].op == "*" {
                    Some(monkeys[i].left_val.unwrap() * monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "/" {
                    Some(monkeys[i].left_val.unwrap() / monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "+" {
                    Some(monkeys[i].left_val.unwrap() + monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "-" {
                    Some(monkeys[i].left_val.unwrap() - monkeys[i].right_val.unwrap())
                } else {
                    None
                };
                monkeys[i].op = "VAL".to_string();
            }
        }
    }
}

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_two(input: &str) -> Option<i64> {
    let mut monkeys = input.lines().map(parse_monkey).collect_vec();

    let mut humn_idx = 0;
    let mut humn_val = 1;

    for i in 0..monkeys.len() {
        if monkeys[i].id == "root" {
            monkeys[i].op = "=".to_string();
        } else if monkeys[i].id == "humn" {
            humn_idx = i;
        }
    }

    loop {
        monkeys[humn_idx].val = Some(humn_val);

        if run_test(monkeys.clone()) {
            return Some(humn_val);
        }

        humn_val += 1;
    }
}

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

fn run_test(mut monkeys: Vec<Monkey>) -> bool {
    loop {
        for i in 0..monkeys.len() {
            if monkeys[i].id == "root"
                && monkeys[i].left_val.is_some()
                && monkeys[i].right_val.is_some()
            {
                return monkeys[i].left_val.unwrap() == monkeys[i].right_val.unwrap();
            } else if monkeys[i].op == "VAL" {
                for j in 0..monkeys.len() {
                    if monkeys[j].left == monkeys[i].id && monkeys[j].left_val.is_none() {
                        monkeys[j].left_val = Some(monkeys[i].val.unwrap());
                    }
                    if monkeys[j].right == monkeys[i].id && monkeys[j].right_val.is_none() {
                        monkeys[j].right_val = Some(monkeys[i].val.unwrap());
                    }
                }
            } else if monkeys[i].left_val.is_some() && monkeys[i].right_val.is_some() {
                monkeys[i].val = if monkeys[i].op == "*" {
                    Some(monkeys[i].left_val.unwrap() * monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "/" {
                    Some(monkeys[i].left_val.unwrap() / monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "+" {
                    Some(monkeys[i].left_val.unwrap() + monkeys[i].right_val.unwrap())
                } else if monkeys[i].op == "-" {
                    Some(monkeys[i].left_val.unwrap() - monkeys[i].right_val.unwrap())
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

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
