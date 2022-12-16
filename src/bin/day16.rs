#![feature(test)]
extern crate test;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    u32::MAX,
};

use itertools::{iproduct, Itertools};
use parse_display::{Display, FromStr};
use petgraph::{visit::IntoNodeReferences, Graph};

use petgraph::graph::NodeIndex;

const DAY: &str = "16";

fn get_input() -> Vec<String> {
    let path = format!("inputs/day{}.txt", DAY);
    let file = File::open(path).expect("Could not open file");
    BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect_vec()
}

#[derive(Display, FromStr, Debug, Clone, PartialEq, Eq)]
#[display("Valve {name} has flow rate={rate}")]
struct Valve {
    name: String,
    rate: u32,
}

type ValveGraph = Graph<Valve, ()>;

fn get_graph(lines: &Vec<String>) -> (ValveGraph, NodeIndex) {
    let data = lines
        .iter()
        .map(|l| {
            let (valve, successors) = l.split_once(";").unwrap();
            let valve = valve.parse::<Valve>().unwrap();
            let successors = successors
                .strip_prefix(" tunnels lead to valves ")
                .unwrap_or_else(|| successors.strip_prefix(" tunnel leads to valve ").unwrap())
                .split(", ")
                .collect_vec();
            (valve, successors)
        })
        .collect_vec();
    let mut graph = Graph::new();
    let mut node_map = HashMap::new();
    for (valve, _) in data.iter() {
        let node = graph.add_node(valve.clone());
        node_map.insert(valve.name.clone(), node);
    }
    let start = node_map.get("AA").unwrap();

    for (valve, successors) in data.into_iter() {
        let node = node_map.get(&valve.name).unwrap();
        for successor in successors {
            let successor_node = node_map.get(successor).unwrap();
            graph.add_edge(*node, *successor_node, ());
            graph.add_edge(*successor_node, *node, ());
        }
    }
    (graph, *start)
}

fn floyd_warshall(graph: &ValveGraph) -> Vec<Vec<u32>> {
    let n = graph.node_count();
    let mut dist = vec![vec![MAX; n]; n];
    for edge_index in graph.edge_indices() {
        let (a, b) = graph.edge_endpoints(edge_index).unwrap();
        dist[a.index()][b.index()] = 1;
    }
    for node_index in graph.node_indices() {
        dist[node_index.index()][node_index.index()] = 0;
    }
    for (k, i, j) in iproduct!(
        graph.node_indices(),
        graph.node_indices(),
        graph.node_indices()
    )
    .map(|(i, j, k)| (i.index(), j.index(), k.index()))
    {
        if dist[i][k] != MAX && dist[k][j] != MAX && dist[i][j] > dist[i][k] + dist[k][j] {
            dist[i][j] = dist[i][k] + dist[k][j];
        }
    }
    dist
}

const MAX_TIME: u32 = 30;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Route {
    released: u32,
    path: Vec<usize>,
    path_str: Vec<String>,
}

fn get_released_pressure(
    rates: &Vec<u32>,
    dist: &Vec<Vec<u32>>,
    path: &Vec<usize>,
    time: u32,
) -> Option<u32> {
    let mut released = 0;
    let mut remaining = time;
    for i in 0..path.len() - 1 {
        let dt = dist[path[i]][path[i + 1]] + 1;
        if remaining <= dt {
            return None;
        }
        remaining -= dt;
        released += remaining * rates[path[i + 1]];
    }
    Some(released)
}

fn part1(lines: &Vec<String>) -> u32 {
    let (graph, start) = get_graph(lines);
    let start_valve = &graph[start];
    let rates = graph.node_weights().map(|w| w.rate).collect_vec();
    let dist = floyd_warshall(&graph);

    let mut routes: Vec<Route> = vec![];

    for node in graph.node_indices() {
        let index = node.index();
        let valve = graph.node_weight(node).unwrap();

        let path = vec![start.index(), index];
        let released = get_released_pressure(&rates, &dist, &path, MAX_TIME).unwrap();
        let path_str = vec![start_valve.name.clone(), valve.name.clone()];
        routes.push(Route {
            released,
            path,
            path_str,
        });
    }

    loop {
        let mut max_routes = routes.clone();
        for route_i in 0..routes.len() {
            let route = routes[route_i].clone();
            for (index_to_add, valve_to_add) in graph.node_references() {
                if route.path.contains(&index_to_add.index()) {
                    continue;
                }
                //Middle
                for insert_pos in 1..route.path.len() {
                    let mut path = route.path.clone();
                    path.insert(insert_pos, index_to_add.index());
                    if let Some(new_released) =
                        get_released_pressure(&rates, &dist, &path, MAX_TIME)
                    {
                        if new_released >= max_routes[route_i].released {
                            let mut path_str = route.path_str.clone();
                            path_str.insert(insert_pos, valve_to_add.name.clone());
                            max_routes[route_i] = Route {
                                released: new_released,
                                path,
                                path_str,
                            };
                        }
                    }
                }
                //Last
                let mut path = route.path.clone();
                path.push(index_to_add.index());
                if let Some(new_released) = get_released_pressure(&rates, &dist, &path, MAX_TIME) {
                    if new_released >= max_routes[index_to_add.index()].released {
                        let mut path_str = route.path_str.clone();
                        path_str.push(valve_to_add.name.clone());
                        max_routes[index_to_add.index()] = Route {
                            released: new_released,
                            path,
                            path_str,
                        };
                    }
                }
            }
        }
        let mut should_continue = false;
        for i in 0..routes.len() {
            if max_routes[i].released > routes[i].released {
                routes[i] = max_routes[i].clone();
                should_continue = true;
            }
        }
        if !should_continue {
            break;
        }
    }
    let best = routes.iter().max_by_key(|c| c.released).unwrap();
    println!("Best:{:#?}", best);
    best.released
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct RouteWithElephant {
    released: u32,
    path: Vec<usize>,
    path_str: Vec<String>,
    epath: Vec<usize>,
    epath_str: Vec<String>,
}

const MAX_TIME_ELEPHANT: u32 = 26;

fn part2(lines: &Vec<String>) -> u32 {
    let (graph, start) = get_graph(lines);
    let start_valve = &graph[start];
    let rates = graph.node_weights().map(|w| w.rate).collect_vec();
    let dist = floyd_warshall(&graph);

    let mut routes: Vec<Vec<Option<(Route, Route)>>> = vec![];

    for node in graph.node_indices() {
        let index = node.index();
        let valve = graph.node_weight(node).unwrap();
        let path = vec![start.index(), index];
        let released = get_released_pressure(&rates, &dist, &path, MAX_TIME_ELEPHANT).unwrap();
        let path_str = vec![start_valve.name.clone(), valve.name.clone()];
        let route = Route {
            released,
            path,
            path_str,
        };
        let mut tmp_routes = vec![];
        for enode in graph.node_indices() {
            if node == enode {
                tmp_routes.push(None);
                continue;
            }
            let eindex = enode.index();
            let evalve = graph.node_weight(enode).unwrap();
            let epath = vec![start.index(), eindex];
            let ereleased =
                get_released_pressure(&rates, &dist, &epath, MAX_TIME_ELEPHANT).unwrap();
            let epath_str = vec![start_valve.name.clone(), evalve.name.clone()];
            tmp_routes.push(Some((
                route.clone(),
                Route {
                    released: ereleased,
                    path: epath,
                    path_str: epath_str,
                },
            )));
        }
        routes.push(tmp_routes);
    }

    loop {
        // routes.iter().for_each(|c| println!("{:#?}", c));
        let mut max_routes = routes.clone();
        for route_i in 0..routes.len() {
            for route_j in 0..routes[route_i].len() {
                let d = routes[route_i][route_j].clone();
                if d.is_none() {
                    continue;
                }
                let (route, eroute) = d.unwrap().clone();
                for (index_to_add, valve_to_add) in graph.node_references() {
                    if route.path.contains(&index_to_add.index())
                        || eroute.path.contains(&index_to_add.index())
                    {
                        continue;
                    }
                    //Middle
                    for insert_pos in 1..route.path.len() {
                        let mut path = route.path.clone();
                        path.insert(insert_pos, index_to_add.index());
                        if let Some(new_released) =
                            get_released_pressure(&rates, &dist, &path, MAX_TIME_ELEPHANT)
                        {
                            if let Some((max_route, max_eroute)) =
                                max_routes[route_i][route_j].clone()
                            {
                                if new_released + eroute.released
                                    >= max_route.released + max_eroute.released
                                {
                                    let mut path_str = route.path_str.clone();
                                    path_str.insert(insert_pos, valve_to_add.name.clone());
                                    max_routes[route_i][route_j] = Some((
                                        Route {
                                            released: new_released,
                                            path,
                                            path_str,
                                        },
                                        eroute.clone(),
                                    ));
                                }
                            }
                        }
                    }
                    for insert_pos in 1..eroute.path.len() {
                        let mut epath = eroute.path.clone();
                        epath.insert(insert_pos, index_to_add.index());
                        if let Some(new_released) =
                            get_released_pressure(&rates, &dist, &epath, MAX_TIME_ELEPHANT)
                        {
                            if let Some((max_route, max_eroute)) =
                                max_routes[route_i][route_j].clone()
                            {
                                if route.released + new_released
                                    >= max_route.released + max_eroute.released
                                {
                                    let mut epath_str = eroute.path_str.clone();
                                    epath_str.insert(insert_pos, valve_to_add.name.clone());
                                    max_routes[route_i][route_j] = Some((
                                        route.clone(),
                                        Route {
                                            released: new_released,
                                            path: epath,
                                            path_str: epath_str,
                                        },
                                    ));
                                }
                            }
                        }
                    }
                    //Last
                    let mut path = route.path.clone();
                    path.push(index_to_add.index());
                    if let Some(new_released) =
                        get_released_pressure(&rates, &dist, &path, MAX_TIME_ELEPHANT)
                    {
                        if let Some((max_route, max_eroute)) =
                            max_routes[index_to_add.index()][route_j].clone()
                        {
                            if new_released + eroute.released
                                >= max_route.released + max_eroute.released
                            {
                                let mut path_str = route.path_str.clone();
                                path_str.push(valve_to_add.name.clone());
                                max_routes[index_to_add.index()][route_j] = Some((
                                    Route {
                                        released: new_released,
                                        path,
                                        path_str,
                                    },
                                    eroute.clone(),
                                ));
                            }
                        }
                    }
                    let mut epath = eroute.path.clone();
                    epath.push(index_to_add.index());
                    if let Some(new_released) =
                        get_released_pressure(&rates, &dist, &epath, MAX_TIME_ELEPHANT)
                    {
                        if let Some((max_route, max_eroute)) =
                            max_routes[route_i][index_to_add.index()].clone()
                        {
                            if new_released + route.released
                                >= max_route.released + max_eroute.released
                            {
                                let mut epath_str = eroute.path_str.clone();
                                epath_str.push(valve_to_add.name.clone());
                                max_routes[route_i][index_to_add.index()] = Some((
                                    route.clone(),
                                    Route {
                                        released: new_released,
                                        path: epath,
                                        path_str: epath_str,
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
        let mut should_continue = false;
        for i in 0..routes.len() {
            for j in 0..routes.len() {
                if let (Some(m), Some(r)) = (max_routes[i][j].clone(), routes[i][j].clone()) {
                    if m.0.released + m.1.released > r.0.released + r.1.released {
                        routes[i][j] = max_routes[i][j].clone();
                        should_continue = true;
                    }
                }
            }
        }
        if !should_continue {
            break;
        }
    }
    let best = routes
        .iter()
        .flatten()
        .filter(|e| e.is_some())
        .map(|e| e.clone().unwrap())
        .max_by_key(|(a, b)| a.released + b.released)
        .unwrap();
    println!("best={:#?}", best);
    best.0.released + best.1.released
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
