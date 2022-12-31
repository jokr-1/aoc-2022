use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use regex::Regex;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Resources {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    ore: Resources,
    clay: Resources,
    obsidian: Resources,
    geode: Resources,
}

impl FromStr for Blueprint {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\d+").unwrap();
        let costs: Vec<u16> = re
            .find_iter(s)
            .skip(1)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();

        Ok(Self {
            ore: Resources {
                ore: costs[0],
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay: Resources {
                ore: costs[1],
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian: Resources {
                ore: costs[2],
                clay: costs[3],
                obsidian: 0,
                geode: 0,
            },
            geode: Resources {
                ore: costs[4],
                clay: 0,
                obsidian: costs[5],
                geode: 0,
            },
        })
    }
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn bfs(blueprint: Blueprint, max_time: usize) -> Option<u16> {
    let mut queue = VecDeque::new();

    let mut best_at = vec![0; max_time];
    let max_clay = blueprint.obsidian.clay;
    let max_obsidian = blueprint.geode.obsidian;
    let max_ore = blueprint.clay.ore.max(
        blueprint
            .obsidian
            .ore
            .max(blueprint.obsidian.ore.max(blueprint.geode.ore)),
    ) * 2;

    // minutes, robots, resources
    queue.push_back((0, (1, 0, 0, 0), (0, 0, 0, 0)));
    let mut best: u16 = 0;

    while let Some((minute, (r_ore, r_clay, r_obsidian, r_geode), (ore, clay, obsidian, geode))) =
        queue.pop_front()
    {
        if minute == max_time {
            best = best.max(geode);
            continue;
        }

        if minute > 1 && geode + r_geode < best_at[minute - 1] {
            continue;
        }

        best_at[minute] = best_at[minute].max(geode);

        // geode robot
        if ore >= blueprint.geode.ore && obsidian >= blueprint.geode.obsidian {
            queue.push_back((
                minute + 1,
                (r_ore, r_clay, r_obsidian, r_geode + 1),
                (
                    ore + r_ore - blueprint.geode.ore,
                    clay + r_clay,
                    obsidian + r_obsidian - blueprint.geode.obsidian,
                    geode + r_geode,
                ),
            ))
        } else {
            let max_possible = (max_time - minute) * (max_time - minute + 1) / 2;
            if best_at[minute] < (geode + max_possible as u16) {
                // ore robot
                if ore >= blueprint.ore.ore && r_ore < max_ore && ore <= max_ore {
                    queue.push_back((
                        minute + 1,
                        (r_ore + 1, r_clay, r_obsidian, r_geode),
                        (
                            ore + r_ore - blueprint.ore.ore,
                            clay + r_clay,
                            obsidian + r_obsidian,
                            geode + r_geode,
                        ),
                    ))
                }

                // clay robot
                if ore >= blueprint.clay.ore && r_clay < max_clay && clay <= max_clay {
                    queue.push_back((
                        minute + 1,
                        (r_ore, r_clay + 1, r_obsidian, r_geode),
                        (
                            ore + r_ore - blueprint.clay.ore,
                            clay + r_clay,
                            obsidian + r_obsidian,
                            geode + r_geode,
                        ),
                    ))
                }

                // obsidian robot
                if ore >= blueprint.obsidian.ore
                    && clay >= blueprint.obsidian.clay
                    && r_obsidian < max_obsidian
                    && obsidian < max_obsidian
                {
                    queue.push_back((
                        minute + 1,
                        (r_ore, r_clay, r_obsidian + 1, r_geode),
                        (
                            ore + r_ore - blueprint.obsidian.ore,
                            clay + r_clay - blueprint.obsidian.clay,
                            obsidian + r_obsidian,
                            geode + r_geode,
                        ),
                    ))
                }

                queue.push_back((
                    minute + 1,
                    (r_ore, r_clay, r_obsidian, r_geode),
                    (
                        ore + r_ore,
                        clay + r_clay,
                        obsidian + r_obsidian,
                        geode + r_geode,
                    ),
                ));
            }
            // }
        }
    }
    dbg!(best);
    Some(best)
}

fn part_one(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();

    let x: u32 = blueprints
        .iter()
        .zip(1..)
        .map(|(b, id)| id * bfs(*b, 24).unwrap() as u32)
        .sum();
    x
}

fn part_two(input: &str) -> u32 {
    let blueprints: Vec<Blueprint> = input.lines().map(|l| l.parse().unwrap()).collect();

    let x: u32 = blueprints
        .iter()
        .take(3)
        .map(|b| bfs(*b, 32).unwrap() as u32)
        .product();
    x
}
