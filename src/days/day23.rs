use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use crate::util::bit_set::BitSet;

use super::day17::Direction;

#[derive(Debug, Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Wall,
    Empty,
    SlopeU,
    SlopeR,
    SlopeL,
    SlopeD,
}

#[derive(Debug)]
struct Edge {
    dest: (isize, isize),
    cost: usize,
}

#[derive(Debug, Clone, Copy)]
struct IdEdge {
    dest: usize,
    cost: usize,
}

#[aoc_generator(day23)]
fn parse(input: &str) -> Grid {
    Grid {
        tiles: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        '^' => Tile::SlopeU,
                        'v' => Tile::SlopeD,
                        '<' => Tile::SlopeL,
                        '>' => Tile::SlopeR,
                        c => panic!("unknown {c}"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &Grid) -> usize {
    let start = (
        input.tiles[0]
            .iter()
            .position(|t| *t == Tile::Empty)
            .unwrap() as isize,
        0,
    );
    let end = (
        input
            .tiles
            .last()
            .unwrap()
            .iter()
            .position(|t| *t == Tile::Empty)
            .unwrap() as isize,
        input.tiles.len() as isize - 1,
    );

    let mut goals = FxHashSet::default();
    let mut to_move = VecDeque::new();
    to_move.push_back((start, 0, FxHashSet::default()));

    while let Some(((x, y), steps, mut visited)) = to_move.pop_back() {
        visited.insert((x, y));
        if (x, y) == end {
            goals.insert(steps);
            continue;
        }
        for neigh in input.neighbors((x, y)) {
            if !visited.contains(&neigh) {
                to_move.push_back((neigh, steps + 1, visited.clone()));
            }
        }
    }

    goals.into_iter().max().unwrap()
}

#[aoc(day23, part2)]
pub fn part2(input: &Grid) -> usize {
    let start = (
        input.tiles[0]
            .iter()
            .position(|t| *t == Tile::Empty)
            .unwrap() as isize,
        0,
    );
    let end = (
        input
            .tiles
            .last()
            .unwrap()
            .iter()
            .position(|t| *t == Tile::Empty)
            .unwrap() as isize,
        input.tiles.len() as isize - 1,
    );

    // Start by building an adjacency graph to reduce node density
    let mut to_move = VecDeque::new();
    let mut graph: FxHashMap<(isize, isize), (isize, Vec<Edge>)> = FxHashMap::default();
    to_move.push_back((start, Direction::Down, start, 0));
    let mut visited = FxHashSet::default();

    let mut i = -1;

    while let Some(((x, y), dir, path_start, steps)) = to_move.pop_back() {
        if visited.contains(&(x, y, dir)) {
            continue;
        }
        if (x, y) == end {
            graph
                .entry(path_start)
                .or_insert_with(|| {
                    i += 1;
                    (i, vec![])
                })
                .1
                .push(Edge {
                    dest: end,
                    cost: steps,
                });
            continue;
        }
        visited.insert((x, y, dir));
        let neighs = input.neighbors_2((x, y, dir)).collect_vec();
        if neighs.len() == 1 {
            let (x, y, dir) = neighs[0];
            to_move.push_back(((x, y), dir, path_start, steps + 1));
        } else if !neighs.is_empty() {
            // Found intersection
            graph
                .entry(path_start)
                .or_insert_with(|| {
                    i += 1;
                    (i, vec![])
                })
                .1
                .push(Edge {
                    dest: (x, y),
                    cost: steps,
                });
            for (neigh_x, neigh_y, dir) in neighs {
                to_move.push_back(((neigh_x, neigh_y), dir, (x, y), 1));
            }
        }
    }

    // for bit set
    assert!(graph.len() < 64);
    let start_id = graph[&start].0 as usize;

    // More efficient graph representation, also allows to use a bit set for visited
    let mut id_graph: Vec<Vec<IdEdge>> = vec![vec![]; graph.len()];
    for (id, edges) in graph.values() {
        id_graph[*id as usize] = edges
            .iter()
            .map(|e| IdEdge {
                dest: if e.dest == end {
                    usize::MAX
                } else {
                    graph[&e.dest].0 as usize
                },
                cost: e.cost,
            })
            .collect_vec();
    }

    // Same as part 1, with bit set for visited tracking
    let mut goals = FxHashSet::default();
    let mut to_move = VecDeque::new();
    to_move.push_back((start_id, 0, BitSet::default()));
    while let Some((node, steps, mut visited)) = to_move.pop_back() {
        if node == usize::MAX {
            goals.insert(steps);
            continue;
        }
        visited.insert(node);
        for IdEdge { dest, cost } in &id_graph[node] {
            if *dest == usize::MAX || !visited.contains(*dest) {
                to_move.push_back((*dest, steps + cost, visited.clone()));
            }
        }
    }

    goals.into_iter().max().unwrap()
}

impl Grid {
    pub fn neighbors(&self, cur: (isize, isize)) -> impl Iterator<Item = (isize, isize)> + '_ {
        let tile = self.tiles[cur.1 as usize][cur.0 as usize];
        let dirs: &[Direction] = match tile {
            Tile::SlopeU => &[Direction::Up],
            Tile::SlopeR => &[Direction::Right],
            Tile::SlopeL => &[Direction::Left],
            Tile::SlopeD => &[Direction::Down],
            _ => &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        };
        dirs.into_iter()
            .map(move |d| (cur.0 + d.delta().0, cur.1 + d.delta().1))
            .filter(move |(x, y)| {
                self.tiles
                    .get(*y as usize)
                    .and_then(|r| r.get(*x as usize))
                    .is_some_and(|t| *t != Tile::Wall)
            })
    }

    pub fn neighbors_2(
        &self,
        cur: (isize, isize, Direction),
    ) -> impl Iterator<Item = (isize, isize, Direction)> + '_ {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .filter(move |d| match (cur.2, d) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => false,
            _ => true,
        })
        .map(move |d| (cur.0 + d.delta().0, cur.1 + d.delta().1, d))
        .filter(move |(x, y, _)| {
            self.tiles
                .get(*y as usize)
                .and_then(|r| r.get(*x as usize))
                .is_some_and(|t| *t != Tile::Wall)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
        assert_eq!(part1(&parse(&input)), 94);
    }

    #[test]
    fn part2_example() {
        let input = r#"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"#;
        assert_eq!(part2(&parse(&input)), 154);
    }
}
