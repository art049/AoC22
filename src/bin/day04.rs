#![feature(test)]
extern crate test;
use itertools::Itertools;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "04";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

struct Assignement {
    start: u64,
    end: u64,
}

impl From<&str> for Assignement {
    fn from(s: &str) -> Self {
        let (start, end) = s
            .split("-")
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Self { start, end }
    }
}

impl Assignement {
    fn size(&self) -> u64 {
        self.end - self.start + 1
    }
    fn fully_contains(&self, other: &Assignement) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    fn overlaps_with(&self, other: &Assignement) -> bool {
        let (first, second) = if self.start <= other.start {
            (self, other)
        } else {
            (other, self)
        };
        second.start <= first.end
    }
}

fn part1(lines: impl Iterator<Item = String>) -> u64 {
    let mut fully_contained_count: u64 = 0;
    for line in lines {
        let (first, second) = line
            .split(",")
            .map(|s| Assignement::from(s))
            .collect_tuple()
            .expect("assignments not found");
        if first.size() > second.size() {
            if first.fully_contains(&second) {
                fully_contained_count += 1;
            }
        } else {
            if second.fully_contains(&first) {
                fully_contained_count += 1;
            }
        }
    }
    fully_contained_count
}

fn part2(lines: impl Iterator<Item = String>) -> u64 {
    let mut overlapping_count: u64 = 0;
    for line in lines {
        let (first, second) = line
            .split(",")
            .map(|s| Assignement::from(s))
            .collect_tuple()
            .expect("assignments not found");
        if first.overlaps_with(&second) {
            overlapping_count += 1;
        }
    }
    overlapping_count
}

fn main() {
    let p1_total = part1(get_input());
    println!("Part1 total: {}", p1_total);
    let p2_total = part2(get_input());
    println!("Part2 total: {}", p2_total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part1(black_box(lines.to_vec().into_iter())));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part2(black_box(lines.to_vec().into_iter())));
    }
}
