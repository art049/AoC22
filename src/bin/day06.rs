#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "06";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn detect_pattern(pattern_length: usize, payload: &str) -> u32 {
    let mut buffer = Vec::new();
    buffer.resize(pattern_length, 0);

    let mut buffer_i = 0;
    for (index, c) in payload.chars().enumerate() {
        buffer[buffer_i] = c as u8;
        buffer_i = (buffer_i + 1) % pattern_length;
        if index >= pattern_length && buffer.iter().unique().count() == pattern_length {
            return (index + 1) as u32;
        }
    }
    0
}

fn part1(lines: &Vec<String>) -> u32 {
    const PATTERN_LENGTH: usize = 4;
    let line = &lines[0];
    detect_pattern(PATTERN_LENGTH, line.as_str())
}

fn part2(lines: &Vec<String>) -> u32 {
    const PATTERN_LENGTH: usize = 14;
    let line = &lines[0];
    detect_pattern(PATTERN_LENGTH, line)
}

fn main() {
    let input = get_input().collect_vec();
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
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part1(&black_box(lines.to_vec())));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part2(&black_box(lines.to_vec())));
    }
}
