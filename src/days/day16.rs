use std::{cmp::Reverse, collections::BinaryHeap, usize};

use fxhash::FxHashMap;
use regex::Regex;

use crate::util::BitSet;

#[derive(Clone)]
pub struct Map {
    nodes: FxHashMap<u16, Node>,
    aa_key: u16,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Node {
    key: u16,
    name: String,
    linked_nodes: Vec<u16>,
    flow_rate: i32,
    parsed_neighbors: Vec<Neighbor>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Neighbor {
    key: u16,
    // How much pressure is added over one minute
    gain: i32,
    // Minutes it costs to reach the node
    cost: i32,
}

// Field order matters for hash and comparison performance
#[derive(Eq, PartialEq, Hash)]
struct MemoState {
    minutes: u8,
    visited: BitSet,
    node: u16,
    player_id: bool,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Map {
    let regex =
        Regex::new(r#"Valve (.+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"#).unwrap();
    let name_map: FxHashMap<String, u16> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (regex.captures(line).unwrap()[1].to_string(), i as u16))
        .collect();
    let nodes = input
        .lines()
        .map(|line| {
            let captures = regex.captures(line).unwrap();
            let key = name_map[&captures[1]];
            let node = Node {
                key,
                name: captures[1].to_string(),
                flow_rate: captures[2].parse().unwrap(),
                linked_nodes: captures[3].split(", ").map(|s| name_map[s]).collect(),
                parsed_neighbors: vec![],
            };
            (key, node)
        })
        .collect();
    Map {
        nodes,
        aa_key: name_map[&"AA".to_string()],
    }
}

fn dijkstra(
    nodes: &FxHashMap<u16, Node>,
    source: &Node,
    known_paths: &mut FxHashMap<u16, FxHashMap<u16, i32>>,
) {
    let mut distances = FxHashMap::default();
    let mut heap = BinaryHeap::new();

    for node in nodes.keys() {
        distances.insert(*node, i32::MAX);
    }

    distances.insert(source.key, 0);
    heap.push(Reverse((0, source.key)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if cost > distances[&node] {
            continue;
        }
        for neighbor in &nodes[&node].linked_nodes {
            let next = (cost + 1, *neighbor);
            if next.0 < distances[neighbor] {
                distances.insert(*neighbor, next.0);
                heap.push(Reverse(next));
            }
        }
    }

    known_paths.insert(source.key, distances);
}

fn move_valve(
    map: &Map,
    node: &Node,
    minutes: i32,
    player_id: u8,
    memo: &mut FxHashMap<MemoState, i32>,
    visited: BitSet,
) -> i32 {
    if minutes <= 1 {
        return 0;
    }
    let memo_state = MemoState {
        player_id: player_id == 1,
        node: node.key,
        minutes: minutes as u8,
        visited,
    };
    if let Some(res) = memo.get(&memo_state) {
        return *res;
    }
    let mut neighbors_score = node
        .parsed_neighbors
        .iter()
        .filter(|neigh| neigh.cost < minutes && !visited.contains(neigh.key as usize))
        .map(|neighbor| {
            let new_node = &map.nodes[&neighbor.key];
            let mut visited = visited;
            visited.insert(neighbor.key as usize);
            move_valve(
                map,
                new_node,
                minutes - 1 - neighbor.cost,
                player_id,
                memo,
                visited,
            )
        })
        .max()
        .unwrap_or_default();
    if player_id != 0 {
        let elephant_result = move_valve(map, &map.nodes[&map.aa_key], 26, 0, memo, visited);
        neighbors_score = neighbors_score.max(elephant_result);
    }
    let res = node.flow_rate * minutes + neighbors_score;
    memo.insert(memo_state, res);
    res
}

#[aoc(day16, part1)]
pub fn part1(input: &Map) -> i32 {
    let mut known_paths = FxHashMap::default();
    let aa_key = input.aa_key;
    let mut nodes = input
        .nodes
        .values()
        .filter(|node| node.key == aa_key || node.flow_rate > 0)
        .cloned()
        .collect::<Vec<_>>();
    for node in &nodes {
        dijkstra(&input.nodes, node, &mut known_paths);
    }
    for parent in &mut nodes {
        parent.parsed_neighbors = known_paths[&parent.key]
            .iter()
            .filter_map(|(dest, cost)| {
                let node = &input.nodes[dest];
                (*dest != parent.key && node.flow_rate > 0).then_some(Neighbor {
                    key: *dest,
                    gain: node.flow_rate,
                    cost: *cost,
                })
            })
            .collect();
    }
    let nodes: FxHashMap<_, _> = nodes.into_iter().map(|n| (n.key, n)).collect();
    let aa = &nodes[&aa_key].clone();
    let mut map = input.clone();
    map.nodes = nodes;
    move_valve(
        &map,
        aa,
        30,
        0,
        &mut FxHashMap::with_capacity_and_hasher(2_500_000, Default::default()),
        BitSet::default(),
    )
}

#[aoc(day16, part2)]
pub fn part2(input: &Map) -> i32 {
    let mut known_paths = FxHashMap::default();
    let aa_key = input.aa_key;
    let mut nodes = input
        .nodes
        .values()
        .filter(|node| node.key == aa_key || node.flow_rate > 0)
        .cloned()
        .collect::<Vec<_>>();
    for node in &nodes {
        dijkstra(&input.nodes, node, &mut known_paths);
    }
    for parent in &mut nodes {
        parent.parsed_neighbors = known_paths[&parent.key]
            .iter()
            .filter_map(|(dest, cost)| {
                let node = &input.nodes[dest];
                (*dest != parent.key && node.flow_rate > 0).then_some(Neighbor {
                    key: *dest,
                    gain: node.flow_rate,
                    cost: *cost,
                })
            })
            .collect();
    }
    let nodes: FxHashMap<_, _> = nodes.into_iter().map(|n| (n.key, n)).collect();
    let mut map = input.clone();
    let aa = &nodes[&aa_key].clone();
    map.nodes = nodes;
    move_valve(
        &map,
        aa,
        26,
        1,
        &mut FxHashMap::with_capacity_and_hasher(2_500_000, fxhash::FxBuildHasher::default()),
        BitSet::default(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
        assert_eq!(part1(&parse(input)), 1651);
    }

    #[test]
    fn part2_example() {
        let input = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
        assert_eq!(part2(&parse(input)), 1707);
    }
}
