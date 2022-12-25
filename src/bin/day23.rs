#![feature(test)]
extern crate test;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{iproduct, Itertools, MinMaxResult};

const DAY: &str = "23";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn print_elves(elves: &HashSet<(i32, i32)>) {
    for i in 0..12 {
        println!(
            "{}",
            (0..14)
                .map(|j| if elves.contains(&(i, j)) {
                    "🟥"
                } else {
                    "⬜️"
                })
                .join("")
        );
    }
    println!("");
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut elves = HashSet::new();
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }
    println!("== Initial State ==");
    print_elves(&elves);
    for round in 0..10 {
        let mut moves = HashMap::new();
        let mut duplicate_destinations = HashSet::new();
        for (i, j) in elves.iter() {
            if iproduct!([-1, 0, 1], [-1, 0, 1])
                .filter(|(di, dj)| *di != 0 || *dj != 0)
                .all(|(di, dj)| !elves.contains(&(i + di, j + dj)))
            {
                continue;
            }
            let north = (
                [-1, 0, 1]
                    .iter()
                    .all(|&dj| !elves.contains(&(i - 1, j + dj))),
                (*i - 1, *j),
            );
            let south = (
                [-1, 0, 1]
                    .iter()
                    .all(|&dj| !elves.contains(&(i + 1, j + dj))),
                (*i + 1, *j),
            );
            let west = (
                [-1, 0, 1]
                    .iter()
                    .all(|&di| !elves.contains(&(i + di, j - 1))),
                (*i, *j - 1),
            );
            let east = (
                [-1, 0, 1]
                    .iter()
                    .all(|&di| !elves.contains(&(i + di, j + 1))),
                (*i, *j + 1),
            );
            let pos = [north, south, west, east]
                .iter()
                .cycle()
                .skip(round)
                .take(4)
                .find(|(c, _)| *c)
                .map(|e| e.1);
            if let Some(new_pos) = pos {
                if moves.insert(new_pos, (*i, *j)) != None {
                    duplicate_destinations.insert(new_pos);
                }
            }
        }
        for (dest, base) in moves.clone().into_iter() {
            if duplicate_destinations.contains(&dest) {
                continue;
            }
            elves.remove(&base);
            elves.insert(dest);
        }
        println!("== End of Round {} ==", round + 1);
        print_elves(&elves);
        moves.clear();
        duplicate_destinations.clear();
    }

    match (
        elves.iter().map(|(x, _)| x).minmax(),
        elves.iter().map(|(_, y)| y).minmax(),
    ) {
        (MinMaxResult::MinMax(xmin, xmax), MinMaxResult::MinMax(ymin, ymax)) => {
            (xmax - xmin + 1) * (ymax - ymin + 1) - elves.len() as i32
        }
        _ => panic!("nothing"),
    }
}
fn part2(lines: &Vec<String>) -> usize {
    let mut elves = HashSet::new();
    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }
    let mut first_with_no_moving = None;
    for round in 0.. {
        let mut moves = HashMap::new();
        let mut duplicate_destinations = HashSet::new();
        for (i, j) in elves.iter() {
            if iproduct!([-1, 0, 1], [-1, 0, 1])
                .filter(|(di, dj)| *di != 0 || *dj != 0)
                .all(|(di, dj)| !elves.contains(&(i + di, j + dj)))
            {
                continue;
            }
            let north = (
                [-1, 0, 1]
                    .iter()
                    .all(|&dj| !elves.contains(&(i - 1, j + dj))),
                (*i - 1, *j),
            );
            let south = (
                [-1, 0, 1]
                    .iter()
                    .all(|&dj| !elves.contains(&(i + 1, j + dj))),
                (*i + 1, *j),
            );
            let west = (
                [-1, 0, 1]
                    .iter()
                    .all(|&di| !elves.contains(&(i + di, j - 1))),
                (*i, *j - 1),
            );
            let east = (
                [-1, 0, 1]
                    .iter()
                    .all(|&di| !elves.contains(&(i + di, j + 1))),
                (*i, *j + 1),
            );
            let pos = [north, south, west, east]
                .iter()
                .cycle()
                .skip(round)
                .take(4)
                .find(|(c, _)| *c)
                .map(|e| e.1);
            if let Some(new_pos) = pos {
                if moves.insert(new_pos, (*i, *j)) != None {
                    duplicate_destinations.insert(new_pos);
                }
            }
        }
        let move_it = moves.clone().into_iter().collect_vec();
        if move_it.is_empty() {
            if let Some(n) = first_with_no_moving {
                if round - n >= 4 {
                    return n + 1;
                }
            } else {
                first_with_no_moving = Some(round);
            }
        } else {
            first_with_no_moving = None;
        }
        for (dest, base) in move_it
            .iter()
            .filter(|(d, _)| !duplicate_destinations.contains(d))
        {
            elves.remove(base);
            elves.insert(*dest);
        }
        moves.clear();
        duplicate_destinations.clear();
    }
    panic!("nope");
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
