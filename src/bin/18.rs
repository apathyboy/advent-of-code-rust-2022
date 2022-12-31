use std::collections::VecDeque;

use itertools::Itertools;

#[must_use]
pub fn part_one(input: &str) -> Option<u16> {
    let cubes = parse(input);
    let mut open_faces = 0;

    for i in 0..cubes.len() {
        open_faces += count_open_faces(cubes[i], &cubes);
    }

    Some(open_faces)
}

/// # Panics
///
/// Will panic on invalid input
#[must_use]
pub fn part_two(input: &str) -> Option<usize> {
    let cubes = parse(input);

    let (min_corner, max_corner) = get_bounds(&cubes);

    let mut visited: Vec<Cube> = Vec::new();
    let mut open_faces: Vec<Cube> = Vec::new();
    let mut q = VecDeque::new();

    q.push_back(min_corner);

    while !q.is_empty() {
        let c = q.pop_front().map_or_else(|| panic!("Expected item"), |c| c);

        if is_in_bounds(min_corner, max_corner, c) && !cubes.contains(&c) && !visited.contains(&c) {
            visited.push(c);

            open_faces.extend(touches_droplet(&cubes, c));

            q.extend(adjacent_positions(c));
        }
    }

    Some(open_faces.len())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i16,
    y: i16,
    z: i16,
}

fn touches_droplet(droplet: &[Cube], test: Cube) -> Vec<Cube> {
    let adjacents = adjacent_positions(test);
    let mut touching = Vec::new();

    for a in adjacents {
        if droplet.contains(&a) {
            touching.push(a);
        }
    }

    touching
}

const fn is_in_bounds(min: Cube, max: Cube, test: Cube) -> bool {
    test.x >= min.x
        && test.y >= min.y
        && test.z >= min.z
        && test.x <= max.x
        && test.y <= max.y
        && test.z <= max.z
}

fn get_bounds(droplet: &[Cube]) -> (Cube, Cube) {
    let mut min = Cube {
        x: i16::MAX,
        y: i16::MAX,
        z: i16::MAX,
    };
    let mut max = Cube {
        x: i16::MIN,
        y: i16::MIN,
        z: i16::MIN,
    };

    for c in droplet {
        if c.x < min.x {
            min.x = c.x;
        }
        if c.y < min.y {
            min.y = c.y;
        }
        if c.z < min.z {
            min.z = c.z;
        }

        if c.x > max.x {
            max.x = c.x;
        }

        if c.y > max.y {
            max.y = c.y;
        }

        if c.z > max.z {
            max.z = c.z;
        }
    }

    min.x -= 2;
    min.y -= 2;
    min.z -= 2;

    max.x += 2;
    max.y += 2;
    max.z += 2;

    (min, max)
}

fn is_adjacent(a: Cube, b: Cube) -> bool {
    adjacent_positions(a).contains(&b)
}

fn adjacent_positions(cube: Cube) -> Vec<Cube> {
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

fn count_open_faces(cube: Cube, all_cubes: &[Cube]) -> u16 {
    6 - all_cubes
        .iter()
        .fold(0, |i, c| i + u16::from(is_adjacent(cube, *c)))
}

fn parse_cube(line: &str) -> Cube {
    line.split(',')
        .tuple_windows()
        .map(|(x_str, y_str, z_str)| Cube {
            x: x_str
                .parse::<i16>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |m| m),
            y: y_str
                .parse::<i16>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |m| m),
            z: z_str
                .parse::<i16>()
                .map_or_else(|e| panic!("Invalid format: {e:?}"), |m| m),
        })
        .next()
        .map_or_else(|| panic!("Invalid format"), |c| c)
}

fn parse(input: &str) -> Vec<Cube> {
    input.lines().map(parse_cube).collect_vec()
}

fn main() {
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
        assert_eq!(part_two(&input), Some(58));
    }
}
