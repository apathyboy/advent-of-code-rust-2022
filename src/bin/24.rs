use std::collections::HashSet;

advent_of_code::solution!(24);

pub type Grid = Vec<Vec<char>>;

fn parse_input(contents: &str) -> Grid {
    let mut grid: Grid = vec![];
    contents.lines().for_each(|line| {
        if !line.contains("###") {
            let n = line.len();
            grid.push(line.chars().skip(1).take(n - 2).collect::<Vec<char>>());
        }
    });
    grid
}

fn solve(steps: i32, start_pos: (i32, i32), end_pos: (i32, i32), grid: &Grid) -> Option<i32> {
    let mut s = steps;
    let h = grid.len() as i32;
    let w = grid[0].len() as i32;
    let mut positions: HashSet<(i32, i32)> = HashSet::from([start_pos]);
    loop {
        let mut possibles: HashSet<(i32, i32)> = HashSet::new();
        for (row, col) in positions.into_iter() {
            for (nr, nc) in [
                (row, col),
                (row - 1, col),
                (row + 1, col),
                (row, col - 1),
                (row, col + 1),
            ] {
                if (nr, nc) == end_pos {
                    return Some(s);
                }
                if (nr >= 0)
                    && (nr < h)
                    && (nc >= 0)
                    && (nc < w)
                    && grid[nr as usize][(nc - s).rem_euclid(w) as usize] != '>'
                    && grid[nr as usize][(nc + s).rem_euclid(w) as usize] != '<'
                    && grid[(nr - s).rem_euclid(h) as usize][nc as usize] != 'v'
                    && grid[(nr + s).rem_euclid(h) as usize][nc as usize] != '^'
                {
                    possibles.insert((nr, nc));
                }
            }
        }
        positions = possibles;
        if positions.is_empty() {
            positions.insert(start_pos);
        }
        s += 1;
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = parse_input(input);
    let start_pos = (-1, 0);
    let end_pos = ((grid.len()) as i32, (grid[0].len() - 1) as i32);

    solve(1, start_pos, end_pos, &grid)
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = parse_input(input);
    let start_pos = (-1, 0);
    let end_pos = ((grid.len()) as i32, (grid[0].len() - 1) as i32);
    let part1 = solve(1, start_pos, end_pos, &grid);
    if let Some(n1) = part1 {
        let part2 = solve(n1, end_pos, start_pos, &grid);
        if let Some(n2) = part2 {
            let part3 = solve(n2, start_pos, end_pos, &grid);
            if let Some(n3) = part3 {
                return Some(n3);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(54));
    }
}
