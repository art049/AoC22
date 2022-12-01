use std::{
    cmp::max,
    fs::File,
    io::{BufRead, BufReader},
    num::IntErrorKind,
};

const DAY: &str = "01";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}

fn part1() {
    let lines = get_input();
    let mut max_elf_calories = 0;
    let mut elf_calories = 0;
    for line in lines {
        match line.parse::<u64>() {
            Ok(n) => elf_calories += n,
            Err(e) if *e.kind() == IntErrorKind::Empty => {
                max_elf_calories = max(max_elf_calories, elf_calories);
                elf_calories = 0;
            }
            Err(_) => panic!(),
        }
    }
    println!("Max elf calories: {}", max_elf_calories);
}

fn part2() {
    let lines = get_input();
    let mut current_cals = 0;
    let mut sorted_elf_cals = Vec::new();
    for line in lines {
        if line.is_empty() {
            let (Ok(pos) | Err(pos)) = sorted_elf_cals.binary_search(&current_cals);
            sorted_elf_cals.insert(pos, current_cals);
            current_cals = 0;
        } else {
            let item_calories = line.parse::<u64>().unwrap();
            current_cals += item_calories;
        }
    }
    let top3: Vec<u64> = sorted_elf_cals.into_iter().rev().take(3).collect();
    println!("Top 3 elf calories: {:?}", top3);
    let total: u64 = top3.into_iter().sum();
    println!("Total calories from top3: {}", total);
}

fn main() {
    part1();
    part2();
}
