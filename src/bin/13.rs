use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

pub fn part_one(input: &str) -> Option<u32> {
    let sum_of_indices = parse(input)
        .iter()
        .tuples()
        .enumerate()
        .filter_map(|(idx, (left, right))| {
            if left < right {
                Some(idx as u32 + 1)
            } else {
                None
            }
        })
        .sum();

    Some(sum_of_indices)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut packets = parse(input);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.extend_from_slice(&[div1.clone(), div2.clone()]);
    packets.sort();

    let pos1 = packets.iter().position(|e| e == &div1).unwrap() + 1;
    let pos2 = packets.iter().position(|e| e == &div2).unwrap() + 1;

    Some((pos1 * pos2) as u32)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Int(i64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(a), Packet::Int(b)) => a.cmp(b),
            (Packet::Int(a), Packet::List(b)) => vec![Packet::Int(*a)].cmp(b),
            (Packet::List(a), Packet::Int(b)) => a.cmp(&vec![Packet::Int(*b)]),
            (Packet::List(a), Packet::List(b)) => a.cmp(b),
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(i64, Packet::Int),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
    ))(input)
}

fn parse(input: &str) -> Vec<Packet> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_packet(l).unwrap().1)
        .collect()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
