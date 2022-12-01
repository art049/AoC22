use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "01";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn part1() {
    let lines = get_input();
}

fn part2() {
    let lines = get_input();
}

fn main() {
    part1();
    part2();
}
