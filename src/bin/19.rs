use std::cmp;

use regex::Regex;

advent_of_code::solution!(19);

const NUM_MINUTES: i32 = 24;

#[derive(Clone, Debug)]
pub struct Blueprint {
    pub id: i32,
    pub ore_cost: i32,
    pub clay_cost: i32,
    pub obsidian_cost: (i32, i32),
    pub geode_cost: (i32, i32),
}

fn parse_input(contents: &str) -> Vec<Blueprint> {
    let re = Regex::new(concat!(
        r"Blueprint (\d+): Each ore robot costs (\d+) ore.\s+",
        r"Each clay robot costs (\d+) ore.\s+",
        r"Each obsidian robot costs (\d+) ore and (\d+) clay.\s+",
        r"Each geode robot costs (\d+) ore and (\d+) obsidian."
    ))
    .unwrap();
    let mut blueprints: Vec<Blueprint> = vec![];
    contents.lines().for_each(|line| {
        if re.is_match(line) {
            let caps = re.captures(line).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let ore_cost = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let clay_cost = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let obsidian_cost = (
                caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let geode_cost = (
                caps.get(6).unwrap().as_str().parse::<i32>().unwrap(),
                caps.get(7).unwrap().as_str().parse::<i32>().unwrap(),
            );
            let blueprint = Blueprint {
                id,
                ore_cost,
                clay_cost,
                obsidian_cost,
                geode_cost,
            };
            blueprints.push(blueprint);
        }
    });

    blueprints
}

fn time_to(cost: i32, current: i32, producer: i32) -> i32 {
    cmp::max((cost - current + producer - 1) / producer, 0) + 1
}

#[allow(clippy::too_many_arguments)]
fn search(
    geodes: i32,
    time: i32,
    ores: i32,
    clays: i32,
    obsidians: i32,
    ore_robots: i32,
    clay_robots: i32,
    obsidian_robots: i32,
    blueprint: &Blueprint,
    max_x: &mut i32,
    max_ores: &i32,
) {
    let max_geodes = geodes + time * time - time;
    if time <= 0 || max_geodes <= *max_x {
        return;
    }
    *max_x = cmp::max(*max_x, geodes);
    if obsidian_robots > 0 {
        let delta_t = cmp::max(
            time_to(blueprint.geode_cost.0, ores, ore_robots),
            time_to(blueprint.geode_cost.1, obsidians, obsidian_robots),
        );
        search(
            geodes + cmp::max(time - delta_t, 0),
            time - delta_t,
            ores + delta_t * ore_robots - blueprint.geode_cost.0,
            clays + delta_t * clay_robots,
            obsidians + delta_t * obsidian_robots - blueprint.geode_cost.1,
            ore_robots,
            clay_robots,
            obsidian_robots,
            blueprint,
            max_x,
            max_ores,
        );
    }
    if max_geodes <= *max_x {
        return;
    }
    if (clay_robots > 0) && (obsidians + (obsidian_robots * time)) < (blueprint.geode_cost.1 * time)
    {
        let delta_t = cmp::max(
            time_to(blueprint.obsidian_cost.0, ores, ore_robots),
            time_to(blueprint.obsidian_cost.1, clays, clay_robots),
        );
        search(
            geodes,
            time - delta_t,
            ores + delta_t * ore_robots - blueprint.obsidian_cost.0,
            clays + delta_t * clay_robots - blueprint.obsidian_cost.1,
            obsidians + delta_t * obsidian_robots,
            ore_robots,
            clay_robots,
            obsidian_robots + 1,
            blueprint,
            max_x,
            max_ores,
        );
    }
    if max_geodes <= *max_x {
        return;
    }
    if clays + (clay_robots * time) < (blueprint.obsidian_cost.1 * time) {
        let delta_t = time_to(blueprint.clay_cost, ores, ore_robots);
        search(
            geodes,
            time - delta_t,
            ores + delta_t * ore_robots - blueprint.clay_cost,
            clays + delta_t * clay_robots,
            obsidians + delta_t * obsidian_robots,
            ore_robots,
            clay_robots + 1,
            obsidian_robots,
            blueprint,
            max_x,
            max_ores,
        );
    }
    if max_geodes <= *max_x {
        return;
    }
    if (ores + (ore_robots * time)) < (*max_ores * time) {
        let delta_t = time_to(blueprint.ore_cost, ores, ore_robots);
        search(
            geodes,
            time - delta_t,
            ores + delta_t * ore_robots - blueprint.ore_cost,
            clays + delta_t * clay_robots,
            obsidians + delta_t * obsidian_robots,
            ore_robots + 1,
            clay_robots,
            obsidian_robots,
            blueprint,
            max_x,
            max_ores,
        );
    }
}

fn solve(blueprint: &Blueprint, num_minutes: i32) -> i32 {
    let max_ore: i32 = *[
        blueprint.clay_cost,
        blueprint.obsidian_cost.0,
        blueprint.geode_cost.0,
    ]
    .iter()
    .max()
    .unwrap();
    let mut max_x: i32 = 0;
    search(
        0,
        num_minutes,
        0,
        0,
        0,
        1,
        0,
        0,
        blueprint,
        &mut max_x,
        &max_ore,
    );
    max_x
}

pub fn part_one(input: &str) -> Option<u32> {
    let blueprints = parse_input(input);
    let quality_sum: i32 = blueprints
        .iter()
        .map(|blueprint| blueprint.id * solve(blueprint, NUM_MINUTES))
        .sum();

    Some(quality_sum as u32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let blueprints = parse_input(input);
    let geodes: i32 = blueprints
        .iter()
        .take(3)
        .fold(1, |acc, blueprint| acc * solve(blueprint, 32));

    Some(geodes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(3472));
    }
}
