#![feature(array_zip)]
#![feature(test)]
extern crate test;

use int_enum::IntEnum;
use itertools::{concat, Itertools};
use regex::Regex;
use std::iter;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    vec,
};
use strum::EnumIter;
use strum::IntoEnumIterator;

const DAY: &str = "19";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

#[derive(Debug, IntEnum, Clone, Copy, PartialEq, Eq, EnumIter)]
#[repr(u8)]
enum Resource {
    Ore = 0,
    Clay = 1,
    Obsidian = 2,
    Geode = 3,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Resources([u32; 4]);

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Blueprint([Resources; 4]);

fn get_blueprints(lines: &Vec<String>) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (?P<i>\d+): .+? costs (?P<ore_ore>\d+) .+? costs (?P<clay_ore>\d+) .+? costs (?P<obsidian_ore>\d+) ore and (?P<obsidian_clay>\d+) clay.+? costs (?P<geode_ore>\d+) ore and (?P<geode_obsidian>\d+) obsidian\.").unwrap();
    lines
        .iter()
        .map(|line| {
            let caps = re.captures(line.as_str()).unwrap();
            let ore_ore: u32 = caps.name("ore_ore").unwrap().as_str().parse().unwrap();
            let clay_ore: u32 = caps.name("clay_ore").unwrap().as_str().parse().unwrap();
            let obsidian_ore: u32 = caps.name("obsidian_ore").unwrap().as_str().parse().unwrap();
            let obsidian_clay: u32 = caps
                .name("obsidian_clay")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            let geode_ore: u32 = caps.name("geode_ore").unwrap().as_str().parse().unwrap();
            let geode_obsidian: u32 = caps
                .name("geode_obsidian")
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            Blueprint([
                Resources([ore_ore, 0, 0, 0]),
                Resources([clay_ore, 0, 0, 0]),
                Resources([obsidian_ore, obsidian_clay, 0, 0]),
                Resources([geode_ore, 0, geode_obsidian, 0]),
            ])
        })
        .collect_vec()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct State {
    production: Resources,
    resources: Resources,
    finished_minute: u32,
    geode_produced: u32,
    produced: Vec<(u32, Resource, Resources, [Option<u32>; 4])>,
}

impl State {
    fn advance_time(&self, dt: u32) -> State {
        State {
            resources: Resources(
                self.resources
                    .0
                    .zip(self.production.0)
                    .map(|(r, p)| r + p * dt),
            ),
            finished_minute: self.finished_minute + dt,
            ..self.clone()
        }
    }

    fn get_time_to_produce(&self, blueprint: &Blueprint, kind: Resource) -> Option<u32> {
        let cost = blueprint.0[kind.int_value() as usize];
        if cost
            .0
            .zip(self.production.0)
            .iter()
            .any(|(c, p)| *c > 0 && *p == 0)
        {
            return None;
        }
        Some(
            self.resources
                .0
                .zip(cost.0)
                .zip(self.production.0)
                .iter()
                .filter(|((r, c), p)| *c > *r && *p != 0)
                .map(|((r, c), p)| ((*c - *r) as f32 / *p as f32).ceil() as u32)
                .max()
                .unwrap_or(0),
        )
    }
    fn produce_robot(&self, blueprint: &Blueprint, kind: Resource, max_time: u32) -> Option<State> {
        let cost = blueprint.0[kind.int_value() as usize];
        if self.resources.0.zip(cost.0).iter().all(|(r, c)| *r >= *c) {
            let new_finished_minute = self.finished_minute + 1;
            let mut new_production = self.production.clone();
            let mut geode_produced = self.geode_produced;
            if kind != Resource::Geode {
                new_production.0[kind.int_value() as usize] += 1;
            } else {
                geode_produced += max_time - new_finished_minute;
            }
            let mut new_resources = self.advance_time(1).resources;
            for (i, c) in cost.0.iter().enumerate() {
                new_resources.0[i] -= *c;
            }

            let new_state = State {
                production: new_production,
                resources: new_resources,
                geode_produced,
                produced: self.produced.clone(),
                finished_minute: new_finished_minute,
            };
            let produced = self
                .produced
                .clone()
                .into_iter()
                .chain(
                    [(
                        new_finished_minute,
                        kind,
                        new_resources.clone(),
                        [
                            new_state.get_time_to_produce(blueprint, Resource::Ore),
                            new_state.get_time_to_produce(blueprint, Resource::Clay),
                            new_state.get_time_to_produce(blueprint, Resource::Obsidian),
                            new_state.get_time_to_produce(blueprint, Resource::Geode),
                        ],
                    )]
                    .into_iter(),
                )
                .collect_vec();

            Some(State {
                produced,
                ..new_state
            })
        } else {
            None
        }
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    const MAX_TIME: u32 = 24;
    let blueprints = get_blueprints(lines);
    let mut max_geodes_per_blueprint = vec![];
    let mut max_state_per_blueprint = vec![];
    for blueprint in blueprints.iter() {
        println!("Blueprint={:?}", blueprint);
        let mut stack = vec![];
        stack.push(State {
            resources: Resources([1, 0, 0, 0]),
            geode_produced: 0,
            production: Resources([1, 0, 0, 0]),
            finished_minute: 1,
            produced: vec![(
                0,
                Resource::Ore,
                Resources([0, 0, 0, 0]),
                [None, None, None, None],
            )],
        });
        let mut max_geodes = 0;
        let mut max_state = stack[0].clone();
        while let Some(state) = stack.pop() {
            if state.geode_produced >= max_geodes {
                max_geodes = state.geode_produced;
                max_state = state.clone();
            }
            for kind in Resource::iter().rev() {
                let kind_index = kind.int_value() as usize;
                if (kind != Resource::Geode
                    && state.production.0[kind_index]
                        >= *blueprint.0.map(|c| c.0[kind_index]).iter().max().unwrap())
                {
                    continue;
                }
                if let Some(dt) = state.get_time_to_produce(blueprint, kind) {
                    if let Some(new_state) = state
                        .advance_time(dt)
                        .produce_robot(blueprint, kind, MAX_TIME)
                    {
                        if new_state.finished_minute >= MAX_TIME {
                            continue;
                        }
                        stack.push(new_state);
                    }
                }
            }
        }
        max_geodes_per_blueprint.push(max_geodes);
        max_state_per_blueprint.push(max_state.clone());
        println!("{:#?}", max_state);
    }
    max_geodes_per_blueprint
        .iter()
        .enumerate()
        .map(|(i, e)| (i as u32 + 1) * e)
        .sum()
}

fn part2(lines: &Vec<String>) -> u32 {
    const MAX_TIME: u32 = 32;
    let blueprints = get_blueprints(lines).into_iter().take(3).collect_vec();
    let mut max_geodes_per_blueprint = vec![];
    let mut max_state_per_blueprint = vec![];
    for blueprint in blueprints.iter() {
        println!("Blueprint={:?}", blueprint);
        let mut stack = vec![];
        stack.push(State {
            resources: Resources([1, 0, 0, 0]),
            geode_produced: 0,
            production: Resources([1, 0, 0, 0]),
            finished_minute: 1,
            produced: vec![(
                0,
                Resource::Ore,
                Resources([0, 0, 0, 0]),
                [None, None, None, None],
            )],
        });
        let mut max_geodes = 0;
        let mut max_state = stack[0].clone();
        while let Some(state) = stack.pop() {
            if state.geode_produced >= max_geodes {
                max_geodes = state.geode_produced;
                max_state = state.clone();
            }
            for kind in Resource::iter().rev() {
                let kind_index = kind.int_value() as usize;
                if kind != Resource::Geode
                    && state.production.0[kind_index]
                        >= *blueprint.0.map(|c| c.0[kind_index]).iter().max().unwrap()
                {
                    continue;
                }
                if let Some(dt) = state.get_time_to_produce(blueprint, kind) {
                    if let Some(new_state) = state
                        .advance_time(dt)
                        .produce_robot(blueprint, kind, MAX_TIME)
                    {
                        if new_state.finished_minute > MAX_TIME - 1 {
                            continue;
                        }
                        stack.push(new_state);
                    }
                }
            }
        }
        max_geodes_per_blueprint.push(max_geodes);
        max_state_per_blueprint.push(max_state.clone());
        println!("{:#?}", max_state);
    }
    max_geodes_per_blueprint.iter().product()
}

fn main() {
    let input = get_input();
    let p1_total = part1(&input);
    println!("Part1 total: {}", p1_total);
    let p2_total = part2(&input);
    println!("Part2 total: {}", p2_total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part1(black_box(&lines)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part2(black_box(&lines)));
    }
}
