#![feature(test)]
extern crate test;

use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "10";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}
#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn get_instructions_from_lines(lines: &Vec<String>) -> Vec<Instruction> {
    lines
        .iter()
        .map(|s| -> Instruction {
            if s.starts_with("noop") {
                Instruction::Noop
            } else {
                Instruction::AddX(s.split(" ").collect_vec()[1].parse::<i32>().unwrap())
            }
        })
        .collect_vec()
}

fn part1(lines: &Vec<String>) -> i32 {
    let instructions = get_instructions_from_lines(lines);
    let mut execution_buffer = VecDeque::new();
    let mut rx: i32 = 1;
    let mut signal_sum: i32 = 0;
    let mut pc = 0;
    let mut cycle = 1;

    while pc < instructions.len() || !execution_buffer.is_empty() {
        if let Some(instr) = execution_buffer.pop_back() {
            // println!("Cycle {} -> instr={:?} rx={}", cycle, instr, rx);

            if let Some(Instruction::AddX(dx)) = instr {
                rx += dx;
            }
            cycle += 1;
            if cycle >= 20 && cycle <= 220 && (cycle - 20) % 40 == 0 {
                let signal = (cycle as i32) * rx;
                signal_sum += signal;
            }
        } else {
            match instructions[pc] {
                Instruction::Noop => execution_buffer.push_front(None),
                Instruction::AddX(dx) => {
                    execution_buffer.push_front(None);
                    execution_buffer.push_front(Some(Instruction::AddX(dx)));
                }
            }
            pc += 1;
        }
    }
    signal_sum
}
fn part2(lines: &Vec<String>) -> () {
    let instructions = get_instructions_from_lines(lines);
    let mut execution_buffer = VecDeque::new();
    let mut rx: i32 = 1;
    let mut signal_sum: i32 = 0;
    let mut pc = 0;
    let mut cycle = 1;
    let mut crt = [[false; 40]; 6];
    while pc < instructions.len() || !execution_buffer.is_empty() {
        if let Some(instr) = execution_buffer.pop_back() {
            // println!("Cycle {} -> instr={:?} rx={}", cycle, instr, rx);
            let (crt_i, crt_j) = ((cycle - 1) / 40, (cycle - 1) % 40);
            if (crt_j as i32).abs_diff(rx) <= 1 {
                crt[crt_i][crt_j] = true;
            }
            if let Instruction::AddX(dx) = instr {
                rx += dx;
            }
            if cycle >= 20 && cycle <= 220 && (cycle - 20) % 40 == 0 {
                let signal = (cycle as i32) * rx;
                signal_sum += signal;
            }
            cycle += 1;
        } else {
            match instructions[pc] {
                Instruction::Noop => execution_buffer.push_front(Instruction::Noop),
                Instruction::AddX(dx) => {
                    execution_buffer.push_front(Instruction::Noop);
                    execution_buffer.push_front(Instruction::AddX(dx));
                }
            }
            pc += 1;
        }
    }
    println!("Screen:");
    for row in crt {
        println!("{}", row.map(|b| if b { "#" } else { "." }).join(""))
    }
}

fn main() {
    let input = get_input();
    let p1_total = part1(&input);
    println!("Part1 total: {}", p1_total);
    part2(&input);
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
