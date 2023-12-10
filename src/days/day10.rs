use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
    Vert,
    Hori,
    Ne,
    Nw,
    Sw,
    Se,
    Ground,
    Start,
}

pub struct Grid {
    tiles: Vec<Vec<Tile>>,
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Grid {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '|' => Tile::Vert,
                    '-' => Tile::Hori,
                    'L' => Tile::Ne,
                    'J' => Tile::Nw,
                    '7' => Tile::Sw,
                    'F' => Tile::Se,
                    '.' => Tile::Ground,
                    'S' => Tile::Start,
                    _ => todo!(),
                })
                .collect_vec()
        })
        .collect_vec();
    Grid { tiles: grid }
}

#[aoc(day10, part1)]
pub fn part1(input: &Grid) -> i32 {
    let start = input.get_start();
    let start_tile = input.calc_tile(start);
    let (mut way_a, mut way_b) = start_tile.neighbors(start);
    let (mut prev_a, mut prev_b) = (start, start);
    let (mut steps_a, mut steps_b) = (0, 0);
    while way_a != way_b {
        steps_a += 1;
        steps_b += 1;
        let new_a = input.next(way_a, prev_a);
        let new_b = input.next(way_b, prev_b);
        prev_a = way_a;
        prev_b = way_b;
        way_a = new_a;
        way_b = new_b;
    }
    steps_a.max(steps_b) + 1
}

#[aoc(day10, part2)]
pub fn part2(input: &Grid) -> isize {
    let start = input.get_start();
    let start_tile = input.calc_tile(start);
    let mut vertices = vec![start];
    let mut way = start_tile.neighbors(start).1;
    let mut prev = start;
    let mut perimeter = 1;

    while way != start {
        let tile = input.tiles[way.1][way.0];
        if tile.is_junction() {
            vertices.push(way);
        }
        perimeter += 1;

        let new = input.next(way, prev);
        prev = way;
        way = new;
    }

    vertices.push(start);

    let area = vertices
        .windows(2)
        .map(|w| {
            let &[a, b] = w else { panic!() };
            a.0 as isize * b.1 as isize - a.1 as isize * b.0 as isize
        })
        .sum::<isize>()
        .abs()
        / 2;
    area - perimeter / 2 + 1
}

impl Grid {
    pub fn next(&self, cur: (usize, usize), prev: (usize, usize)) -> (usize, usize) {
        let tile = self.tiles[cur.1][cur.0];
        let (a, b) = tile.neighbors(cur);
        if a == prev {
            b
        } else {
            a
        }
    }

    pub fn get_start(&self) -> (usize, usize) {
        for (y, r) in self.tiles.iter().enumerate() {
            for (x, c) in r.iter().enumerate() {
                if *c == Tile::Start {
                    return (x, y);
                }
            }
        }
        unreachable!()
    }

    pub fn calc_tile(&self, start: (usize, usize)) -> Tile {
        let start_i = (start.0 as isize, start.1 as isize);
        let dirs = [(0, -1), (0, 1), (1, 0), (-1, 0)];
        let (dir_a, dir_b) = dirs
            .into_iter()
            .filter(|&(x, y)| {
                if x == 0 && y == 0 || start_i.0 + x < 0 || start_i.1 + y < 0 {
                    return false;
                }
                self.tiles
                    .get((start_i.1 + y) as usize)
                    .and_then(|r| r.get((start_i.0 + x) as usize))
                    .is_some_and(|t| *t != Tile::Ground)
            })
            .filter(|&(x, y)| {
                let (x, y) = ((start_i.0 + x) as usize, (start_i.1 + y) as usize);
                let tile = self.tiles[y][x];
                let (a, b) = tile.neighbors((x, y));
                a == start || b == start
            })
            .collect_tuple()
            .expect("not exactly 2 possible directions");
        match (dir_a, dir_b) {
            ((0, -1), (0, 1)) => Tile::Vert,
            ((0, -1), (1, 0)) => Tile::Ne,
            ((0, -1), (-1, 0)) => Tile::Nw,
            ((0, 1), (1, 0)) => Tile::Se,
            ((0, 1), (-1, 0)) => Tile::Sw,
            ((-1, 0), (1, 0)) => Tile::Hori,
            c => panic!("unknown {c:?}"),
        }
    }
}

impl Tile {
    pub fn is_junction(&self) -> bool {
        !matches!(self, Self::Vert | Self::Hori | Self::Ground | Self::Start)
    }

    pub fn neighbors(&self, cur: (usize, usize)) -> ((usize, usize), (usize, usize)) {
        match self {
            Tile::Vert => ((cur.0, cur.1 - 1), (cur.0, cur.1 + 1)),
            Tile::Hori => ((cur.0 - 1, cur.1), (cur.0 + 1, cur.1)),
            Tile::Ne => ((cur.0, cur.1 - 1), (cur.0 + 1, cur.1)),
            Tile::Nw => ((cur.0, cur.1 - 1), (cur.0 - 1, cur.1)),
            Tile::Sw => ((cur.0, cur.1 + 1), (cur.0 - 1, cur.1)),
            Tile::Se => ((cur.0, cur.1 + 1), (cur.0 + 1, cur.1)),
            Tile::Ground => todo!(),
            Tile::Start => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = r#".....
.S-7.
.|.|.
.L-J.
....."#;
        assert_eq!(part1(&parse(&input)), 4);
    }

    #[test]
    fn part1_example2() {
        let input = r#"..F7.
.FJ|.
SJ.L7
|F--J
LJ..."#;
        assert_eq!(part1(&parse(&input)), 8);
    }

    #[test]
    fn part2_example() {
        let input = r#"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."#;
        assert_eq!(part2(&parse(&input)), 4);
    }
}
