#![feature(const_option)]
#![feature(test)]
extern crate test;

use itertools::Itertools;
use num_bigint::{BigUint, ToBigUint};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "11";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

#[derive(Debug)]
enum Operation {
    Square,
    Multiply(u64),
    Add(u64),
}

#[derive(Debug)]
enum Test {
    DivisibleBy(u64),
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<BigUint>,
    operation: Operation,
    test: Test,
    recipient_if_true: usize,
    recipient_if_false: usize,
}

fn apply_operation(operation: &Operation, item: &BigUint) -> BigUint {
    use Operation::*;
    match operation {
        Multiply(x) => item * x,
        Add(x) => item + x,
        Square => item.pow(2),
    }
}

fn get_test_outcome(test: &Test, item: &BigUint) -> bool {
    match test {
        Test::DivisibleBy(x) => (item % x).eq(&0.to_biguint().unwrap()),
    }
}

fn get_monkeys(lines: &Vec<String>) -> Vec<Monkey> {
    let mut out = vec![];

    for chunk in lines.chunks(7) {
        let mut iter = chunk.iter().map(|l| l.trim());
        iter.next();
        let items = iter
            .next()
            .unwrap()
            .strip_prefix("Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap().to_biguint().unwrap())
            .collect::<VecDeque<_>>();
        let (op, val) = iter
            .next()
            .unwrap()
            .strip_prefix("Operation: new = old ")
            .unwrap()
            .split(" ")
            .collect_tuple()
            .unwrap();
        let operation = match (op, val) {
            ("*", "old") => Operation::Square,
            ("*", _) => Operation::Multiply(val.parse::<u64>().unwrap()),
            ("+", _) => Operation::Add(val.parse::<u64>().unwrap()),
            _ => panic!("Unmatched operator"),
        };
        let test = Test::DivisibleBy(
            iter.next()
                .unwrap()
                .strip_prefix("Test: divisible by ")
                .unwrap()
                .parse::<u64>()
                .unwrap(),
        );
        let recipient_if_true = iter
            .next()
            .unwrap()
            .strip_prefix("If true: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let recipient_if_false = iter
            .next()
            .unwrap()
            .strip_prefix("If false: throw to monkey ")
            .unwrap()
            .parse::<usize>()
            .unwrap();

        out.push(Monkey {
            items,
            operation,
            test,
            recipient_if_true,
            recipient_if_false,
        })
    }
    out
}

fn part1(lines: &Vec<String>) -> u64 {
    let mut monkeys = get_monkeys(lines);
    let mut inspection_count = vec![0u64; monkeys.len()];
    const ROUNDS: u64 = 20;
    let monkey_count = monkeys.len();
    for _ in 0..ROUNDS {
        for i in 0..monkey_count {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                inspection_count[i] += 1;
                let worry_level =
                    apply_operation(&monkeys[i].operation, &item) / &3.to_biguint().unwrap();
                let recipient = if get_test_outcome(&monkeys[i].test, &worry_level) {
                    monkeys[i].recipient_if_true
                } else {
                    monkeys[i].recipient_if_false
                };
                monkeys[recipient].items.push_back(worry_level);
            }
        }
    }
    inspection_count.sort();
    let (first, second) = inspection_count
        .iter()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    first * second
}

fn part2(lines: &Vec<String>) -> u64 {
    let mut monkeys = get_monkeys(lines);
    let mut inspection_count = vec![0u64; monkeys.len()];
    let lcm = monkeys
        .iter()
        .map(|m| match m.test {
            Test::DivisibleBy(x) => x,
        })
        .product::<u64>();
    const ROUNDS: u64 = 10000;
    let monkey_count = monkeys.len();
    for _ in 0..ROUNDS {
        for i in 0..monkey_count {
            while !monkeys[i].items.is_empty() {
                let item = monkeys[i].items.pop_front().unwrap();
                inspection_count[i] += 1;
                let worry_level = apply_operation(&monkeys[i].operation, &item);
                let recipient = if get_test_outcome(&monkeys[i].test, &worry_level) {
                    monkeys[i].recipient_if_true
                } else {
                    monkeys[i].recipient_if_false
                };
                let new_val = worry_level % lcm;
                monkeys[recipient].items.push_back(new_val);
            }
        }
    }
    inspection_count.sort();
    let (first, second) = inspection_count
        .iter()
        .rev()
        .take(2)
        .collect_tuple()
        .unwrap();
    first * second
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
