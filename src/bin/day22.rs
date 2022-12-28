#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use int_enum::IntEnum;
use itertools::Itertools;

const DAY: &str = "22";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}
#[derive(Debug)]
enum Step {
    Move(u32),
    RotateCW,
    RotateCCW,
}

enum MapContent {
    Empty,
    Wall,
}

fn get_data(lines: &Vec<String>) -> (Vec<Vec<Option<bool>>>, Vec<Step>) {
    let map_lines = lines.iter().take_while(|l| !l.is_empty()).collect_vec();
    let max_length = map_lines.iter().map(|l| l.len()).max().unwrap();
    let mut occupied = vec![vec![None; max_length]; map_lines.len()];
    for (i, l) in map_lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            occupied[i][j] = match c {
                ' ' => None,
                '#' => Some(true),
                '.' => Some(false),
                _ => panic!("unmatched map character"),
            }
        }
    }
    let path_str = lines.last().unwrap();
    let mut path = vec![];
    let mut buffer = vec![];
    for c in path_str.chars() {
        if c.is_ascii_digit() {
            buffer.push(c);
        } else {
            if let Ok(v) = buffer.iter().join("").parse::<u32>() {
                path.push(Step::Move(v));
                buffer.clear();
            }
            if c == 'R' {
                path.push(Step::RotateCW);
            } else {
                path.push(Step::RotateCCW);
            }
        }
    }
    if let Ok(v) = buffer.iter().join("").parse::<u32>() {
        path.push(Step::Move(v));
        buffer.clear();
    }
    (occupied, path)
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, IntEnum)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn get_new_direction(&self, step: &Step) -> Self {
        match step {
            Step::Move(_) => self.clone(),
            Step::RotateCW => Direction::from_int((self.int_value() + 1) % 4).unwrap(),
            Step::RotateCCW => Direction::from_int((self.int_value() + 3) % 4).unwrap(),
        }
    }
}
#[derive(Debug)]
struct PathSlice {
    start: usize,
    end: usize,
    len: usize,
}

fn get_starting_position(occupancy: &Vec<Vec<Option<bool>>>) -> (usize, usize) {
    (
        0 as usize,
        occupancy[0]
            .iter()
            .enumerate()
            .filter(|(_, e)| e.is_some() && !e.unwrap())
            .next()
            .unwrap()
            .0,
    )
}

fn part1(lines: &Vec<String>) -> usize {
    let (occupancy, path) = get_data(lines);
    let (width, height) = (occupancy[0].len(), occupancy.len());
    let start = get_starting_position(&occupancy);
    let line_slices = (0..height)
        .map(|e| {
            let mut it = occupancy[e].iter().enumerate().filter(|(_, e)| e.is_some());
            let start = it.next().unwrap().0;
            let end = it.next_back().unwrap().0 + 1;
            PathSlice {
                start,
                end,
                len: end - start,
            }
        })
        .collect_vec();
    let column_slices = (0..width)
        .map(|j| {
            let mut it = (0..height)
                .map(|i| (i, occupancy[i][j]))
                .filter(|(_, e)| e.is_some());
            let start = it.next().unwrap().0;
            let end = it.next_back().unwrap().0 + 1;
            PathSlice {
                start,
                end,
                len: end - start,
            }
        })
        .collect_vec();

    let mut visited = vec![vec![None; width]; height];
    let mut pos = start.clone();
    let mut direction = Direction::Right;
    visited[pos.0][pos.1] = Some(direction);
    for step in path {
        if let Step::Move(n) = step {
            match direction {
                Direction::Right => {
                    let slice = &line_slices[pos.0];
                    for _ in 0..n {
                        let new_j = if pos.1 + 1 >= slice.end {
                            slice.start
                        } else {
                            pos.1 + 1
                        };
                        if let Some(true) = occupancy[pos.0][new_j] {
                            break;
                        }
                        pos = (pos.0, new_j);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Left => {
                    let slice = &line_slices[pos.0];
                    for _ in 0..n {
                        let new_j = if pos.1 < slice.start + 1 {
                            slice.end - 1
                        } else {
                            pos.1 - 1
                        };
                        if let Some(true) = occupancy[pos.0][new_j] {
                            break;
                        }
                        pos = (pos.0, new_j);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Down => {
                    let slice = &column_slices[pos.1];
                    for _ in 0..n {
                        let new_i = if pos.0 + 1 >= slice.end {
                            slice.start
                        } else {
                            pos.0 + 1
                        };
                        if let Some(true) = occupancy[new_i][pos.1] {
                            break;
                        }
                        pos = (new_i, pos.1);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
                Direction::Up => {
                    let slice = &column_slices[pos.1];
                    for _ in 0..n {
                        let new_i = if pos.0 < slice.start + 1 {
                            slice.end - 1
                        } else {
                            pos.0 - 1
                        };
                        if let Some(true) = occupancy[new_i][pos.1] {
                            break;
                        }
                        pos = (new_i, pos.1);
                        visited[pos.0][pos.1] = Some(direction);
                    }
                }
            }
        } else {
            direction = direction.get_new_direction(&step);
        }

        // println!("{:?} {:?}", pos, direction);
    }
    // for i in 0..height {
    //     for j in 0..width {
    //         if let Some(dir) = visited[i][j] {
    //             match dir {
    //                 Direction::Down => print!("👇"),
    //                 Direction::Right => print!("👉"),
    //                 Direction::Up => print!("👆"),
    //                 Direction::Left => print!("👈"),
    //             };
    //         } else if let Some(v) = occupancy[i][j] {
    //             print!("{}", if v { "⬛️" } else { "⬜" });
    //         } else {
    //             print!("🔳");
    //         }
    //     }
    //     println!();
    // }
    1000 * (pos.0 + 1) + 4 * (pos.1 + 1) + direction.int_value() as usize
}

use Direction::*;
#[derive(Debug)]
struct Link {
    in_dir: Direction,
    in_face: usize,
    inverted: bool,
}

#[derive(Debug, Default)]
struct Face {
    occupancy: Vec<Vec<bool>>,
    to_global_pos: Vec<Vec<(usize, usize)>>,
    links: [Option<Link>; 4],
}

fn get_face_starting_position(occupancy: &Vec<Vec<bool>>) -> (usize, usize) {
    (
        0 as usize,
        occupancy[0]
            .iter()
            .enumerate()
            .filter(|(_, e)| !**e)
            .next()
            .unwrap()
            .0,
    )
}

impl Face {
    fn from_occupancy(occupancy: &Vec<Vec<Option<bool>>>, start: (usize, usize), n: usize) -> Face {
        Face {
            occupancy: occupancy
                .iter()
                .skip(start.0)
                .take(n)
                .map(|l| {
                    l.iter()
                        .skip(start.1)
                        .take(n)
                        .map(|e| e.unwrap())
                        .collect_vec()
                })
                .collect_vec()
                .clone(),
            to_global_pos: (start.0..start.0 + n)
                .map(|i| (start.1..start.1 + n).map(|j| (i, j)).collect_vec())
                .collect_vec(),
            ..Face::default()
        }
    }

    fn add_link(&mut self, out_dir: Direction, in_face: usize, in_dir: Direction, inverted: bool) {
        self.links[out_dir.int_value() as usize] = Some(Link {
            in_dir,
            in_face,
            inverted,
        })
    }

    fn pass_through(&self, dir: Direction, index: usize) -> ((usize, usize), usize, Direction) {
        let n = self.occupancy.len();
        let link = self.links[dir.int_value() as usize].as_ref().unwrap();
        let new_index = if link.inverted { n - index - 1 } else { index };
        let face_pos = match link.in_dir {
            Up => (n - 1, new_index),
            Right => (new_index, 0),
            Down => (0, new_index),
            Left => (new_index, n - 1),
        };
        (face_pos, link.in_face, link.in_dir)
    }
}

fn part2(lines: &Vec<String>) -> usize {
    let (occupancy, path) = get_data(lines);
    let (width, height) = (occupancy[0].len(), occupancy.len());
    let n = occupancy.len() / 4;
    // let n = occupancy.len() / 3;
    // Example
    // let mut faces = [
    //     Face::from_occupancy(&occupancy, (0, 2 * n), n),
    //     Face::from_occupancy(&occupancy, (n, 0), n),
    //     Face::from_occupancy(&occupancy, (n, n), n),
    //     Face::from_occupancy(&occupancy, (n, 2 * n), n),
    //     Face::from_occupancy(&occupancy, (2 * n, 2 * n), n),
    //     Face::from_occupancy(&occupancy, (2 * n, 3 * n), n),
    // ];
    // {
    //     faces[0].add_link(Up, 1, Down, true);
    //     faces[0].add_link(Right, 5, Left, true);
    //     faces[0].add_link(Down, 3, Down, false);
    //     faces[0].add_link(Left, 2, Down, false);

    //     faces[1].add_link(Up, 0, Down, true);
    //     faces[1].add_link(Right, 2, Right, false);
    //     faces[1].add_link(Down, 4, Up, true);
    //     faces[1].add_link(Left, 5, Up, true);

    //     faces[2].add_link(Up, 0, Right, false);
    //     faces[2].add_link(Right, 3, Right, false);
    //     faces[2].add_link(Down, 4, Right, true);
    //     faces[2].add_link(Left, 1, Left, false);

    //     faces[3].add_link(Up, 0, Up, false);
    //     faces[3].add_link(Right, 5, Down, true);
    //     faces[3].add_link(Down, 4, Down, false);
    //     faces[3].add_link(Left, 2, Left, false);

    //     faces[4].add_link(Up, 3, Up, false);
    //     faces[4].add_link(Right, 5, Right, false);
    //     faces[4].add_link(Down, 1, Up, true);
    //     faces[4].add_link(Left, 2, Up, true);

    //     faces[5].add_link(Up, 3, Left, true);
    //     faces[5].add_link(Right, 0, Left, true);
    //     faces[5].add_link(Down, 1, Right, true);
    //     faces[5].add_link(Left, 4, Left, false);
    // }

    let mut faces = [
        Face::from_occupancy(&occupancy, (0, n), n),
        Face::from_occupancy(&occupancy, (0, 2 * n), n),
        Face::from_occupancy(&occupancy, (n, n), n),
        Face::from_occupancy(&occupancy, (2 * n, 0), n),
        Face::from_occupancy(&occupancy, (2 * n, n), n),
        Face::from_occupancy(&occupancy, (3 * n, 0), n),
    ];
    {
        faces[0].add_link(Up, 5, Right, false);
        faces[0].add_link(Right, 1, Right, false);
        faces[0].add_link(Down, 2, Down, false);
        faces[0].add_link(Left, 3, Right, true);

        faces[1].add_link(Up, 5, Up, false);
        faces[1].add_link(Right, 4, Left, true);
        faces[1].add_link(Down, 2, Left, false);
        faces[1].add_link(Left, 0, Left, false);

        faces[2].add_link(Up, 0, Up, false);
        faces[2].add_link(Right, 1, Up, false);
        faces[2].add_link(Down, 4, Down, false);
        faces[2].add_link(Left, 3, Down, false);

        faces[3].add_link(Up, 2, Right, false);
        faces[3].add_link(Right, 4, Right, false);
        faces[3].add_link(Down, 5, Down, false);
        faces[3].add_link(Left, 0, Right, true);

        faces[4].add_link(Up, 2, Up, false);
        faces[4].add_link(Right, 1, Left, true);
        faces[4].add_link(Down, 5, Left, false);
        faces[4].add_link(Left, 3, Left, false);

        faces[5].add_link(Up, 3, Up, false);
        faces[5].add_link(Right, 4, Up, false);
        faces[5].add_link(Down, 1, Down, false);
        faces[5].add_link(Left, 0, Down, false);
    }
    let mut direction = Direction::Right;
    let mut face_index = 0usize;
    let mut pos = get_face_starting_position(&faces[face_index].occupancy);

    let mut visited = vec![vec![None; width]; height];
    let global_pos = faces[face_index].to_global_pos[pos.0][pos.1];
    visited[global_pos.0][global_pos.1] = Some(direction);
    for step in path {
        match step {
            Step::Move(dist) => {
                for _ in 0..dist {
                    let mut next_pos = pos.clone();
                    let mut next_face_index = face_index;
                    let mut next_direction = direction.clone();
                    match direction {
                        Up => {
                            if pos.0 == 0 {
                                (next_pos, next_face_index, next_direction) =
                                    faces[face_index].pass_through(Up, pos.1)
                            } else {
                                next_pos.0 = pos.0 - 1;
                            }
                        }
                        Right => {
                            if pos.1 == n - 1 {
                                (next_pos, next_face_index, next_direction) =
                                    faces[face_index].pass_through(Right, pos.0)
                            } else {
                                next_pos.1 = pos.1 + 1;
                            }
                        }
                        Down => {
                            if pos.0 == n - 1 {
                                (next_pos, next_face_index, next_direction) =
                                    faces[face_index].pass_through(Down, pos.1)
                            } else {
                                next_pos.0 = pos.0 + 1;
                            }
                        }
                        Left => {
                            if pos.1 == 0 {
                                (next_pos, next_face_index, next_direction) =
                                    faces[face_index].pass_through(Left, pos.0)
                            } else {
                                next_pos.1 = pos.1 - 1;
                            }
                        }
                    }
                    if faces[next_face_index].occupancy[next_pos.0][next_pos.1] {
                        break;
                    }
                    pos = next_pos;
                    face_index = next_face_index;
                    direction = next_direction;
                    let global_pos = faces[face_index].to_global_pos[pos.0][pos.1];
                    visited[global_pos.0][global_pos.1] = Some(direction);
                }
            }
            Step::RotateCW => {
                direction = Direction::from_int((direction.int_value() + 1) % 4).unwrap()
            }
            Step::RotateCCW => {
                direction = Direction::from_int((direction.int_value() + 3) % 4).unwrap()
            }
        }
    }
    // for i in 0..height {
    //     for j in 0..width {
    //         if let Some(dir) = visited[i][j] {
    //             match dir {
    //                 Direction::Down => print!("👇"),
    //                 Direction::Right => print!("👉"),
    //                 Direction::Up => print!("👆"),
    //                 Direction::Left => print!("👈"),
    //             };
    //         } else if let Some(v) = occupancy[i][j] {
    //             print!("{}", if v { "⬛️" } else { "⬜" });
    //         } else {
    //             print!("🔳");
    //         }
    //     }
    //     println!();
    // }
    let final_pos = faces[face_index].to_global_pos[pos.0][pos.1];
    1000 * (final_pos.0 + 1) + 4 * (final_pos.1 + 1) + direction.int_value() as usize
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
