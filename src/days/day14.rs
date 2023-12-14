use fxhash::FxHashMap;
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid(Vec<Vec<Tile>>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Rock,
    Empty,
    Obstacle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    North,
    West,
    South,
    East,
}

#[aoc_generator(day14)]
fn parse(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        'O' => Tile::Rock,
                        '#' => Tile::Obstacle,
                        c => panic!("unknown {c:?}"),
                    })
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[aoc(day14, part1)]
pub fn part1(input: &Grid) -> usize {
    let mut input = input.clone();
    input.move_dir(Direction::North, None);
    let max_y = input.0.len();
    input
        .0
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| l.into_iter().map(move |c| (y, c)))
        .filter(|(_, c)| *c == Tile::Rock)
        .map(|(y, _)| max_y - y)
        .sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &Grid) -> usize {
    const TARGET: usize = 1_000_000_000 * 4;

    let mut input = input.clone();
    let mut memo = FxHashMap::default();
    let mut i = 0;
    while i < TARGET {
        let dir = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ][i % 4];
        let res = input.move_dir(dir, Some((&mut memo, i)));
        if let Some((cycle, j)) = res {
            input = cycle;
            let cycle_len = i - j;
            i += (TARGET - i) / cycle_len * cycle_len;
        }
        i += 1;
    }
    let max_y = input.0.len();
    input
        .0
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| l.into_iter().map(move |c| (y, c)))
        .filter(|(_, c)| *c == Tile::Rock)
        .map(|(y, _)| max_y - y)
        .sum()
}

impl Grid {
    pub fn move_dir(
        &mut self,
        dir: Direction,
        memo: Option<(&mut FxHashMap<(Grid, Direction), (Grid, usize)>, usize)>,
    ) -> Option<(Grid, usize)> {
        let mut key = None;
        if let Some((memo, _)) = &memo {
            let k = (self.clone(), dir);
            if let Some(g) = memo.get(&k) {
                self.clone_from(&g.0);
                return Some(g.clone());
            }
            key = Some(k);
        }
        match dir {
            // Iterator types and logic differ between direction, hard to extract
            Direction::North => {
                'coords: for (y, x) in (0..self.0.len()).cartesian_product(0..self.0[0].len()) {
                    let c = self.0[y][x];
                    if c == Tile::Rock && y > 0 && self.0[y - 1][x] == Tile::Empty {
                        let mut last_y = y;
                        for y in (0..y).rev() {
                            if self.0[y][x] == Tile::Empty {
                                self.0[y][x] = c;
                                self.0[last_y][x] = Tile::Empty;
                                last_y = y;
                            } else {
                                continue 'coords;
                            }
                        }
                    }
                }
                return None;
            }
            Direction::South => {
                'coords: for (y, x) in (0..self.0.len() - 1)
                    .rev()
                    .cartesian_product(0..self.0[0].len())
                {
                    let c = self.0[y][x];
                    if c == Tile::Rock && self.0[y + 1][x] == Tile::Empty {
                        let mut last_y = y;
                        for y in y + 1..self.0.len() {
                            if self.0[y][x] == Tile::Empty {
                                self.0[y][x] = c;
                                self.0[last_y][x] = Tile::Empty;
                                last_y = y;
                            } else {
                                continue 'coords;
                            }
                        }
                    }
                }
            }
            Direction::East => {
                'coords: for (x, y) in
                    ((0..self.0[0].len() - 1).rev()).cartesian_product(0..self.0.len())
                {
                    let c = self.0[y][x];
                    if c == Tile::Rock && self.0[y][x + 1] == Tile::Empty {
                        let mut last_x = x;
                        for x in x + 1..self.0[0].len() {
                            if self.0[y][x] == Tile::Empty {
                                self.0[y][x] = c;
                                self.0[y][last_x] = Tile::Empty;
                                last_x = x;
                            } else {
                                continue 'coords;
                            }
                        }
                    }
                }
            }
            Direction::West => {
                'coords: for (y, x) in (0..self.0.len()).cartesian_product(0..self.0[0].len()) {
                    let c = self.0[y][x];
                    if c == Tile::Rock && x > 0 && self.0[y][x - 1] == Tile::Empty {
                        let mut last_x = x;
                        for x in (0..x).rev() {
                            if self.0[y][x] == Tile::Empty {
                                self.0[y][x] = c;
                                self.0[y][last_x] = Tile::Empty;
                                last_x = x;
                            } else {
                                continue 'coords;
                            }
                        }
                    }
                }
            }
        }

        if let Some(key) = key {
            let (memo, i) = memo.unwrap();
            memo.insert(key, (self.clone(), i));
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(part1(&parse(&input)), 136);
    }

    #[test]
    fn part2_example() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        assert_eq!(part2(&parse(&input)), 64);
    }
}
