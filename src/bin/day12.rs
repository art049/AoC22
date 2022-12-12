#![feature(test)]
extern crate test;

use itertools::{iproduct, Itertools};
use petgraph::{graph::NodeIndex, visit::IntoNodeReferences, Directed, Graph};
use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs::File,
    i32::MAX,
    io::{BufRead, BufReader},
};

const DAY: &str = "12";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

fn get_current_height(c: char) -> i32 {
    match c {
        'S' => get_current_height('a'),
        'E' => get_current_height('z'),
        c if c >= 'a' && c <= 'z' => c as i32 - 'a' as i32,
        _ => panic!("Unknown height character"),
    }
}

type ClimbGraph = Graph<(usize, usize), (), Directed>;

fn get_graph(lines: &Vec<Vec<char>>) -> (ClimbGraph, NodeIndex, NodeIndex, Vec<Vec<NodeIndex>>) {
    let mut graph = ClimbGraph::new();
    let n = lines.len();
    let m = lines[0].len();
    let mut heights = vec![vec![0i32; m]; n];
    let mut nodes = vec![vec![None; m]; n];
    let (mut start, mut end) = (None, None);
    for (i, j) in iproduct!(0..n, 0..m) {
        let node = graph.add_node((i, j));
        if lines[i][j] == 'S' {
            start = Some(node);
        } else if lines[i][j] == 'E' {
            end = Some(node);
        }
        heights[i][j] = get_current_height(lines[i][j]);
        nodes[i][j] = Some(node);
    }
    for (i, j) in iproduct!(0..n, 0..m) {
        if i > 0 && heights[i - 1][j] - heights[i][j] <= 1 {
            graph.add_edge(nodes[i][j].unwrap(), nodes[i - 1][j].unwrap(), ());
        }
        if j > 0 && heights[i][j - 1] - heights[i][j] <= 1 {
            graph.add_edge(nodes[i][j].unwrap(), nodes[i][j - 1].unwrap(), ());
        }
        if i < n - 1 && heights[i + 1][j] - heights[i][j] <= 1 {
            graph.add_edge(nodes[i][j].unwrap(), nodes[i + 1][j].unwrap(), ());
        }
        if j < m - 1 && heights[i][j + 1] - heights[i][j] <= 1 {
            graph.add_edge(nodes[i][j].unwrap(), nodes[i][j + 1].unwrap(), ());
        }
    }
    (
        graph,
        start.expect("start node not found"),
        end.expect("end node not found"),
        nodes
            .iter()
            .map(|l| l.iter().map(|e| e.unwrap()).collect_vec())
            .collect_vec(),
    )
}

struct QueueItem {
    priority: i32,
    node: NodeIndex,
}

impl PartialEq for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl Eq for QueueItem {}

impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // Reversed to have a min heap
        Some(self.cmp(other))
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed to have a min heap
        other.priority.cmp(&self.priority)
    }
}

fn dijkstra(graph: &ClimbGraph, start: &NodeIndex) -> HashMap<NodeIndex, i32> {
    let mut dist = HashMap::<NodeIndex, i32>::new();
    let mut prev = HashMap::<NodeIndex, Option<NodeIndex>>::new();
    let mut priority_queue = BinaryHeap::<QueueItem>::new();
    let mut nodes_in_queue = HashSet::new();

    dist.insert(*start, 0);
    priority_queue.push(QueueItem {
        priority: dist[start],
        node: *start,
    });
    nodes_in_queue.insert(*start);
    for (index, _) in graph.node_references() {
        if index != *start {
            dist.insert(index, MAX);
            prev.insert(index, None);
        }
    }

    while let Some(QueueItem { node: u, .. }) = priority_queue.pop() {
        nodes_in_queue.remove(&u);
        for v in graph.neighbors(u) {
            let alt = dist[&u] + 1;
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, Some(u));
                if !nodes_in_queue.contains(&v) {
                    priority_queue.push(QueueItem {
                        node: v,
                        priority: alt,
                    });
                    nodes_in_queue.insert(v);
                }
            }
        }
    }
    dist
}

fn bfs(graph: &ClimbGraph, start: &NodeIndex) -> HashMap<NodeIndex, i32> {
    let mut dist = HashMap::<NodeIndex, i32>::new();
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    dist.insert(*start, 0);
    queue.push_back(*start);
    while let Some(u) = queue.pop_front() {
        for v in graph.neighbors(u) {
            if explored.insert(v) {
                dist.insert(v, dist.get(&u).unwrap() + 1);
                queue.push_back(v);
            }
        }
    }
    dist
}

fn part1_dijsktra(lines: &Vec<String>) -> u32 {
    let char_lines = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (graph, start, end, _) = get_graph(&char_lines);
    let res = dijkstra(&graph, &start);
    *res.get(&end).expect("path not found") as u32
}

fn part1_bfs(lines: &Vec<String>) -> u32 {
    let char_lines = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let (graph, start, end, _) = get_graph(&char_lines);
    let res = bfs(&graph, &start);
    *res.get(&end).expect("path not found") as u32
}

fn part2_dijsktra(lines: &Vec<String>) -> i32 {
    let char_lines = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let mut potential_starts: Vec<(usize, usize)> = vec![];
    for (i, line) in char_lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'a' {
                potential_starts.push((i, j));
            }
        }
    }
    let (mut graph, _, end, nodes) = get_graph(&char_lines);
    graph.reverse();
    let res = dijkstra(&graph, &end);
    *potential_starts
        .iter()
        .filter_map(|(i, j)| res.get(&nodes[*i][*j]))
        .min()
        .unwrap()
}

fn part2_bfs(lines: &Vec<String>) -> i32 {
    let char_lines = lines.iter().map(|l| l.chars().collect_vec()).collect_vec();
    let mut potential_starts: Vec<(usize, usize)> = vec![];
    for (i, line) in char_lines.iter().enumerate() {
        for (j, char) in line.iter().enumerate() {
            if *char == 'a' {
                potential_starts.push((i, j));
            }
        }
    }
    let (mut graph, _, end, nodes) = get_graph(&char_lines);
    graph.reverse();
    let res = bfs(&graph, &end);
    *potential_starts
        .iter()
        .filter_map(|(i, j)| res.get(&nodes[*i][*j]))
        .min()
        .unwrap()
}

fn main() {
    let input = get_input();
    let p1_total = part1_bfs(&input);
    println!("Part1 total: {}", p1_total);
    let p2_total = part2_bfs(&input);
    println!("Part2 total: {}", p2_total);
}

#[cfg(test)]
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_part1_dijsktra(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part1_dijsktra(black_box(&lines)));
    }

    #[bench]
    fn bench_part1_bfs(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part1_bfs(black_box(&lines)));
    }

    #[bench]
    fn bench_part2_dijsktra(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part2_dijsktra(black_box(&lines)));
    }

    #[bench]
    fn bench_part2_bfs(b: &mut Bencher) {
        let lines: Vec<String> = get_input();
        b.iter(|| part2_bfs(black_box(&lines)));
    }
}
