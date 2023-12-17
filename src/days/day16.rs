use std::collections::VecDeque;

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug, Clone)]
pub struct Grid(Vec<Vec<Cell>>);

#[derive(Debug, Clone)]
struct Cell {
    tile: Tile,
    beams: Vec<Beam>,
}

#[derive(Debug, Clone)]
struct Beam {
    dir: Direction,
    moved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    MirrorL,
    MirrorR,
    SplitVert,
    SplitHori,
}

#[aoc_generator(day16)]
fn parse(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Cell {
                        tile: match c {
                            '.' => Tile::Empty,
                            '\\' => Tile::MirrorL,
                            '/' => Tile::MirrorR,
                            '|' => Tile::SplitVert,
                            '-' => Tile::SplitHori,
                            c => panic!("unknown {c}"),
                        },
                        beams: vec![],
                    })
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[aoc(day16, part1)]
pub fn part1(input: &Grid) -> usize {
    solve(input, (0, 0, Direction::Right))
}

// 22ms on a 5700X. I paid for 8C/16T, let's use 8C/16T!
#[aoc(day16, part2)]
pub fn part2_brute(input: &Grid) -> usize {
    let (w, h) = (input.0[0].len(), input.0.len());
    let corners = [
        (0, 0, Direction::Down),
        (0, 0, Direction::Right),
        (w - 1, 0, Direction::Left),
        (w - 1, 0, Direction::Down),
        (0, h - 1, Direction::Up),
        (0, h - 1, Direction::Right),
        (w - 1, h - 1, Direction::Left),
        (w - 1, h - 1, Direction::Up),
    ];
    let points = corners
        .into_iter()
        .chain((1..w - 1).map(|x| (x, 0, Direction::Down)))
        .chain((1..w - 1).map(|x| (x, h - 1, Direction::Up)))
        .chain((1..h - 1).map(|y| (0, y, Direction::Right)))
        .chain((1..h - 1).map(|y| (w - 1, y, Direction::Left)))
        .collect_vec();
    points
        .into_par_iter()
        .map(|start| solve(input, start))
        .max()
        .unwrap()
}

fn solve(input: &Grid, start: (usize, usize, Direction)) -> usize {
    let mut input = input.clone();
    input
        .get_mut(start.0 as isize, start.1 as isize)
        .unwrap()
        .beams
        .push(Beam {
            dir: start.2,
            moved: false,
        });
    let mut to_move: VecDeque<(usize, usize)> = VecDeque::new();
    to_move.push_back((start.0, start.1));
    while let Some((x, y)) = to_move.pop_back() {
        let cell = &mut input.0[y][x];
        let mut dirs: Vec<Direction> = vec![];
        for beam in cell.beams.iter_mut().filter(|b| !b.moved) {
            match (cell.tile, beam.dir) {
                (Tile::MirrorR, d) => dirs.push(d.reflect_r()),
                (Tile::MirrorL, d) => dirs.push(d.reflect_l()),
                (Tile::SplitVert, d) if d != Direction::Up && d != Direction::Down => {
                    dirs.extend([Direction::Up, Direction::Down]);
                }
                (Tile::SplitHori, d) if d != Direction::Left && d != Direction::Right => {
                    dirs.extend([Direction::Left, Direction::Right]);
                }
                (_, d) => dirs.push(d),
            };
            beam.moved = true;
        }
        for dir in dirs {
            let (dx, dy) = dir.delta();
            if let Some(c) = input.get_mut(x as isize + dx, y as isize + dy) {
                if c.beams.iter().any(|b| b.dir == dir) {
                    continue;
                }
                to_move.push_back(((x as isize + dx) as usize, (y as isize + dy) as usize));
                c.beams.push(Beam { dir, moved: false });
            }
        }
    }
    input
        .0
        .into_iter()
        .flat_map(|r| r.into_iter())
        .filter(|c| !c.beams.is_empty())
        .count()
}

impl Grid {
    fn get_mut(&mut self, x: isize, y: isize) -> Option<&mut Cell> {
        if x < 0 || y < 0 {
            return None;
        }
        self.0
            .get_mut(y as usize)
            .and_then(|r| r.get_mut(x as usize))
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

    fn reflect_r(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn reflect_l(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(part1(&parse(&input)), 46);
    }

    #[test]
    fn part2_example() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
        assert_eq!(part2_brute(&parse(&input)), 51);
    }
}
