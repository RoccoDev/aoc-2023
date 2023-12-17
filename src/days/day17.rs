use std::{cmp::Reverse, collections::BinaryHeap};

use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug)]
pub struct Grid(Vec<Vec<u32>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    coords: (isize, isize),
    cost: u32,
    direction: Direction,
    steps: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[aoc_generator(day17)]
fn parse(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
            .collect_vec(),
    )
}

#[aoc(day17, part1)]
pub fn part1(input: &Grid) -> u32 {
    let start = Node {
        coords: (0, 0),
        cost: 0,
        direction: Direction::Up,
        steps: 0,
    };
    dijkstra(
        input,
        start,
        (
            (input.0[0].len() - 1) as isize,
            (input.0.len() - 1) as isize,
        ),
        0,
        3,
    )
}

#[aoc(day17, part2)]
pub fn part2(input: &Grid) -> u32 {
    let start = Node {
        coords: (0, 0),
        cost: 0,
        direction: Direction::Up,
        steps: 0,
    };
    dijkstra(
        input,
        start,
        (
            (input.0[0].len() - 1) as isize,
            (input.0.len() - 1) as isize,
        ),
        4,
        10,
    )
}

fn dijkstra(grid: &Grid, source: Node, end: (isize, isize), min_steps: u8, max_steps: u8) -> u32 {
    let mut distances = FxHashMap::default();
    let mut heap = BinaryHeap::new();

    distances.insert(source, 0);
    heap.push(Reverse((0, source)));

    while let Some(Reverse((cost, node))) = heap.pop() {
        if node.coords == end {
            return distances[&node];
        }
        if cost > distances[&node] {
            continue;
        }
        for neighbor in node.neighbors(grid, min_steps, max_steps) {
            let next = (cost + neighbor.cost, neighbor);
            if next.0 < *distances.entry(neighbor).or_insert(u32::MAX) {
                distances.insert(neighbor, next.0);
                heap.push(Reverse(next));
            }
        }
    }
    unreachable!();
}

impl Grid {
    pub fn get(&self, x: isize, y: isize) -> Option<u32> {
        if x < 0 || y < 0 {
            return None;
        }
        self.0
            .get(y as usize)
            .and_then(|r| r.get(x as usize))
            .copied()
    }
}

impl Node {
    pub fn neighbors<'a>(
        &self,
        grid: &'a Grid,
        min_steps: u8,
        max_steps: u8,
    ) -> impl Iterator<Item = Node> + 'a {
        let dirs = if self.steps == 0 {
            // Starting node
            vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
        } else {
            let mut dirs = Vec::with_capacity(3);
            if self.steps >= min_steps {
                dirs.push(self.direction.left());
                dirs.push(self.direction.right());
            }
            if self.steps < max_steps {
                dirs.push(self.direction);
            }
            dirs
        };

        let (x, y) = self.coords;
        let steps = self.steps;
        let cur_dir = self.direction;
        dirs.into_iter().filter_map(move |dir| {
            let (dx, dy) = dir.delta();
            let coords = (x + dx, y + dy);
            Some(Node {
                coords,
                cost: grid.get(coords.0, coords.1)?,
                direction: dir,
                steps: if cur_dir == dir { steps + 1 } else { 1 },
            })
        })
    }
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(part1(&parse(&input)), 102);
    }

    #[test]
    fn part2_example() {
        let input = r#"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"#;
        assert_eq!(part2(&parse(&input)), 94);
    }
}
