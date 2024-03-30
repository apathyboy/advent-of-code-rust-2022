use std::collections::{BTreeMap, BTreeSet};

advent_of_code::solution!(23);

const ROUNDS: i32 = 10;

fn parse_input(contents: &str) -> BTreeSet<(i32, i32)> {
    let mut elves: BTreeSet<(i32, i32)> = BTreeSet::new();
    contents.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, ch)| {
            if ch == '#' {
                elves.insert((row as i32, col as i32));
            }
        });
    });
    elves
}

fn propose_move(round: i32, pos: (i32, i32), elves: &BTreeSet<(i32, i32)>) -> (i32, i32) {
    let (row, col) = pos;
    let neighbors = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];
    if neighbors
        .iter()
        .all(|&p| (p == pos) || (!elves.contains(&p)))
    {
        return pos;
    } else {
        #[allow(clippy::type_complexity)]
        let moves: Vec<(Vec<(i32, i32)>, (i32, i32))> = vec![
            (
                vec![(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)],
                (row - 1, col),
            ),
            (
                vec![(row + 1, col - 1), (row + 1, col), (row + 1, col + 1)],
                (row + 1, col),
            ),
            (
                vec![(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)],
                (row, col - 1),
            ),
            (
                vec![(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)],
                (row, col + 1),
            ),
        ];
        for i in 0..4 {
            if moves[((i + round) % 4) as usize]
                .0
                .iter()
                .all(|&p| (!elves.contains(&p)))
            {
                return moves[((i + round) % 4) as usize].1;
            }
        }
    }
    pos
}

fn simulate(elves: &mut BTreeSet<(i32, i32)>) -> i32 {
    for round in 0..ROUNDS {
        let mut proposals: BTreeMap<(i32, i32), Vec<(i32, i32)>> = BTreeMap::new();
        for pos in elves.iter() {
            let proposal = propose_move(round, *pos, elves);
            proposals
                .entry(proposal)
                .and_modify(|candidates| {
                    candidates.push(*pos);
                })
                .or_insert_with(|| vec![*pos]);
        }
        let mut elves_new: BTreeSet<(i32, i32)> = BTreeSet::new();
        proposals.iter().for_each(|(k, v)| {
            if v.len() == 1 {
                elves_new.insert(*k);
            } else {
                v.iter().for_each(|p| {
                    elves_new.insert(*p);
                });
            }
        });
        *elves = elves_new;
    }
    let r_extent = elves.iter().map(|(r, _)| *r).max().unwrap()
        - elves.iter().map(|(r, _)| *r).min().unwrap()
        + 1;
    let c_extent = elves.iter().map(|(_, c)| *c).max().unwrap()
        - elves.iter().map(|(_, c)| *c).min().unwrap()
        + 1;
    r_extent * c_extent - (elves.len() as i32)
}

fn propose_move2(round: i32, pos: (i32, i32), elves: &BTreeSet<(i32, i32)>) -> (i32, i32) {
    let (row, col) = pos;
    let neighbors = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];
    if neighbors
        .iter()
        .all(|&p| (p == pos) || (!elves.contains(&p)))
    {
        return pos;
    } else {
        #[allow(clippy::type_complexity)]
        let moves: Vec<(Vec<(i32, i32)>, (i32, i32))> = vec![
            (
                vec![(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)],
                (row - 1, col),
            ),
            (
                vec![(row + 1, col - 1), (row + 1, col), (row + 1, col + 1)],
                (row + 1, col),
            ),
            (
                vec![(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)],
                (row, col - 1),
            ),
            (
                vec![(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)],
                (row, col + 1),
            ),
        ];
        for i in 0..4 {
            if moves[((i + round) % 4) as usize]
                .0
                .iter()
                .all(|&p| (!elves.contains(&p)))
            {
                return moves[((i + round) % 4) as usize].1;
            }
        }
    }
    pos
}

fn simulate2(elves: &mut BTreeSet<(i32, i32)>) -> i32 {
    let mut round: i32 = 0;
    loop {
        let mut proposals: BTreeMap<(i32, i32), Vec<(i32, i32)>> = BTreeMap::new();
        for pos in elves.iter() {
            let proposal = propose_move2(round, *pos, elves);
            proposals
                .entry(proposal)
                .and_modify(|candidates| {
                    candidates.push(*pos);
                })
                .or_insert_with(|| vec![*pos]);
        }
        let mut elves_new: BTreeSet<(i32, i32)> = BTreeSet::new();
        proposals.iter().for_each(|(k, v)| {
            if v.len() == 1 {
                elves_new.insert(*k);
            } else {
                v.iter().for_each(|p| {
                    elves_new.insert(*p);
                });
            }
        });
        if elves_new == *elves {
            break;
        }
        *elves = elves_new;
        round += 1;
    }
    round + 1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut elves = parse_input(input);
    let count = simulate(&mut elves);

    Some(count as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = parse_input(input);
    let count = simulate2(&mut elves);

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(20));
    }
}
