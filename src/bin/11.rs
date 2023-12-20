use itertools::Itertools;

advent_of_code::solution!(11);

struct Monkey {
    items: Vec<u64>,
    true_target: usize,
    false_target: usize,
    test_val: u64,
    operation: String,
    inspected_count: usize,
}

fn perform_operation(operation: &str, old_val: u64) -> u64 {
    let (first, rest) = operation
        .split_once(' ')
        .map_or_else(|| panic!("Invalid format"), |i| i);
    let (op, second) = rest
        .split_once(' ')
        .map_or_else(|| panic!("Invalid format"), |i| i);

    let left_val = match first {
        "old" => old_val,
        _ => first
            .parse::<u64>()
            .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i),
    };

    let right_val = match second {
        "old" => old_val,
        _ => second
            .parse::<u64>()
            .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i),
    };

    if op == "+" {
        left_val + right_val
    } else {
        left_val * right_val
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    input
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|s| -> Monkey {
            let lines = s.lines().collect_vec();

            let items = lines[1][18..]
                .split(", ")
                .map(|s| {
                    s.parse::<u64>()
                        .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i)
                })
                .sorted()
                .collect_vec();
            let true_target = lines[4][29..]
                .parse::<usize>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i);
            let false_target = lines[5][30..]
                .parse::<usize>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i);
            let test_val = lines[3][21..]
                .parse::<u64>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |i| i);
            let operation = lines[2][19..].to_string();

            Monkey {
                items,
                true_target,
                false_target,
                test_val,
                operation,
                inspected_count: 0,
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            monkeys[i].items.sort_by(|a, b| b.cmp(a));

            while let Some(mut item) = monkeys[i].items.pop() {
                monkeys[i].inspected_count += 1;
                item = perform_operation(&(monkeys[i].operation), item);
                item /= 3;

                if item % monkeys[i].test_val == 0 {
                    let true_target = monkeys[i].true_target;
                    monkeys[true_target].items.push(item);
                } else {
                    let false_target = monkeys[i].false_target;
                    monkeys[false_target].items.push(item);
                }
            }
        }
    }

    let monkey_business_level = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(2)
        .reduce(|accum, item| accum * item)
        .map_or_else(|| panic!("Problem reducing vector"), |i| i);

    Some(monkey_business_level)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut monkeys = parse(input);

    let gcd = monkeys
        .iter()
        .map(|m| m.test_val)
        .reduce(|accum, item| accum * item)
        .map_or_else(|| panic!("Problem reducing vector"), |i| i);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            monkeys[i].items.sort_by(|a, b| b.cmp(a));

            while let Some(mut item) = monkeys[i].items.pop() {
                monkeys[i].inspected_count += 1;
                item = perform_operation(&(monkeys[i].operation), item);

                if item > gcd {
                    item %= gcd;
                }

                if item % monkeys[i].test_val == 0 {
                    let true_target = monkeys[i].true_target;
                    monkeys[true_target].items.push(item);
                } else {
                    let false_target = monkeys[i].false_target;
                    monkeys[false_target].items.push(item);
                }
            }
        }
    }

    let monkey_business_level = monkeys
        .iter()
        .map(|m| m.inspected_count)
        .sorted_by(|a, b| Ord::cmp(&b, &a))
        .take(2)
        .reduce(|accum, item| accum * item)
        .map_or_else(|| panic!("Problem reducing vector"), |i| i);

    Some(monkey_business_level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(2_713_310_158));
    }
}
