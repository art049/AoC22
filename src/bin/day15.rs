#![feature(test)]
extern crate test;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use itertools::Itertools;

const DAY: &str = "15";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug, Clone, Copy, Hash)]
#[display("x={x}, y={y}")]
struct Pos {
    x: i32,
    y: i32,
}
#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Pair {
    sensor: Pos,
    beacon: Pos,
}

fn distance(Pos { x: x1, y: y1 }: Pos, Pos { x: x2, y: y2 }: Pos) -> u32 {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}

fn get_pairs(lines: &Vec<String>) -> Vec<Pair> {
    lines
        .iter()
        .map(|l| l.parse::<Pair>().expect("unable to parse line"))
        .collect_vec()
}

fn get_sorted_intervals_on_line(pairs: &Vec<Pair>, y: i32) -> Vec<(i32, i32)> {
    let mut intervals = vec![];
    for pair in pairs.iter() {
        let closest_beacon_distance = distance(pair.sensor, pair.beacon);
        let target_line_distance = distance(
            pair.sensor,
            Pos {
                x: pair.sensor.x,
                y,
            },
        );
        if closest_beacon_distance < target_line_distance {
            continue;
        }
        let dx = closest_beacon_distance - target_line_distance;
        intervals.push((pair.sensor.x - dx as i32, pair.sensor.x + dx as i32));
    }
    intervals.sort();
    intervals
}

/// `intervals` should be sorted
fn intervals_union_card(intervals: &Vec<(i32, i32)>) -> u32 {
    if intervals.len() == 0 {
        return 0;
    }
    let mut iter = intervals.iter();
    let mut out = 0;
    let (mut min, mut max) = iter.next().unwrap();
    while let Some((l, r)) = iter.next() {
        if *l <= max + 1 {
            if *r > max {
                max = *r;
            }
        } else {
            out += (max - min + 1) as u32;
            min = *l;
            max = *r;
        }
    }
    out += (max - min + 1) as u32;
    out
}

/// `intervals` should be sorted
fn find_first_hole_in_intervals(intervals: &Vec<(i32, i32)>) -> Option<i32> {
    if intervals.len() == 0 {
        return None;
    }
    let mut iter = intervals.iter();
    let (_, mut max) = iter.next().unwrap();
    while let Some((l, r)) = iter.next() {
        if *l <= max + 1 {
            if *r > max {
                max = *r;
            }
        } else {
            return Some(max + 1);
        }
    }
    None
}

fn part1(lines: &Vec<String>) -> u32 {
    const TARGET_Y: i32 = 2000000;
    let pairs = get_pairs(lines);
    let intervals = get_sorted_intervals_on_line(&pairs, TARGET_Y);

    let mut count = intervals_union_card(&intervals);
    for Pair { sensor, .. } in pairs.iter() {
        if sensor.y == TARGET_Y {
            count -= 1
        }
    }
    let beacons_on_line = pairs
        .iter()
        .map(|p| p.beacon)
        .filter(|b| b.y == TARGET_Y)
        .map(|b| b.x)
        .unique()
        .count();
    count -= beacons_on_line as u32;

    count as u32
}

fn part2(lines: &Vec<String>) -> u64 {
    const MAX_Y: i32 = 4000000;
    let pairs = get_pairs(lines);
    for y in 0..=MAX_Y {
        let intervals = get_sorted_intervals_on_line(&pairs, y);
        if let Some(x) = find_first_hole_in_intervals(&intervals) {
            return x as u64 * 4000000 + y as u64;
        }
    }
    panic!("Hole not found");
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

    #[test]
    fn test_parsing() {
        assert_eq!(
            "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
                .parse::<Pair>()
                .unwrap(),
            Pair {
                sensor: Pos { x: 2, y: 18 },
                beacon: Pos { x: -2, y: 15 },
            }
        );
    }

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
