#![feature(test)]
extern crate test;

use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

use itertools::{EitherOrBoth, Itertools};

const DAY: &str = "13";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Item {
    List(Vec<Item>),
    Integer(u64),
}

fn parse_item(line: &String) -> Item {
    if !line.starts_with("[") {
        Item::Integer(line.parse::<u64>().expect("Invalid integer"))
    } else {
        let line = line.strip_prefix("[").unwrap().strip_suffix("]").unwrap();
        let mut current_list = vec![];
        let mut buffer = String::new();
        let mut depth = 0;
        for c in line.chars() {
            match c {
                ',' if depth == 0 => {
                    if !buffer.is_empty() {
                        current_list.push(parse_item(&buffer));
                        buffer.clear();
                    }
                }
                '[' => {
                    depth += 1;
                    buffer.push(c);
                }
                ']' => {
                    depth -= 1;
                    buffer.push(c);
                    if depth == 0 {
                        current_list.push(parse_item(&buffer));
                        buffer.clear();
                    }
                }
                c => buffer.push(c),
            }
        }
        if buffer.len() > 0 {
            current_list.push(parse_item(&buffer));
        }
        Item::List(current_list)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        use Item::*;
        match (self, other) {
            (Integer(a), Integer(b)) => a.cmp(b),
            (List(a), List(b)) => {
                for e in a.iter().zip_longest(b.iter()) {
                    match e {
                        EitherOrBoth::Both(a, b) => {
                            let res = a.cmp(b);
                            if res != Ordering::Equal {
                                return res;
                            }
                        }
                        EitherOrBoth::Right(_) => return Ordering::Less,
                        EitherOrBoth::Left(_) => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            (List(a), Integer(b)) => List(a.clone()).cmp(&List(vec![Integer(b.clone())])),
            (Integer(_), List(_)) => other.cmp(self).reverse(),
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(lines: &Vec<String>) -> u32 {
    let mut pairs = vec![];
    let mut iter = lines.iter();
    loop {
        let a = parse_item(&iter.next().unwrap());
        let b = parse_item(&iter.next().unwrap());
        pairs.push((a, b));
        if iter.next().is_none() {
            break;
        }
    }
    let mut sum = 0;
    for (i, (a, b)) in pairs.iter().enumerate() {
        if a < b {
            sum += i + 1
        }
    }
    sum as u32
}

fn part2(lines: &Vec<String>) -> u32 {
    use Item::*;
    let mut items = vec![];
    let mut iter = lines.iter();
    loop {
        let a = parse_item(&iter.next().unwrap());
        let b = parse_item(&iter.next().unwrap());
        items.push(a);
        items.push(b);
        if iter.next().is_none() {
            break;
        }
    }
    let dividers = [
        List(vec![List(vec![Integer(2)])]),
        List(vec![List(vec![Integer(6)])]),
    ];
    items.extend(dividers.clone());
    items.sort();
    let divider_indexes = dividers.map(|d| items.binary_search(&d).expect("divider not found"));
    divider_indexes.iter().map(|i| i + 1).product::<usize>() as u32
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
