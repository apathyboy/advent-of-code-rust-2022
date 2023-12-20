use itertools::Itertools;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::i64, combinator::map,
    multi::separated_list0, sequence::delimited, IResult,
};

advent_of_code::solution!(13);

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Int(i64),
    List(Vec<Self>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Int(a), Self::List(b)) => vec![Self::Int(*a)].cmp(b),
            (Self::List(a), Self::Int(b)) => a.cmp(&vec![Self::Int(*b)]),
            (Self::List(a), Self::List(b)) => a.cmp(b),
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
        .map(|l| {
            parse_packet(l)
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |p| p)
                .1
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let sum_of_indices = parse(input)
        .iter()
        .tuples()
        .enumerate()
        .filter_map(
            |(idx, (left, right))| {
                if left < right {
                    Some(idx + 1)
                } else {
                    None
                }
            },
        )
        .sum();

    Some(sum_of_indices)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut packets = parse(input);

    let div1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
    let div2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

    packets.extend_from_slice(&[div1.clone(), div2.clone()]);
    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(idx, p)| {
            if *p == div1 || *p == div2 {
                Some(idx + 1)
            } else {
                None
            }
        })
        .reduce(|a, b| a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(140));
    }
}
