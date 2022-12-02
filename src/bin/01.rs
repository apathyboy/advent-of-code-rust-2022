pub fn part_one(input: &str) -> Option<u32> {
    let mut largest: u32 = 0;

    let split = input.split("\n\n");
    for s in split {
        let mut total: u32 = 0;
        for s1 in s.lines() {
            total += s1.parse::<u32>().unwrap()
        }

        if total > largest {
            largest = total;
        }
    }

    Some(largest)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calorie_totals = Vec::<u32>::new();
    let split = input.split("\n\n");
    for s in split {
        let mut total: u32 = 0;
        for s1 in s.lines() {
            total += s1.parse::<u32>().unwrap()
        }
        calorie_totals.push(total);
    }

    calorie_totals.sort();

    let sum_top_three: u32 = calorie_totals.iter().rev().take(3).sum();

    Some(sum_top_three)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
