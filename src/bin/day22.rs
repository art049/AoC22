#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use int_enum::IntEnum;
use itertools::Itertools;

const DAY: &str = "22";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}
#[derive(Debug)]
enum Step {
    Move(u32),
    RotateCW,
    RotateCCW,
}

enum MapContent {
    Empty,
    Wall,
}

fn get_data(lines: &Vec<String>) -> (Vec<Vec<Option<bool>>>, Vec<Step>) {
    let map_lines = lines.iter().take_while(|l| !l.is_empty()).collect_vec();
    let max_length = map_lines.iter().map(|l| l.len()).max().unwrap();
    let mut occupied = vec![vec![None; max_length]; map_lines.len()];
    for (i, l) in map_lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            occupied[i][j] = match c {
                ' ' => None,
                '#' => Some(true),
                '.' => Some(false),
                _ => panic!("unmatched map character"),
            }
        }
    }
    let path_str = lines.last().unwrap();
    let mut path = vec![];
    let mut buffer = vec![];
    for c in path_str.chars() {
        if c.is_ascii_digit() {
            buffer.push(c);
        } else {
            if let Ok(v) = buffer.iter().join("").parse::<u32>() {
                path.push(Step::Move(v));
                buffer.clear();
            }
            if c == 'R' {
                path.push(Step::RotateCW);
            } else {
                path.push(Step::RotateCCW);
            }
        }
    }
    if let Ok(v) = buffer.iter().join("").parse::<u32>() {
        path.push(Step::Move(v));
        buffer.clear();
    }
    (occupied, path)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntEnum)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn get_new_direction(&self, step: &Step) -> Self {
        match step {
            Step::Move(_) => self.clone(),
            Step::RotateCW => Direction::from_int((self.int_value() + 1) % 4).unwrap(),
            Step::RotateCCW => Direction::from_int((self.int_value() + 3) % 4).unwrap(),
        }
    }
}
#[derive(Debug)]
struct PathSlice {
    start: usize,
    end: usize,
    len: usize,
}

fn part1(lines: &Vec<String>) -> usize {
    let (occupancy, path) = get_data(lines);
    let (width, height) = (occupancy[0].len(), occupancy.len());
    let start = (
        0 as usize,
        occupancy[0]
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some() && !e.unwrap())
            .next()
            .unwrap()
            .0,
    );
    let line_slices = (0..height)
        .map(|e| {
            let mut it = occupancy[e].iter().enumerate().filter(|(_, e)| e.is_some());
            let start = it.next().unwrap().0;
            let end = it.next_back().unwrap().0 + 1;
            PathSlice {
                start,
                end,
                len: end - start,
            }
        })
        .collect_vec();
    let column_slices = (0..width)
        .map(|j| {
            let mut it = (0..height)
                .map(|i| (i, occupancy[i][j]))
                .filter(|(_, e)| e.is_some());
            let start = it.next().unwrap().0;
            let end = it.next_back().unwrap().0 + 1;
            PathSlice {
                start,
                end,
                len: end - start,
            }
        })
        .collect_vec();
    println!("{:#?}", column_slices);

    let mut visited = vec![vec![None; width]; height];
    let mut pos = start.clone();
    let mut direction = Direction::Right;
    visited[pos.0][pos.1] = Some(direction);
    for step in path {
        if let Step::Move(n) = step {
            match direction {
                Direction::Right => {
                    let slice = &line_slices[pos.0];
                    for _ in 0..n {
                        let new_j = if pos.1 + 1 >= slice.end {
                            slice.start
                        } else {
                            pos.1 + 1
                        };
                        if let Some(true) = occupancy[pos.0][new_j] {
                            break;
                        }
                        pos = (pos.0, new_j);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Left => {
                    let slice = &line_slices[pos.0];
                    for _ in 0..n {
                        let new_j = if pos.1 < slice.start + 1 {
                            slice.end - 1
                        } else {
                            pos.1 - 1
                        };
                        if let Some(true) = occupancy[pos.0][new_j] {
                            break;
                        }
                        pos = (pos.0, new_j);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Down => {
                    let slice = &column_slices[pos.1];
                    for _ in 0..n {
                        let new_i = if pos.0 + 1 >= slice.end {
                            slice.start
                        } else {
                            pos.0 + 1
                        };
                        if let Some(true) = occupancy[new_i][pos.1] {
                            break;
                        }
                        pos = (new_i, pos.1);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Up => {
                    let slice = &column_slices[pos.1];
                    for _ in 0..n {
                        let new_i = if pos.0 < slice.start + 1 {
                            slice.end - 1
                        } else {
                            pos.0 - 1
                        };
                        if let Some(true) = occupancy[new_i][pos.1] {
                            break;
                        }
                        pos = (new_i, pos.1);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
            }
        } else {
            direction = direction.get_new_direction(&step);
        }

        println!("{:?} {:?}", pos, direction);
    }
    for i in 0..height {
        for j in 0..width {
            if let Some(dir) = visited[i][j] {
                match dir {
                    Direction::Down => print!("👇"),
                    Direction::Right => print!("👉"),
                    Direction::Up => print!("👆"),
                    Direction::Left => print!("👈"),
                };
            } else if let Some(v) = occupancy[i][j] {
                print!("{}", if v { "⬛️" } else { "⬜" });
            } else {
                print!("🔳");
            }
        }
        println!();
    }
    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction.int_value() as usize
}

fn part2(lines: &Vec<String>) -> u32 {
    0
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
