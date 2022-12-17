#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
};

use itertools::{iproduct, Itertools};

const DAY: &str = "17";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

const WIDTH: usize = 7;

#[derive(Debug)]
enum Move {
    Left,
    Right,
}

fn parse(lines: &Vec<String>) -> Vec<Move> {
    lines
        .first()
        .unwrap()
        .chars()
        .into_iter()
        .map(|c| match c {
            '>' => Move::Right,
            '<' => Move::Left,
            _ => panic!("symbol not found"),
        })
        .collect_vec()
}

fn is_dx_ok(
    chamber: &Vec<[bool; WIDTH]>,
    rock: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
    dx: i32,
) -> bool {
    let rock_h = rock.len();
    let rock_w = rock[0].len();
    // Horizontal boundaries
    if x as i32 + dx < 0 || (x + rock_w - 1) as i32 + dx >= WIDTH as i32 {
        return false;
    }
    let new_x = (x as i32 + dx) as usize;
    // No intersection
    if y >= chamber.len() {
        return true;
    }
    let common_lines = chamber.len() - y;
    for (l, j) in iproduct!(0..std::cmp::min(common_lines, rock_h), 0..rock_w) {
        let r = rock[l][j];
        let c = chamber[y + l][new_x + j];
        if r && c {
            return false;
        }
    }
    true
}

fn is_step_down_ok(
    chamber: &Vec<[bool; WIDTH]>,
    rock: &Vec<Vec<bool>>,
    x: usize,
    y: usize,
) -> bool {
    let rock_h = rock.len();
    let rock_w = rock[0].len();
    // Ground check
    if y == 0 {
        return false;
    }
    let new_y = y - 1;
    // No intersection
    if new_y >= chamber.len() {
        return true;
    }

    let common_lines = chamber.len() - new_y;

    // Elt by elt
    for (l, j) in iproduct!(0..std::cmp::min(common_lines, rock_h), 0..rock_w) {
        let r = rock[l][j];
        let c = chamber[new_y + l][x + j];
        if r && c {
            return false;
        }
    }
    true
}

fn _display_chamber(chamber: &Vec<[bool; WIDTH]>) {
    let iter = chamber.iter().rev();
    for line in iter {
        let content = line.map(|e| if e { "🟨" } else { "⬜" });
        println!("⬛{}⬛", content.join(""));
    }
    println!("{}", "⬛".repeat(WIDTH + 2));
}

fn part1(lines: &Vec<String>) -> u32 {
    let rocks = [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ]
    .map(|r| {
        r.iter()
            .map(|l| l.chars().map(|c| c == '#').collect_vec())
            .rev() // Reverse rocks Y to have a better indexing
            .collect_vec()
    });
    let moves = parse(lines);
    let mut moves_iter = moves.iter().cycle();
    let mut chamber: Vec<[bool; WIDTH]> = vec![];

    for rock_index in 0..2022 {
        let rock = &rocks[rock_index % rocks.len()];
        let (rock_h, rock_w) = (rock.len(), rock[0].len());
        let (mut x, mut y) = (2, 3 + chamber.len()); // x of the left y of the bottom
        loop {
            match moves_iter.next().unwrap() {
                Move::Left if is_dx_ok(&chamber, rock, x, y, -1) => x -= 1,
                Move::Right if is_dx_ok(&chamber, rock, x, y, 1) => x += 1,
                _ => (),
            }

            if is_step_down_ok(&chamber, rock, x, y) {
                y -= 1;
            } else {
                if y + rock_h >= chamber.len() {
                    chamber.extend(vec![[false; WIDTH]; (y + rock_h) - chamber.len()]);
                }
                for (i, j) in iproduct!(0..rock_h, 0..rock_w) {
                    chamber[y + i][x + j] |= rock[i][j];
                }
                break;
            }
        }
    }
    chamber.len() as u32
}

#[derive(Debug)]
struct Pattern {
    start: usize,
    len: usize,
}

fn find_consecutive_pattern<T>(data: &Vec<T>) -> Option<Pattern>
where
    T: Eq,
{
    let n = data.len();
    for start in 0..data.len() {
        let duplicate_start_candidates = data[start..]
            .iter()
            .enumerate()
            .filter(|(i, e)| *i > start && i + (i - start + 1) < n && **e == data[start])
            .map(|(i, _)| i);
        for dup_start_candidate in duplicate_start_candidates {
            let len = dup_start_candidate - start;
            if len == 0 {
                continue;
            }
            if (0..len).all(|i| data[start + i] == data[dup_start_candidate + i]) {
                return Some(Pattern { start, len });
            }
        }
    }
    None
}

fn get_relevant_last_lines(lines: &Vec<[bool; WIDTH]>) -> Vec<[bool; 7]> {
    let mut out = vec![];
    let mut union = [false; WIDTH];
    for line in lines.iter().rev() {
        out.push(line.clone());
        for (uv, lv) in union.iter_mut().zip(line) {
            *uv |= *lv;
        }
        if union.iter().all(|e| *e) {
            break;
        }
    }
    out
}

#[derive(Debug, PartialEq, Eq)]
struct AddedRockState {
    relevant_lines: Vec<[bool; 7]>,
    added_height: usize,
    rock_index: usize,
}

const TARGET: usize = 1000000000000;

fn part2(lines: &Vec<String>) -> usize {
    let rocks = [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ]
    .map(|r| {
        r.iter()
            .map(|l| l.chars().map(|c| c == '#').collect_vec())
            .rev() // Reverse rocks Y to have a better indexing
            .collect_vec()
    });
    let moves = parse(lines);

    let mut moves_iter = moves.iter().cycle();
    let mut added_by_rock = vec![];

    let chamber: RefCell<Vec<[bool; WIDTH]>> = RefCell::new(Vec::new());
    for rock_index in 0.. {
        let rock = &rocks[rock_index % rocks.len()];
        let (rock_h, rock_w) = (rock.len(), rock[0].len());
        let initial_chamber_size = chamber.borrow().len();
        let (mut x, mut y) = (2, 3 + initial_chamber_size); // x of the left y of the bottom
        loop {
            match moves_iter.next().unwrap() {
                Move::Left if is_dx_ok(chamber.borrow().as_ref(), rock, x, y, -1) => x -= 1,
                Move::Right if is_dx_ok(chamber.borrow().as_ref(), rock, x, y, 1) => x += 1,
                _ => (),
            }

            if is_step_down_ok(chamber.borrow().as_ref(), rock, x, y) {
                y -= 1;
            } else {
                if y + rock_h >= initial_chamber_size {
                    chamber
                        .borrow_mut()
                        .extend(vec![[false; WIDTH]; (y + rock_h) - initial_chamber_size]);
                }
                for (i, j) in iproduct!(0..rock_h, 0..rock_w) {
                    chamber.borrow_mut()[y + i][x + j] |= rock[i][j];
                }
                break;
            }
        }
        let chamber_ref = chamber.borrow();
        added_by_rock.push(AddedRockState {
            added_height: chamber_ref.len() - initial_chamber_size,
            relevant_lines: get_relevant_last_lines(chamber_ref.as_ref()),
            rock_index: rock_index % rocks.len(),
        });

        if let Some(pattern) = find_consecutive_pattern(&added_by_rock) {
            println!("Found pattern={:#?}", pattern);
            let added_before: usize = added_by_rock[0..pattern.start]
                .iter()
                .map(|a| a.added_height)
                .sum();
            let pattern_weight: usize = added_by_rock[pattern.start..pattern.start + pattern.len]
                .iter()
                .map(|a| a.added_height)
                .sum();
            let pattern_occurences = (TARGET - pattern.start) / pattern.len;
            let remaining_rocks = (TARGET - pattern.start) % pattern.len;
            let remaining_rocks_weight: usize = added_by_rock
                [pattern.start..pattern.start + remaining_rocks]
                .iter()
                .map(|a| a.added_height)
                .sum();

            return added_before + pattern_occurences * pattern_weight + remaining_rocks_weight;
        }
    }
    panic!("Pattern not found");
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
