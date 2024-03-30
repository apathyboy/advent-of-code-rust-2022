use std::collections::HashMap;

use glam::IVec2;

advent_of_code::solution!(17);

enum Jet {
    Left,
    Right,
}

const WIDTH: usize = 7;
const PIECES: [&[IVec2]; 5] = [
    // horizontal line
    &[
        IVec2::new(0, 0),
        IVec2::new(1, 0),
        IVec2::new(2, 0),
        IVec2::new(3, 0),
    ],
    // plus
    &[
        IVec2::new(0, 1),
        IVec2::new(1, 0),
        IVec2::new(1, 1),
        IVec2::new(1, 2),
        IVec2::new(2, 1),
    ],
    // J (or backwards L)
    &[
        IVec2::new(0, 0),
        IVec2::new(1, 0),
        IVec2::new(2, 0),
        IVec2::new(2, 1),
        IVec2::new(2, 2),
    ],
    // vertical line
    &[
        IVec2::new(0, 0),
        IVec2::new(0, 1),
        IVec2::new(0, 2),
        IVec2::new(0, 3),
    ],
    // square
    &[
        IVec2::new(0, 0),
        IVec2::new(1, 0),
        IVec2::new(0, 1),
        IVec2::new(1, 1),
    ],
];

#[derive(Default)]
struct State {
    jet_count: usize,
    piece_count: usize,
    top: usize,
    map: Vec<[bool; WIDTH]>,
    curr: IVec2,
    seen: HashMap<(usize, usize), (usize, usize, usize)>,
    added_by_repeats: usize,
}

impl State {
    fn is_valid(&mut self, new_curr: &IVec2, piece: &[IVec2]) -> bool {
        piece.iter().all(|offset| {
            let x = new_curr.x + offset.x;
            let y = new_curr.y + offset.y;
            while self.map.len() <= y as usize {
                self.map.push([false; WIDTH]);
            }
            (x as usize) < WIDTH && !self.map[y as usize][x as usize]
        })
    }
}

fn parse_jet_patterns(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|line| match line {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid jet stream direction"),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let target = 2022;
    let jets = parse_jet_patterns(input);
    let mut state = State::default();

    while state.piece_count != target {
        // new piece starts falling
        let piece = PIECES[state.piece_count % PIECES.len()];
        state.curr.x = 2;
        state.curr.y = (state.top + 3) as i32;

        // println!("== Piece {} begins falling ==", state.piece_count + 1);
        // println!("{}", state);

        loop {
            // jet
            let jet = &jets[state.jet_count % jets.len()];
            let new_curr = match jet {
                Jet::Left => IVec2::new(state.curr.x.saturating_sub(1), state.curr.y),
                Jet::Right => IVec2::new(state.curr.x + 1, state.curr.y),
            };
            if state.is_valid(&new_curr, piece) {
                state.curr = new_curr;
            }
            state.jet_count += 1;

            // println!(
            //     "Jet of gas pushes piece {}:",
            //     match jet {
            //         Jet::Left => "left",
            //         Jet::Right => "right",
            //     }
            // );
            // println!("{}", state);

            // fall
            let new_curr = IVec2::new(state.curr.x, state.curr.y.saturating_sub(1));
            if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                break;
            }
            state.curr = new_curr;

            // println!("Piece falls 1 unit:");
            // println!("{}", state);
        }

        // settle
        for offset in piece {
            let x = state.curr.x + offset.x;
            let y = state.curr.y + offset.y;
            while state.map.len() <= y as usize {
                state.map.push([false; WIDTH]);
            }
            state.map[y as usize][x as usize] = true;
            // y is 0 indexed.
            state.top = state.top.max(y as usize + 1);
        }

        // prepare for next iteration of while loop
        state.piece_count += 1;
    }

    Some(state.top)
}

pub fn part_two(input: &str) -> Option<usize> {
    let target = 1_000_000_000_000;
    let jets = parse_jet_patterns(input);
    let mut state = State::default();

    while state.piece_count != target {
        // new piece starts falling
        let piece = PIECES[state.piece_count % PIECES.len()];
        state.curr.x = 2;
        state.curr.y = (state.top + 3) as i32;

        loop {
            // jet
            let jet = &jets[state.jet_count % jets.len()];
            let new_curr = match jet {
                Jet::Left => IVec2::new(state.curr.x.saturating_sub(1), state.curr.y),
                Jet::Right => IVec2::new(state.curr.x + 1, state.curr.y),
            };
            if state.is_valid(&new_curr, piece) {
                state.curr = new_curr;
            }
            state.jet_count += 1;

            // fall
            let new_curr = IVec2::new(state.curr.x, state.curr.y.saturating_sub(1));
            if state.curr.y == 0 || !state.is_valid(&new_curr, piece) {
                break;
            }
            state.curr = new_curr;
        }

        // settle
        for offset in piece {
            let x = state.curr.x + offset.x;
            let y = state.curr.y + offset.y;
            while state.map.len() <= y as usize {
                state.map.push([false; WIDTH]);
            }
            state.map[y as usize][x as usize] = true;
            // y is 0 indexed
            state.top = state.top.max(y as usize + 1);
        }

        // look for cycle
        if state.added_by_repeats == 0 {
            let key = (
                state.piece_count % PIECES.len(),
                state.jet_count % jets.len(),
            );
            // at third occurrence of key, the values in the seen map repeat
            // add as many of them as possible without hitting the goal piece_count
            if let Some((2, old_piece_count, old_top)) = state.seen.get(&key) {
                let delta_top = state.top - old_top;
                let delta_piece_count = state.piece_count - old_piece_count;
                let repeats = (target - state.piece_count) / delta_piece_count;
                state.added_by_repeats += repeats * delta_top;
                state.piece_count += repeats * delta_piece_count;
            }
            // update seen map
            // key: (piece_count % PIECES.len(), jet_count % jets.len())
            // value: (amount_of_times_key_was_seen, piece_count, top)
            state
                .seen
                .entry(key)
                .and_modify(|(amnt, old_piece_count, old_top)| {
                    *amnt += 1;
                    *old_piece_count = state.piece_count;
                    *old_top = state.top;
                })
                .or_insert((1, state.piece_count, state.top));
        }

        // prepare for next iteration of while loop
        state.piece_count += 1;
    }

    Some(state.top + state.added_by_repeats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
