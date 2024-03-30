use std::collections::HashMap;

advent_of_code::solution!(22);

pub type Grid = HashMap<(u32, u32), char>;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    TurnLeft,
    TurnRight,
    Advance(u32),
}

#[derive(Debug)]
pub struct Map {
    pub grid: Grid,
    pub start: (u32, u32),
    pub moves: Vec<Move>,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    pub fn to_u32(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[allow(unused_assignments)]
fn parse_input(contents: &str) -> Map {
    let mut grid: Grid = Grid::new();
    let mut moves: Vec<Move> = vec![];
    let mut row: u32 = 1;
    let mut col: u32 = 1;
    let mut lines = contents.lines();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        for c in line.chars() {
            if c == ' ' {
                // ignore
            } else if c == '#' || c == '.' {
                grid.insert((row, col), c);
            } else {
                panic!("parse_input(): unexpected char in input: '{}'", c);
            }
            col += 1;
        }
        row += 1;
        col = 1;
    }
    let start = grid.keys().cloned().min().unwrap();
    let mut move_chars = lines.next().unwrap().chars();
    let mut num_str = String::new();
    let mut reading_num = false;
    loop {
        let maybe_ch = move_chars.next();
        if maybe_ch.is_none() {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
            }
            break;
        }
        let ch = maybe_ch.unwrap();
        if ch == 'L' {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
                reading_num = false;
            }
            moves.push(Move::TurnLeft);
        } else if ch == 'R' {
            if reading_num {
                let n = num_str.parse::<u32>().unwrap();
                num_str = String::new();
                moves.push(Move::Advance(n));
                reading_num = false;
            }
            moves.push(Move::TurnRight);
        } else if ch.is_ascii_digit() {
            reading_num = true;
            num_str.push(ch);
        }
    }

    Map { grid, start, moves }
}

fn solve(map: &Map) -> u32 {
    let mut pos = map.start;
    let mut new_pos: (u32, u32);
    let mut facing = Direction::Right;
    let turn: HashMap<(Move, Direction), Direction> = HashMap::from([
        ((Move::TurnRight, Direction::Right), Direction::Down),
        ((Move::TurnRight, Direction::Down), Direction::Left),
        ((Move::TurnRight, Direction::Left), Direction::Up),
        ((Move::TurnRight, Direction::Up), Direction::Right),
        ((Move::TurnLeft, Direction::Right), Direction::Up),
        ((Move::TurnLeft, Direction::Down), Direction::Right),
        ((Move::TurnLeft, Direction::Left), Direction::Down),
        ((Move::TurnLeft, Direction::Up), Direction::Left),
    ]);
    for m in map.moves.iter().cloned() {
        match m {
            Move::TurnRight | Move::TurnLeft => {
                facing = *turn.get(&(m, facing)).unwrap();
            }
            Move::Advance(n) => {
                for _ in 0..n {
                    match facing {
                        Direction::Right => {
                            new_pos = (pos.0, pos.1 + 1);
                            if !map.grid.contains_key(&new_pos) {
                                new_pos = (
                                    pos.0,
                                    map.grid
                                        .keys()
                                        .filter(|(r, _)| *r == pos.0)
                                        .map(|(_, c)| *c)
                                        .min()
                                        .unwrap(),
                                );
                            }
                        }
                        Direction::Down => {
                            new_pos = (pos.0 + 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                new_pos = (
                                    map.grid
                                        .keys()
                                        .filter(|(_, c)| *c == pos.1)
                                        .map(|(r, _)| *r)
                                        .min()
                                        .unwrap(),
                                    pos.1,
                                );
                            }
                        }
                        Direction::Left => {
                            new_pos = (pos.0, pos.1 - 1);
                            if !map.grid.contains_key(&new_pos) {
                                new_pos = (
                                    pos.0,
                                    map.grid
                                        .keys()
                                        .filter(|(r, _)| *r == pos.0)
                                        .map(|(_, c)| *c)
                                        .max()
                                        .unwrap(),
                                );
                            }
                        }
                        Direction::Up => {
                            new_pos = (pos.0 - 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                new_pos = (
                                    map.grid
                                        .keys()
                                        .filter(|(_, c)| *c == pos.1)
                                        .map(|(r, _)| *r)
                                        .max()
                                        .unwrap(),
                                    pos.1,
                                );
                            }
                        }
                    }
                    let ch = map.grid.get(&new_pos).unwrap();
                    if *ch == '.' {
                        pos = new_pos;
                    } else if *ch == '#' {
                        break;
                    }
                }
            }
        }
    }

    1000 * pos.0 + 4 * pos.1 + facing.to_u32()
}

fn wrap_input(pos: (u32, u32), facing: Direction) -> ((u32, u32), Direction) {
    let (row, col) = pos;
    if (1..=50).contains(&row) && (51..=100).contains(&col) {
        // face 1
        if facing == Direction::Left {
            ((151 - row, 1), Direction::Right)
        } else if facing == Direction::Up {
            ((col + 100, 1), Direction::Right)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if (1..=50).contains(&row) && (101..=150).contains(&col) {
        // face 2
        if facing == Direction::Right {
            ((151 - row, 100), Direction::Left)
        } else if facing == Direction::Down {
            ((col - 50, 100), Direction::Left)
        } else if facing == Direction::Up {
            ((200, col - 100), Direction::Up)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if (51..=100).contains(&row) && (51..=100).contains(&col) {
        // face 3
        if facing == Direction::Right {
            ((50, row + 50), Direction::Up)
        } else if facing == Direction::Left {
            ((101, row - 50), Direction::Down)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if (51..=100).contains(&row) && (51..=100).contains(&col) {
        // face 4
        if facing == Direction::Right {
            ((151 - row, 150), Direction::Left)
        } else if facing == Direction::Down {
            ((col + 100, 50), Direction::Left)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if (101..=150).contains(&row) && (1..=50).contains(&col) {
        // face 5
        if facing == Direction::Left {
            ((151 - row, 51), Direction::Right)
        } else if facing == Direction::Up {
            ((col + 50, 51), Direction::Right)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else if (151..=200).contains(&row) && (1..=50).contains(&col) {
        // face 6
        if facing == Direction::Right {
            ((150, row - 100), Direction::Up)
        } else if facing == Direction::Down {
            ((1, col + 100), Direction::Down)
        } else if facing == Direction::Left {
            ((1, row - 100), Direction::Down)
        } else {
            panic!(
                "wrap_example(): bug: pos=({}, {}), facing={:?}",
                row, col, facing
            );
        }
    } else {
        panic!(
            "wrap_example(): bug: pos=({}, {}), facing={:?}",
            row, col, facing
        );
    }
}

fn solve2(map: &Map) -> u32 {
    let mut pos = map.start;
    let mut new_pos: (u32, u32);
    let mut facing = Direction::Right;
    let turn: HashMap<(Move, Direction), Direction> = HashMap::from([
        ((Move::TurnRight, Direction::Right), Direction::Down),
        ((Move::TurnRight, Direction::Down), Direction::Left),
        ((Move::TurnRight, Direction::Left), Direction::Up),
        ((Move::TurnRight, Direction::Up), Direction::Right),
        ((Move::TurnLeft, Direction::Right), Direction::Up),
        ((Move::TurnLeft, Direction::Down), Direction::Right),
        ((Move::TurnLeft, Direction::Left), Direction::Down),
        ((Move::TurnLeft, Direction::Up), Direction::Left),
    ]);
    for m in map.moves.iter().cloned() {
        match m {
            Move::TurnRight | Move::TurnLeft => {
                facing = *turn.get(&(m, facing)).unwrap();
            }
            Move::Advance(n) => {
                for _ in 0..n {
                    let mut new_facing = facing;
                    match facing {
                        Direction::Right => {
                            new_pos = (pos.0, pos.1 + 1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Down => {
                            new_pos = (pos.0 + 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Left => {
                            new_pos = (pos.0, pos.1 - 1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                        Direction::Up => {
                            new_pos = (pos.0 - 1, pos.1);
                            if !map.grid.contains_key(&new_pos) {
                                (new_pos, new_facing) = wrap_input(pos, facing);
                            }
                        }
                    }
                    let ch = map.grid.get(&new_pos).unwrap();
                    if *ch == '.' {
                        pos = new_pos;
                        facing = new_facing;
                    } else if *ch == '#' {
                        break;
                    }
                }
            }
        }
    }
    1000 * pos.0 + 4 * pos.1 + facing.to_u32()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let password = solve(&map);

    Some(password)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse_input(input);
    let password = solve2(&map);

    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(6032));
    }
}
