#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "02";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}
#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("Unknown shape"),
        }
    }
}

impl Shape {
    fn base_score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn is_winning_against(&self, other: Shape) -> bool {
        match (self, other) {
            (Shape::Rock, Shape::Scissors) => true,
            (Shape::Paper, Shape::Rock) => true,
            (Shape::Scissors, Shape::Paper) => true,
            _ => false,
        }
    }
    fn get_shape_for_outcome(&self, outcome: RoundOutcome) -> Self {
        match outcome {
            RoundOutcome::Draw => self.clone(),
            RoundOutcome::Win => match self {
                Shape::Scissors => Shape::Rock,
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
            },
            RoundOutcome::Lose => match self {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
        }
    }
}

enum RoundOutcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for RoundOutcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("Unknown shape"),
        }
    }
}

fn get_round_score(theirs: Shape, ours: Shape) -> u64 {
    let round_outcome_score = if theirs == ours {
        3
    } else if ours.is_winning_against(theirs) {
        6
    } else {
        0
    };
    ours.base_score() + round_outcome_score
}

fn part1(lines: impl Iterator<Item = String>) {
    let mut score = 0;
    for line in lines {
        let shapes: Vec<Shape> = line
            .split(" ")
            .into_iter()
            .map(|c| Shape::from(c))
            .collect();
        let (theirs, ours) = (shapes[0], shapes[1]);
        score += get_round_score(theirs, ours);
    }
    println!("Part1 score: {}", score);
}

fn part2(lines: impl Iterator<Item = String>) {
    let mut score = 0;
    for line in lines {
        let chars: Vec<&str> = line.split(" ").collect();
        let (theirs, expected_outcome) = (Shape::from(chars[0]), RoundOutcome::from(chars[1]));
        let ours = theirs.get_shape_for_outcome(expected_outcome);
        score += get_round_score(theirs, ours);
    }
    println!("Part2 score: {}", score);
}

fn main() {
    part1(get_input());
    part2(get_input());
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
