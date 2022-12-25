#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "25";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> String {
    let mut total = lines.iter().fold(0i64, |acc, l| {
        acc + l.chars().rev().enumerate().fold(0i64, |acc, (i, c)| {
            let digit = match c {
                '2' => 2,
                '1' => 1,
                '0' => 0,
                '-' => -1,
                '=' => -2,
                _ => panic!("unknown"),
            };
            acc + digit * 5i64.pow(i as u32)
        })
    });
    let mut snafu = "".to_string();
    while total > 0 {
        let rem = total.rem_euclid(5);
        total /= 5;
        if rem <= 2 {
            snafu.push_str(format!("{}", rem).as_str());
        } else {
            snafu.push(if rem == 3 { '=' } else { '-' });
            total += 1;
        }
    }
    snafu.chars().rev().join("")
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
