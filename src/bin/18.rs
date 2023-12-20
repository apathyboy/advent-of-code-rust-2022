use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(18);

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
        min.x = min.x.min(c.x);
        min.y = min.y.min(c.y);
        min.z = min.z.min(c.z);

        max.x = max.x.max(c.x);
        max.y = max.y.max(c.y);
        max.z = max.z.max(c.z);
    }

    min.x -= 1;
    min.y -= 1;
    min.z -= 1;

    max.x += 1;
    max.y += 1;
    max.z += 1;

    (min, max)
}

fn is_adjacent(a: Cube, b: Cube) -> bool {
    adjacent_positions(a).contains(&b)
}

fn adjacent_positions(cube: Cube) -> Vec<Cube> {
    vec![
        Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z - 1,
        },
        Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z + 1,
        },
        Cube {
            x: cube.x,
            y: cube.y - 1,
            z: cube.z,
        },
        Cube {
            x: cube.x,
            y: cube.y + 1,
            z: cube.z,
        },
        Cube {
            x: cube.x - 1,
            y: cube.y,
            z: cube.z,
        },
        Cube {
            x: cube.x + 1,
            y: cube.y,
            z: cube.z,
        },
    ]
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

pub fn part_one(input: &str) -> Option<u16> {
    let cubes = parse(input);
    let mut open_faces = 0;

    for i in 0..cubes.len() {
        open_faces += count_open_faces(cubes[i], &cubes);
    }

    Some(open_faces)
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(58));
    }
}
