use std::{collections::VecDeque, rc::Rc};

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use regex::Regex;

type Node = Rc<str>;

#[derive(Debug)]
pub struct Edge {
    start: Node,
    items: Vec<Node>,
}

#[aoc_generator(day25)]
fn parse(input: &str) -> Vec<Edge> {
    let regex = Regex::new(r#"(\w+)"#).unwrap();
    input
        .lines()
        .map(|l| Edge {
            start: Rc::from(l.split(':').next().unwrap()),
            items: regex
                .captures_iter(l)
                .skip(1)
                .map(|c| Rc::from(&c[1]))
                .collect_vec(),
        })
        .collect_vec()
}

#[aoc(day25, part1)]
pub fn part1(input: &[Edge]) -> usize {
    let mut nodes: FxHashMap<Node, FxHashSet<Node>> = FxHashMap::default();
    for conn in input {
        for dest in &conn.items {
            nodes
                .entry(dest.clone())
                .or_default()
                .insert(conn.start.clone());
            nodes
                .entry(conn.start.clone())
                .or_default()
                .insert(dest.clone());
        }
    }
    let first = nodes.keys().next().unwrap().clone();
    let others = nodes.keys().skip(1).cloned().collect_vec();
    for node in others {
        let mut nodes = nodes.clone();
        for _ in 0..3 {
            let path = shortest_path(&nodes, &first, &node).unwrap();
            for w in path.windows(2) {
                let [a, b] = &w else { panic!() };
                nodes.get_mut(a).unwrap().remove(b);
                nodes.get_mut(b).unwrap().remove(a);
            }
        }

        // Is there still a path if we disconnect the nodes from three paths?
        // If there is, then the paths we removed aren't connecting the two groups.
        // (Credits to u/enderlord113 for the idea)
        if shortest_path(&nodes, &first, &node).is_none() {
            let mut visited = FxHashSet::default();
            let mut to_move = Vec::new();
            to_move.push(&first);
            while let Some(node) = to_move.pop() {
                visited.insert(node);
                for neigh in &nodes[node] {
                    if !visited.contains(neigh) {
                        to_move.push(neigh);
                    }
                }
            }
            return visited.len() * (nodes.len() - visited.len());
        }
    }
    unreachable!()
}

fn shortest_path(
    nodes: &FxHashMap<Node, FxHashSet<Node>>,
    start: &Node,
    end: &Node,
) -> Option<Vec<Node>> {
    let mut visited = FxHashSet::default();
    let mut to_move = VecDeque::new();
    to_move.push_back((start, Vec::new()));
    let mut final_path = None;
    while let Some((node, path)) = to_move.pop_back() {
        if node == end {
            final_path = Some(path);
            break;
        }
        visited.insert(node);
        for neigh in &nodes[node] {
            if !visited.contains(&neigh) {
                let mut path = path.clone();
                path.push(Rc::clone(&node));
                to_move.push_front((&neigh, path));
            }
        }
    }
    if let Some(path) = &mut final_path {
        path.push(Rc::clone(&start));
    }
    final_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
        assert_eq!(part1(&parse(&input)), 54);
    }
}
