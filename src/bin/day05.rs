#![feature(test)]
extern crate test;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "05";

#[derive(Clone, Debug)]
struct CrateStack {
    data: VecDeque<char>,
}

struct MoveInstruction {
    source: usize,
    destination: usize,
    amount: u64,
}

impl From<&str> for MoveInstruction {
    fn from(s: &str) -> Self {
        let words: Vec<&str> = s.split(" ").collect();
        Self {
            amount: words[1].parse::<u64>().unwrap(),
            source: words[3].parse::<usize>().unwrap() - 1,
            destination: words[5].parse::<usize>().unwrap() - 1,
        }
    }
}

fn get_input() -> (Vec<CrateStack>, Vec<MoveInstruction>) {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec();
    let stack_lines = lines
        .iter()
        .take_while(|line| -> bool { !line.trim().is_empty() })
        .collect_vec();
    let stack_count = stack_lines.last().unwrap().split("   ").collect_vec().len();
    let mut stacks = Vec::new();
    for _ in 0..stack_count {
        stacks.push(CrateStack {
            data: VecDeque::new(),
        });
    }
    for stack_data_line in stack_lines
        .iter()
        .rev()
        .skip(1)
        .map(|s| s.chars().collect_vec())
    {
        for i in 0..stack_count {
            match stack_data_line.get(1 + i * 4) {
                None | Some(' ') => (),
                Some(&c) => stacks[i].data.push_front(c),
            }
        }
    }
    let instruction_lines = lines
        .iter()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .take_while(|line| !line.is_empty());
    let instructions = instruction_lines
        .map(|s| MoveInstruction::from(s.as_str()))
        .collect_vec();
    (stacks, instructions)
}

fn part1((stacks, instructions): (Vec<CrateStack>, Vec<MoveInstruction>)) -> String {
    let mut stacks = stacks.clone();
    for instruction in instructions {
        for _ in 0..instruction.amount {
            if let Some(item) = stacks[instruction.source].data.pop_front() {
                stacks[instruction.destination].data.push_front(item);
            }
        }
    }
    let mut out = String::new();
    for stack in stacks {
        if let Some(item) = stack.data.front() {
            out += item.to_string().as_str();
        }
    }
    out
}
fn part2((stacks, instructions): (Vec<CrateStack>, Vec<MoveInstruction>)) -> String {
    let mut stacks = stacks.clone();
    let mut lifted_stack = VecDeque::new();
    for instruction in instructions {
        lifted_stack.clear();
        for _ in 0..instruction.amount {
            if let Some(item) = stacks[instruction.source].data.pop_front() {
                lifted_stack.push_front(item)
            }
        }
        while let Some(item) = lifted_stack.pop_back() {
            stacks[instruction.destination].data.push_front(item)
        }
    }
    let mut out = String::new();
    for stack in stacks {
        if let Some(item) = stack.data.front() {
            out += item.to_string().as_str();
        }
    }
    out
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
        b.iter(|| part1(black_box(get_input())));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(|| part2(black_box(get_input())));
    }
}
