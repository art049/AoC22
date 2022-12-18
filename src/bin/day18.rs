#![feature(test)]
extern crate test;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "18";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> u32 {
    let positions = lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect_vec();
    let max_x = positions.iter().map(|t| t.0).max().unwrap();
    let max_y = positions.iter().map(|t| t.1).max().unwrap();
    let max_z = positions.iter().map(|t| t.2).max().unwrap();
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z + 1]; max_y + 1]; max_x + 1];
    for (x, y, z) in positions.iter() {
        grid[*x][*y][*z] = true;
    }
    let mut count = 0;
    for (x, y, z) in positions.iter() {
        if *x == 0 || *x > 0 && !grid[x - 1][*y][*z] {
            count += 1;
        }
        if *x == max_x || *x < max_x && !grid[x + 1][*y][*z] {
            count += 1;
        }
        if *y == 0 || *y > 0 && !grid[*x][y - 1][*z] {
            count += 1;
        }
        if *y == max_y || *y < max_y && !grid[*x][y + 1][*z] {
            count += 1;
        }
        if *z == 0 || *z > 0 && !grid[*x][*y][z - 1] {
            count += 1;
        }
        if *z == max_z || *z < max_z && !grid[*x][*y][z + 1] {
            count += 1;
        }
    }
    count
}

fn explore_water(
    grid: &Vec<Vec<Vec<bool>>>,
    pos: (usize, usize, usize),
    max: (usize, usize, usize),
    explored: &mut HashSet<(usize, usize, usize)>,
) -> u32 {
    if grid[pos.0][pos.1][pos.2] {
        panic!("Can't explore a lava cell");
    }
    if !explored.insert(pos) {
        return 0;
    }

    let mut count = 0;
    let (x, y, z) = pos;
    let (max_x, max_y, max_z) = max;
    if x > 0 {
        if grid[x - 1][y][z] {
            count += 1;
        } else {
            count += explore_water(grid, (x - 1, y, z), max, explored)
        }
    }
    if x <= max_x {
        if grid[x + 1][y][z] {
            count += 1;
        } else {
            count += explore_water(grid, (x + 1, y, z), max, explored)
        }
    }
    if y > 0 {
        if grid[x][y - 1][z] {
            count += 1;
        } else {
            count += explore_water(grid, (x, y - 1, z), max, explored)
        }
    }
    if y <= max_y {
        if grid[x][y + 1][z] {
            count += 1;
        } else {
            count += explore_water(grid, (x, y + 1, z), max, explored)
        }
    }
    if z > 0 {
        if z == 0 || grid[x][y][z - 1] {
            count += 1;
        } else {
            count += explore_water(grid, (x, y, z - 1), max, explored)
        }
    }
    if z <= max_z {
        if grid[x][y][z + 1] {
            count += 1;
        } else {
            count += explore_water(grid, (x, y, z + 1), max, explored)
        }
    }
    count
}

fn part2(lines: &Vec<String>) -> u32 {
    let positions = lines
        .iter()
        .map(|l| {
            l.split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        .collect_vec();
    let max_x = positions.iter().map(|t| t.0).max().unwrap();
    let max_y = positions.iter().map(|t| t.1).max().unwrap();
    let max_z = positions.iter().map(|t| t.2).max().unwrap();

    // Add more water around the lava
    let mut grid: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; max_z + 3]; max_y + 3]; max_x + 3];
    for (x, y, z) in positions.iter() {
        grid[*x + 1][*y + 1][*z + 1] = true;
    }
    let max = (max_x + 1, max_y + 1, max_z + 1);
    let mut all_explored: HashSet<(usize, usize, usize)> = HashSet::new();
    explore_water(&grid, (0, 0, 0), max, &mut all_explored)
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
