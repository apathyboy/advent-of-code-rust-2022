use itertools::Itertools;

// 2,2,2 -> 0 + 0 + 0 = 0
// 1,2,2 -> 1 + 2 + 2 = 5
// 3,2,2 -> 1 + 2 + 2 = 5
// 2,1,2 -> 2 + 1 + 2 = 5
// 2,3,2 -> 2 + 1 + 2 = 5
// 2,2,1 -> 2 + 2 + 1 = 5
// 2,2,3 -> 2 + 2 + 0 = 4
// 2,2,4 -> 2 + 2 + 1 = 5
// 2,2,6 -> 2 + 2 + 2 = 6
// 1,2,5 -> 2 + 2 + 2 = 6
// 3,2,5 -> 2 + 2 + 2 = 6
// 2,1,5 -> 2 + 2 + 2 = 6
// 2,3,5 -> 2 + 2 + 2 = 6
// tot =
#[must_use]
pub fn part_one(input: &str) -> Option<u16> {
    let cubes = parse(input);
    let mut open_faces = 0;

    for i in 0..cubes.len() {
        let cube = &cubes[i];

        open_faces += count_open_faces(cube, &cubes);
    }

    Some(open_faces)
}
// 2,2,2 -> 0 + 0 + 0 = 0
// 1,2,2 -> 1 + 2 + 1 = 4
// 3,2,2 -> 1 + 2 + 1 = 4
// 2,1,2 -> 2 + 1 + 1 = 4
// 2,3,2 -> 2 + 1 + 1 = 4
// 2,2,1 -> 2 + 2 + 1 = 5
// 2,2,3 -> 2 + 2 + 0 = 4
// 2,2,4 -> 2 + 2 + 0 = 4
// 2,2,6 ->
// 1,2,5 ->
// 3,2,5 ->
// 2,1,5 ->
// 2,3,5 ->
// tot =
#[must_use]
pub fn part_two(input: &str) -> Option<u16> {
    let _cubes = parse(input);

    None
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: i16,
    y: i16,
    z: i16,
}

fn is_adjacent(a: &Cube, b: &Cube) -> bool {
    (a.x == b.x && a.y == b.y && [b.z - 1, b.z + 1].contains(&a.z))
        || (a.x == b.x && a.z == b.z && [b.y - 1, b.y + 1].contains(&a.y))
        || (a.y == b.y && a.z == b.z && [b.x - 1, b.x + 1].contains(&a.x))
}
/*
fn adjacent_positions(cube: &Cube) -> Vec<Cube> {
    let front = Cube {
        x: cube.x,
        y: cube.y,
        z: cube.z - 1,
    };
    let back = Cube {
        x: cube.x,
        y: cube.y,
        z: cube.z + 1,
    };
    let bottom = Cube {
        x: cube.x,
        y: cube.y - 1,
        z: cube.z,
    };
    let top = Cube {
        x: cube.x,
        y: cube.y + 1,
        z: cube.z,
    };
    let left = Cube {
        x: cube.x - 1,
        y: cube.y,
        z: cube.z,
    };
    let right = Cube {
        x: cube.x + 1,
        y: cube.y,
        z: cube.z,
    };

    vec![front, back, top, bottom, left, right]
}
*/
fn count_open_faces(cube: &Cube, all_cubes: &[Cube]) -> u16 {
    let mut adjacent_count = 0;

    for c in all_cubes {
        if cube == c {
            continue;
        }

        adjacent_count += u16::from(is_adjacent(cube, c));
    }

    6 - adjacent_count
}

fn parse_cube(line: &str) -> Cube {
    let (x_str, rest) = line.split_once(',').unwrap();
    let (y_str, z_str) = rest.split_once(',').unwrap();

    Cube {
        x: x_str.parse::<i16>().unwrap(),
        y: y_str.parse::<i16>().unwrap(),
        z: z_str.parse::<i16>().unwrap(),
    }
}

fn parse(input: &str) -> Vec<Cube> {
    input.lines().map(parse_cube).collect_vec()
}

fn main() {
    //not 3406
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), None);
    }
}
