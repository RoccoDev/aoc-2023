use std::collections::VecDeque;

use fxhash::FxHashSet;
use itertools::Itertools;

use super::day17::Direction;

#[derive(Debug, Clone)]
pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Wall,
    Empty,
    Start,
}

#[aoc_generator(day21)]
fn parse(input: &str) -> Grid {
    Grid {
        tiles: input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'S' => Tile::Start,
                        c => panic!("unknown {c}"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &Grid) -> usize {
    solve(input, 64, false)
}

#[aoc(day21, part2)]
pub fn part2(input: &Grid) -> usize {
    let fin: usize = 26501365;
    // Edges are empty
    let max = fin % input.tiles.len();

    // Quadratic sequence, find the first three nums in the sequence and try
    // to derive a formula
    let seq_1 = solve(input, max, true);
    let seq_2 = solve(input, max + input.tiles.len(), true);
    let seq_3 = solve(input, max + 2 * input.tiles.len(), true);

    let coeff_1 = ((seq_3 - seq_2) - (seq_2 - seq_1)) / 2;
    let coeff_2 = (seq_2 - seq_1) - 3 * coeff_1;
    let coeff_3 = seq_1 - coeff_2 - coeff_1;

    let target = fin.div_ceil(input.tiles.len());
    coeff_1 * target * target + coeff_2 * target + coeff_3
}

fn solve(input: &Grid, max_steps: usize, p2: bool) -> usize {
    let mut to_move = VecDeque::new();
    let start = input
        .tiles
        .iter()
        .enumerate()
        .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, t)| (x, y, *t)))
        .find(|(_, _, t)| *t == Tile::Start)
        .unwrap();
    to_move.push_back((start.0 as isize, start.1 as isize, 0));

    let mut total = 0;
    let mut visited = FxHashSet::default();
    let target = max_steps % 2;

    while let Some((x, y, steps)) = to_move.pop_back() {
        if visited.contains(&(x, y)) {
            continue;
        }
        if steps > max_steps {
            break;
        }
        if steps % 2 == target {
            total += 1;
        }
        visited.insert((x, y));
        if p2 {
            for neigh in input.neighbors_2((x, y)) {
                if !visited.contains(&(neigh.0, neigh.1)) {
                    to_move.push_front((neigh.0, neigh.1, steps + 1));
                }
            }
        } else {
            for neigh in input.neighbors((x, y)) {
                if !visited.contains(&(neigh.0, neigh.1)) {
                    to_move.push_front((neigh.0, neigh.1, steps + 1));
                }
            }
        }
    }

    total
}

impl Grid {
    pub fn neighbors(&self, cur: (isize, isize)) -> impl Iterator<Item = (isize, isize)> + '_ {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(move |d| (cur.0 + d.delta().0, cur.1 + d.delta().1))
        .filter(move |(x, y)| {
            self.tiles
                .get(*y as usize)
                .and_then(|r| r.get(*x as usize))
                .is_some_and(|t| *t == Tile::Start || *t == Tile::Empty)
        })
    }

    pub fn neighbors_2(&self, cur: (isize, isize)) -> impl Iterator<Item = (isize, isize)> + '_ {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
        .map(move |d| (cur.0 + d.delta().0, cur.1 + d.delta().1))
        .filter(move |(x, y)| {
            let x = x.rem_euclid(self.tiles[0].len() as isize);
            let y = y.rem_euclid(self.tiles.len() as isize);
            self.tiles
                .get(y as usize)
                .and_then(|r| r.get(x as usize))
                .is_some_and(|t| *t == Tile::Start || *t == Tile::Empty)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#;
        assert_eq!(solve(&parse(&input), 6, false), 16);
    }
}
