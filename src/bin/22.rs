/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub const fn part_one(_input: &str) -> Option<u32> {
    //    let (map, _moves) = parse(input);
    //
    //    let _cur_pos = find_start(&map);
    //
    //    todo!()
    None
}

#[must_use]
pub const fn part_two(_input: &str) -> Option<u32> {
    None
}

//fn find_start(map: &[Tile]) -> (usize, usize) {
//    let starting_tile = map
//        .iter()
//        .find(|t| t.tile_type == TileType::Open)
//        .map_or_else(|| panic!("Couldn't find starting position"), |s| s);
//
//    (starting_tile.x, starting_tile.y)
//}
//
//fn parse_map(input: &str) -> Vec<Tile> {
//    input
//        .lines()
//        .enumerate()
//        .flat_map(|(y, l)| {
//            l.chars()
//                .enumerate()
//                .map(|(x, c)| match c {
//                    ' ' => Tile {
//                        x,
//                        y,
//                        tile_type: TileType::Empty,
//                    },
//                    '.' => Tile {
//                        x,
//                        y,
//                        tile_type: TileType::Open,
//                    },
//                    '#' => Tile {
//                        x,
//                        y,
//                        tile_type: TileType::Wall,
//                    },
//                    _ => panic!("Invalid format"),
//                })
//                .collect_vec()
//        })
//        .collect_vec()
//}
//
//fn parse_moves(input: &str) -> Vec<Move> {
//    let mut moves = Vec::new();
//
//    for chunk in input.split_inclusive(char::is_alphabetic) {
//        let dir_pos = chunk.find(char::is_alphabetic);
//
//        if dir_pos.is_none() {
//            moves.push(Move {
//                move_type: MoveType::Forward,
//                dir: None,
//                distance: chunk.parse().ok(),
//            });
//        } else {
//            let dir_pos = dir_pos.map_or_else(|| panic!("Invalid format"), |p| p);
//            let (dist_move_str, dir_move_str) = chunk.split_at(dir_pos);
//
//            moves.push(Move {
//                move_type: MoveType::Forward,
//                dir: None,
//                distance: dist_move_str.parse().ok(),
//            });
//            moves.push(Move {
//                move_type: MoveType::Turn,
//                dir: Some(dir_move_str.to_string()),
//                distance: None,
//            });
//        }
//    }
//
//    moves
//}
//
//fn parse(input: &str) -> (Vec<Tile>, Vec<Move>) {
//    input
//        .split_once("\n\n")
//        .map(|(map_str, moves_str)| (parse_map(map_str), parse_moves(moves_str)))
//        .map_or_else(|| panic!("Invalid format"), |t| t)
//}
//
//#[derive(PartialEq)]
//enum TileType {
//    Empty,
//    Open,
//    Wall,
//}
//
//struct Tile {
//    x: usize,
//    y: usize,
//    tile_type: TileType,
//}
//
//enum MoveType {
//    Turn,
//    Forward,
//}
//
//struct Move {
//    move_type: MoveType,
//    dir: Option<String>,
//    distance: Option<u32>,
//}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}
