#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    thread, vec,
};

use itertools::Itertools;

const DAY: &str = "14";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn get_rock_path(lines: &Vec<String>) -> Vec<Vec<(usize, usize)>> {
    lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|s| {
                    s.split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect_tuple::<(usize, usize)>()
                        .unwrap()
                })
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy)]
enum Content {
    Source,
    Empty,
    Sand,
    Rock,
}

impl Content {
    fn is_occupied(&self) -> bool {
        match self {
            Self::Empty | Self::Source => false,
            Self::Sand | Self::Rock => true,
        }
    }
}

fn display_grid(grid: &Vec<Vec<Content>>) {
    println!(
        "{}",
        grid.iter()
            .map(|line| {
                line.iter()
                    .map(|content| match content {
                        Content::Empty => "⬜",
                        Content::Rock => "⬛",
                        Content::Sand => "🟨",
                        Content::Source => "🟥",
                    })
                    .collect_vec()
                    .join("")
            })
            .join("\n")
    );
}

fn get_next_sand_pos(
    grid: &Vec<Vec<Content>>,
    max_depth: usize,
    (x, y): (usize, usize),
) -> Option<(usize, usize)> {
    if y >= max_depth {
        return None;
    }

    if !grid[y + 1][x].is_occupied() {
        get_next_sand_pos(grid, max_depth, (x, y + 1))
    } else if !grid[y + 1][x - 1].is_occupied() {
        get_next_sand_pos(grid, max_depth, (x - 1, y + 1))
    } else if !grid[y + 1][x + 1].is_occupied() {
        get_next_sand_pos(grid, max_depth, (x + 1, y + 1))
    } else {
        Some((x, y))
    }
}

const SOURCE_X: usize = 500;
const SIM_WIDTH: usize = 400;
const X_OFFSET: usize = SOURCE_X - SIM_WIDTH / 2;

fn initial_fill(grid: &mut Vec<Vec<Content>>, paths: Vec<Vec<(usize, usize)>>) {
    grid[0][SOURCE_X - X_OFFSET] = Content::Source;
    for path in paths {
        for i in 1..path.len() {
            let ((a_x, a_y), (b_x, b_y)) = (path[i - 1], path[i]);
            if a_x == b_x {
                let (ty, by) = if a_y < b_y { (a_y, b_y) } else { (b_y, a_y) };
                for y in ty..=by {
                    grid[y][a_x - X_OFFSET] = Content::Rock;
                }
            } else {
                let (lx, rx) = if a_x < b_x { (a_x, b_x) } else { (b_x, a_x) };
                for x in lx..=rx {
                    grid[a_y][x - X_OFFSET] = Content::Rock;
                }
            }
        }
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    let rock_paths = get_rock_path(lines);
    let sim_depth = *rock_paths.iter().flatten().map(|(_, y)| y).max().unwrap() as usize;
    let mut grid = vec![vec![Content::Empty; SIM_WIDTH]; sim_depth + 1];
    initial_fill(&mut grid, rock_paths);
    let mut sand_count = 0;
    while let Some((x, y)) = get_next_sand_pos(&grid, sim_depth, (SOURCE_X - X_OFFSET, 0)) {
        grid[y][x] = Content::Sand;
        sand_count += 1;
    }
    sand_count
}

fn part2(lines: &Vec<String>) -> u32 {
    let rock_paths = get_rock_path(lines);
    let floor_depth = *rock_paths.iter().flatten().map(|(_, y)| y).max().unwrap() + 2 as usize;
    let mut grid = vec![vec![Content::Empty; SIM_WIDTH]; floor_depth + 1];
    initial_fill(&mut grid, rock_paths);
    for x in 0..SIM_WIDTH {
        grid[floor_depth][x] = Content::Rock;
    }
    let mut sand_count = 0;
    loop {
        let (x, y) =
            get_next_sand_pos(&grid, floor_depth, (SOURCE_X - X_OFFSET, 0)).expect("sand falled");
        grid[y][x] = Content::Sand;
        sand_count += 1;
        if x == SOURCE_X - X_OFFSET && y == 0 {
            break;
        }
    }
    sand_count
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
