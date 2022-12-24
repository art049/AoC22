#![feature(test)]
extern crate test;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "21";

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
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Scalar(i64),
}

fn get_operation_value(table: &HashMap<String, Operation>, op: &Operation) -> i64 {
    use Operation::*;
    match op {
        Scalar(n) => *n,
        Add(a, b) => {
            let a_op = table.get(a).unwrap();
            let b_op = table.get(b).unwrap();
            get_operation_value(table, a_op) + get_operation_value(table, b_op)
        }
        Sub(a, b) => {
            let a_op = table.get(a).unwrap();
            let b_op = table.get(b).unwrap();
            get_operation_value(table, a_op) - get_operation_value(table, b_op)
        }
        Mul(a, b) => {
            let a_op = table.get(a).unwrap();
            let b_op = table.get(b).unwrap();
            get_operation_value(table, a_op) * get_operation_value(table, b_op)
        }
        Div(a, b) => {
            let a_op = table.get(a).unwrap();
            let b_op = table.get(b).unwrap();
            get_operation_value(table, a_op) / get_operation_value(table, b_op)
        }
    }
}

fn part1(lines: &Vec<String>) -> i64 {
    let mut monkeys: HashMap<String, Operation> = HashMap::new();
    for line in lines {
        let (name, op) = line.split_once(": ").unwrap();
        let operation = if op.contains("+") {
            let (a, b) = op.split_once(" + ").unwrap();
            Operation::Add(a.into(), b.into())
        } else if op.contains("-") {
            let (a, b) = op.split_once(" - ").unwrap();
            Operation::Sub(a.into(), b.into())
        } else if op.contains("*") {
            let (a, b) = op.split_once(" * ").unwrap();
            Operation::Mul(a.into(), b.into())
        } else if op.contains("/") {
            let (a, b) = op.split_once(" / ").unwrap();
            Operation::Div(a.into(), b.into())
        } else {
            Operation::Scalar(op.parse().unwrap())
        };
        monkeys.insert(name.to_string(), operation);
    }
    let root = monkeys
        .get(&"root".to_string())
        .expect("root not found")
        .clone();
    get_operation_value(&monkeys, root)
}

fn populate_values(
    table: &HashMap<String, Operation>,
    values: &mut HashMap<String, Option<i64>>,
    op_name: &String,
) -> Option<i64> {
    if op_name == "humn" {
        values.insert(op_name.clone(), None);
        return None;
    }
    let op = table.get(op_name).unwrap();

    use Operation::*;
    let res = match op {
        Scalar(n) => Some(*n),
        Add(a, b) => {
            if let (Some(va), Some(vb)) = (
                populate_values(table, values, a),
                populate_values(table, values, b),
            ) {
                Some(va + vb)
            } else {
                None
            }
        }
        Sub(a, b) => {
            if let (Some(va), Some(vb)) = (
                populate_values(table, values, a),
                populate_values(table, values, b),
            ) {
                Some(va - vb)
            } else {
                None
            }
        }
        Mul(a, b) => {
            if let (Some(va), Some(vb)) = (
                populate_values(table, values, a),
                populate_values(table, values, b),
            ) {
                Some(va * vb)
            } else {
                None
            }
        }
        Div(a, b) => {
            if let (Some(va), Some(vb)) = (
                populate_values(table, values, a),
                populate_values(table, values, b),
            ) {
                Some(va / vb)
            } else {
                None
            }
        }
    };
    values.insert(op_name.clone(), res);
    res
}

fn explore(
    table: &HashMap<String, Operation>,
    values: &HashMap<String, Option<i64>>,
    op_name: &String,
    expected: i64,
) -> Option<i64> {
    if op_name.as_str() == "humn" {
        return Some(expected);
    }
    let op = table.get(op_name).unwrap();

    use Operation::*;
    match op {
        Scalar(_) => None,
        Add(a, b) | Sub(a, b) | Mul(a, b) | Div(a, b) => {
            let va = values.get(a).unwrap();
            let vb = values.get(b).unwrap();
            if let Some(va) = va {
                let new_expected = match op {
                    Add(_, _) => expected - va,
                    Sub(_, _) => va - expected,
                    Mul(_, _) => expected / va,
                    Div(_, _) => va / expected,
                    _ => panic!("op"),
                };
                explore(table, values, b, new_expected)
            } else if let Some(vb) = vb {
                let new_expected = match op {
                    Add(_, _) => expected - vb,
                    Sub(_, _) => expected + vb,
                    Mul(_, _) => expected / vb,
                    Div(_, _) => expected * vb,
                    _ => panic!("op"),
                };
                explore(table, values, a, new_expected)
            } else {
                panic!("Double unknown branches");
            }
        }
    }
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut monkeys: HashMap<String, Operation> = HashMap::new();
    for line in lines {
        let (name, op) = line.split_once(": ").unwrap();
        let operation = if op.contains("+") {
            let (a, b) = op.split_once(" + ").unwrap();
            Operation::Add(a.into(), b.into())
        } else if op.contains("-") {
            let (a, b) = op.split_once(" - ").unwrap();
            Operation::Sub(a.into(), b.into())
        } else if op.contains("*") {
            let (a, b) = op.split_once(" * ").unwrap();
            Operation::Mul(a.into(), b.into())
        } else if op.contains("/") {
            let (a, b) = op.split_once(" / ").unwrap();
            Operation::Div(a.into(), b.into())
        } else {
            Operation::Scalar(op.parse().unwrap())
        };
        monkeys.insert(name.to_string(), operation);
    }
    let root = monkeys
        .get(&"root".to_string())
        .expect("root not found")
        .clone();

    let mut values: HashMap<String, Option<i64>> = HashMap::new();
    populate_values(&monkeys, &mut values, &"root".to_string());
    let (a, b) = match root {
        Operation::Add(a, b) => (a, b),
        _ => panic!("no"),
    };
    if let Some(va) = values.get(a).unwrap() {
        let expected = va;
        explore(&monkeys, &values, b, *expected).unwrap()
    } else if let Some(vb) = values.get(b).unwrap() {
        let expected = vb;
        explore(&monkeys, &values, a, *expected).unwrap()
    } else {
        0
    }
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
