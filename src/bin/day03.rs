#![feature(test)]
#![feature(iter_next_chunk)]
extern crate test;

use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const DAY: &str = "03";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn get_item_priority(item: char) -> u32 {
    if item.is_uppercase() {
        27 + item as u32 - 'A' as u32
    } else {
        1 + item as u32 - 'a' as u32
    }
}

fn part1(lines: impl Iterator<Item = String>) -> u32 {
    let mut duplicate_totals = 0;
    let mut first_compartment = HashSet::new();
    for line in lines {
        let (left, right) = line.split_at(line.len() / 2);
        for item in left.chars() {
            first_compartment.insert(item);
        }
        for item in right.chars() {
            if first_compartment.contains(&item) {
                duplicate_totals += get_item_priority(item);
                break;
            }
        }
        first_compartment.clear();
    }
    duplicate_totals
}

const GROUP_SIZE: usize = 3;

fn part2(mut lines: impl Iterator<Item = String>) -> u32 {
    let mut total_priorities = 0;
    while let Ok(group) = &lines.next_chunk::<GROUP_SIZE>() {
        let first_items = group.first().expect("No item in the chunk").chars();
        let other_sets: &Vec<HashSet<char>> = &group
            .into_iter()
            .skip(1)
            .map(|s| s.chars().into_iter().collect::<HashSet<char>>())
            .collect();

        let is_item_in_others = |item: &char| other_sets.into_iter().all(|set| set.contains(item));

        let common_item = first_items
            .filter(is_item_in_others)
            .next()
            .expect("No common item found");
        total_priorities += get_item_priority(common_item);
    }
    total_priorities
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

    #[test]
    fn test_item_priority() {
        assert_eq!(get_item_priority('a'), 1);
        assert_eq!(get_item_priority('z'), 26);
        assert_eq!(get_item_priority('A'), 27);
        assert_eq!(get_item_priority('Z'), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(get_input()), 7674);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(get_input()), 2805);
    }

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
