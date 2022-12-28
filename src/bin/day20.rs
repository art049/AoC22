#![feature(linked_list_remove)]
#![feature(test)]
extern crate test;
use std::{
    collections::LinkedList,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::Itertools;

const DAY: &str = "20";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn apply_transformation(data: LinkedList<i64>) -> LinkedList<i64> {
    let mut data: LinkedList<(usize, i64)> = data.into_iter().enumerate().collect();
    let n = data.len();
    for i in 0..n {
        let index = data
            .iter()
            .find_position(|(j, _)| *j == i)
            .expect("not found")
            .0;
        let mut after = data.split_off(index);
        let elt = after.pop_front().unwrap();
        data.append(&mut after);
        let new_index = ((index as i64 + elt.1 - 1).rem_euclid(n as i64 - 1) + 1) as usize;
        let mut rest = data.split_off(new_index);
        data.push_back(elt);
        data.append(&mut rest);
    }
    data.into_iter().map(|(_i, e)| e).collect()
}

fn part1(lines: &Vec<String>) -> u32 {
    let data: LinkedList<i64> = lines.iter().map(|l| l.parse().unwrap()).collect();
    let data = apply_transformation(data);
    let data_vec = data.iter().collect_vec();
    let zero_index = data
        .iter()
        .find_position(|e| **e == 0)
        .expect("not found")
        .0;
    [1000, 2000, 3000]
        .iter()
        .map(|i| *data_vec[(zero_index + *i) % data_vec.len()])
        .sum::<i64>() as u32
}

fn apply_transformation_step(data: LinkedList<(usize, i64)>) -> LinkedList<(usize, i64)> {
    let mut data = data.clone();
    let n = data.len();
    for i in 0..n {
        let index = data
            .iter()
            .find_position(|(j, _)| *j == i)
            .expect("not found")
            .0;
        let elt = data.remove(index);
        let new_index = ((index as i64 + elt.1 - 1).rem_euclid(n as i64 - 1) + 1) as usize;
        let mut rest = data.split_off(new_index);
        data.push_back(elt);
        data.append(&mut rest);
    }
    data.into_iter().collect()
}

fn part2(lines: &Vec<String>) -> i64 {
    const FACTOR: i64 = 811589153;
    let mut data: LinkedList<(usize, i64)> = lines
        .iter()
        .map(|l| l.parse::<i64>().unwrap() * FACTOR)
        .enumerate()
        .collect();
    for _ in 0..10 {
        data = apply_transformation_step(data);
    }
    let data_vec = data.iter().collect_vec();
    let zero_index = data
        .iter()
        .find_position(|(_i, e)| *e == 0)
        .expect("not found")
        .0;
    [1000, 2000, 3000]
        .iter()
        .map(|i| data_vec[(zero_index + *i) % data_vec.len()].1)
        .sum::<i64>() as i64
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
    fn test_transformation() {
        assert_eq!(
            apply_transformation(LinkedList::from([0, -1])),
            LinkedList::from([-1, 0])
        );
        assert_eq!(
            apply_transformation(LinkedList::from([1, 0])),
            LinkedList::from([0, 1])
        );
        assert_eq!(
            apply_transformation(LinkedList::from([2, 0, 0])),
            LinkedList::from([0, 0, 2])
        );
        assert_eq!(
            apply_transformation(LinkedList::from([-2, 0, 0, 0])),
            LinkedList::from([0, 0, -2, 0])
        );
        assert_eq!(
            apply_transformation(LinkedList::from([3, 0, 0])),
            LinkedList::from([0, 3, 0])
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            apply_transformation(LinkedList::from([1, 2, -3, 3, -2, 0, 4])),
            LinkedList::from([1, 2, -3, 4, 0, 3, -2])
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
