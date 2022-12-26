#![feature(test)]
extern crate test;

use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use itertools::Itertools;

const DAY: &str = "24";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}
#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct State {
    i: i32,
    j: i32,
    t: u32,
}

struct Blizzard {
    initial_position: (u32, u32),
    direction: (i32, i32),
}

fn get_blizzard_occupancy_map(
    blizzards: &Vec<Blizzard>,
    t: u32,
    size: (i32, i32),
) -> Vec<Vec<bool>> {
    let mut out = vec![vec![false; size.1 as usize]; size.0 as usize];
    for bliz in blizzards {
        let (i, j) = bliz.initial_position;
        let (di, dj) = bliz.direction;
        let ni = 1 + (i as i32 + di * t as i32 - 1).rem_euclid(size.0 - 2) as usize;
        let nj = 1 + (j as i32 + dj * t as i32 - 1).rem_euclid(size.1 - 2) as usize;
        out[ni][nj] = true;
    }
    out
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut blizzards = vec![];
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            let dir = match c {
                '^' => (-1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                'v' => (1, 0),
                _ => continue,
            };
            blizzards.push(Blizzard {
                initial_position: (i as u32, j as u32),
                direction: dir,
            });
        }
    }
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let start = State { i: 0, j: 1, t: 0 };
    queue.push_front((start, vec![]));
    explored.insert(start);
    while let Some((state, history)) = queue.pop_back() {
        let mut history = history.clone();
        history.push(state);
        let State { i, j, t } = state;
        if i == height - 1 && j == width - 2 {
            return t;
        }
        let blizzard = get_blizzard_occupancy_map(&blizzards, t + 1, (height, width));
        let next_tick = State { i, j, t: t + 1 };
        let potential_states = [
            next_tick.clone(),
            State {
                i: i + 1,
                ..next_tick
            },
            State {
                i: i - 1,
                ..next_tick
            },
            State {
                j: j + 1,
                ..next_tick
            },
            State {
                j: j - 1,
                ..next_tick
            },
        ];
        for new_state in potential_states {
            if !explored.contains(&new_state)
                && ((new_state.i == 0 && new_state.j == 1)
                    || (new_state.i == height - 1 && new_state.j == width - 2)
                    || (new_state.i > 0
                        && new_state.i < height - 1
                        && new_state.j > 0
                        && new_state.j < width - 1))
                && !blizzard[new_state.i as usize][new_state.j as usize]
            {
                queue.push_front((new_state, history.clone()));
                explored.insert(new_state);
            }
        }
    }
    panic!("Path not found");
}

fn get_arrival_time(
    start: (i32, i32),
    dest: (i32, i32),
    start_t: u32,
    blizzards: &Vec<Blizzard>,
    width: i32,
    height: i32,
) -> u32 {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let start = State {
        i: start.0,
        j: start.1,
        t: start_t,
    };
    queue.push_front((start, vec![]));
    explored.insert(start);
    while let Some((state, history)) = queue.pop_back() {
        let mut history = history.clone();
        history.push(state);
        let State { i, j, t } = state;
        if i == dest.0 && j == dest.1 {
            return t;
        }
        let blizzard = get_blizzard_occupancy_map(&blizzards, t + 1, (height, width));
        let next_tick = State { i, j, t: t + 1 };
        let potential_states = [
            next_tick.clone(),
            State {
                i: i + 1,
                ..next_tick
            },
            State {
                i: i - 1,
                ..next_tick
            },
            State {
                j: j + 1,
                ..next_tick
            },
            State {
                j: j - 1,
                ..next_tick
            },
        ];
        for new_state in potential_states {
            if !explored.contains(&new_state)
                && ((new_state.i == 0 && new_state.j == 1)
                    || (new_state.i == height - 1 && new_state.j == width - 2)
                    || (new_state.i > 0
                        && new_state.i < height - 1
                        && new_state.j > 0
                        && new_state.j < width - 1))
                && !blizzard[new_state.i as usize][new_state.j as usize]
            {
                queue.push_front((new_state, history.clone()));
                explored.insert(new_state);
            }
        }
    }
    panic!("Path not found");
}

fn part2(lines: &Vec<String>) -> u32 {
    let mut blizzards = vec![];
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            let dir = match c {
                '^' => (-1, 0),
                '<' => (0, -1),
                '>' => (0, 1),
                'v' => (1, 0),
                _ => continue,
            };
            blizzards.push(Blizzard {
                initial_position: (i as u32, j as u32),
                direction: dir,
            });
        }
    }
    let height = lines.len() as i32;
    let width = lines[0].len() as i32;
    let t1 = get_arrival_time(
        (0, 1),
        (height - 1, width - 2),
        0,
        &blizzards,
        width,
        height,
    );
    let t2 = get_arrival_time(
        (height - 1, width - 2),
        (0, 1),
        t1,
        &blizzards,
        width,
        height,
    );
    get_arrival_time(
        (0, 1),
        (height - 1, width - 2),
        t2,
        &blizzards,
        width,
        height,
    )
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
