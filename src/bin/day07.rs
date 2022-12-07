#![feature(test)]
extern crate test;

use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
    rc::Rc,
};

use itertools::Itertools;

const DAY: &str = "07";

fn get_input() -> impl Iterator<Item = String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file).lines().filter_map(|line| line.ok())
}
#[derive(Debug)]

enum Node {
    File(String, usize),
    Directory(String),
}

#[derive(Debug)]
struct Tree {
    node: Node,
    children: Vec<Rc<RefCell<Tree>>>,
}

impl Tree {
    pub fn new(node: Node) -> Self {
        Self {
            node: node,
            children: Vec::new(),
        }
    }

    pub fn insert_child(&mut self, child: Node) -> Rc<RefCell<Tree>> {
        let tree = Self::new(child);
        let r = Rc::new(RefCell::new(tree));
        self.children.push(Rc::clone(&r));
        Rc::clone(&r)
    }

    pub fn find_child(&self, name: &str) -> Option<Rc<RefCell<Tree>>> {
        let r = self.children.iter().find(|tree| {
            let (Node::File(iname, _) | Node::Directory(iname)) = &tree.borrow().node;
            name == iname.as_str()
        });
        r.map(|e| Rc::clone(e))
    }
}

fn get_tree(lines: Vec<String>) -> Rc<RefCell<Tree>> {
    let lines = &mut lines.iter();
    let root = Rc::new(RefCell::new(Tree::new(Node::Directory("/".to_string()))));
    let mut dir_history = Vec::new();
    let mut current_tree = Rc::clone(&root);
    while let Some(cmd) = lines.next() {
        let cmd = &cmd[2..];
        let cmd_parts = cmd.split(" ").collect_vec();
        match cmd_parts[0] {
            "cd" => {
                current_tree = match cmd_parts[1] {
                    ".." => dir_history.pop().expect("No parent dir found"),
                    "/" => {
                        dir_history.clear();
                        Rc::clone(&root)
                    }
                    s => {
                        dir_history.push(Rc::clone(&current_tree));
                        let mut current = current_tree.deref().borrow_mut();
                        match current.find_child(&s.to_string()) {
                            None => current.insert_child(Node::Directory(s.to_string())),
                            Some(node) => node,
                        }
                    }
                };
            }
            "ls" => {
                let cmd_return = lines.take_while_ref(|l| !l.starts_with("$")).collect_vec();
                for item in cmd_return {
                    if !item.starts_with("dir") {
                        let s = item.split(" ").collect_vec();
                        let size: usize = s[0].parse::<usize>().unwrap();
                        let name = s[1].to_string();
                        if current_tree.borrow().find_child(&name).is_none() {
                            current_tree
                                .deref()
                                .borrow_mut()
                                .insert_child(Node::File(name, size));
                        }
                    }
                }
            }
            _ => panic!("Unknown command"),
        }
    }
    root
}

fn part1(lines: Vec<String>) -> u32 {
    let root = get_tree(lines);
    let mut small_dirs = Vec::new();
    fn compute(small_dirs: &mut Vec<u32>, tree: Rc<RefCell<Tree>>) -> usize {
        let tree = tree.borrow();
        match tree.node {
            Node::Directory(_) => {
                let s = tree
                    .children
                    .iter()
                    .map(|t| compute(small_dirs, Rc::clone(t)))
                    .sum();
                if s < 100_000 {
                    small_dirs.push(s as u32);
                }
                s
            }
            Node::File(_, size) => size,
        }
    }
    compute(&mut small_dirs, Rc::clone(&root));
    small_dirs.iter().sum()
}

const DISK_SIZE: usize = 70_000_000;
const REQUIRED_SIZE: usize = 30_000_000;

fn part2(lines: Vec<String>) -> u32 {
    let root = get_tree(lines);
    fn get_size(tree: Rc<RefCell<Tree>>) -> usize {
        let tree = tree.borrow();
        match tree.node {
            Node::Directory(_) => tree.children.iter().map(|t| get_size(Rc::clone(t))).sum(),
            Node::File(_, size) => size,
        }
    }
    let used_size = get_size(Rc::clone(&root));
    let free_space = DISK_SIZE - used_size;
    let size_required = REQUIRED_SIZE - free_space;
    let mut candidates: Vec<u32> = Vec::new();
    fn compute(candidates: &mut Vec<u32>, size_required: usize, tree: Rc<RefCell<Tree>>) -> usize {
        let tree = tree.borrow();
        match tree.node {
            Node::Directory(_) => {
                let s = tree
                    .children
                    .iter()
                    .map(|t| compute(candidates, size_required, Rc::clone(t)))
                    .sum();
                if s >= size_required {
                    candidates.push(s as u32);
                }
                s
            }
            Node::File(_, size) => size,
        }
    }
    compute(&mut candidates, size_required, root);
    *candidates.iter().min().unwrap()
}

fn main() {
    let input = get_input().collect_vec();
    let p1_total = part1(input);
    println!("Part1 total: {}", p1_total);
    let input = get_input().collect_vec();
    let p2_total = part2(input);
    println!("Part2 total: {}", p2_total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part1(black_box(lines.to_vec())));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let lines: Vec<String> = get_input().collect();
        b.iter(|| part2(black_box(lines.to_vec())));
    }
}
