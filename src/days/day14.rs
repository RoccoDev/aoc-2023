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
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
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
    'coords: for (y, x) in (0..input.0.len()).cartesian_product(0..input.0[0].len()) {
        let c = input.0[y][x];
        if c == Tile::Rock && y > 0 && input.0[y - 1][x] == Tile::Empty {
            let mut last_y = y;
            for y in (0..y).rev() {
                if input.0[y][x] == Tile::Empty {
                    input.0[y][x] = c;
                    input.0[last_y][x] = Tile::Empty;
                    last_y = y;
                } else {
                    continue 'coords;
                }
            }
        }
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

#[aoc(day14, part2)]
pub fn part2(input: &Grid) -> usize {
    let mut input = input.clone();
    let mut memo = FxHashMap::default();
    let mut done = false;
    let mut i = 0;
    while i < 1_000_000_000usize * 4 {
        let dir = [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ][i % 4];
        let res = input.move_dir(dir, &mut memo, i);
        if !done {
            if let Some((cycle, j)) = res {
                input = cycle;
                let cycle_len = i - j;
                done = true;
                while i + cycle_len < 1_000_000_000usize * 4 {
                    i += cycle_len;
                }
            }
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
        memo: &mut FxHashMap<(Grid, Direction), (Grid, usize)>,
        i: usize,
    ) -> Option<(Grid, usize)> {
        let key = (self.clone(), dir);
        if let Some(g) = memo.get(&key) {
            self.clone_from(&g.0);
            return Some(g.clone());
        }
        loop {
            let mut moved = false;
            match dir {
                Direction::North => {
                    for y in 0..self.0.len() {
                        for x in 0..self.0[0].len() {
                            moved |= self.move_pos(x, y, 0, -1);
                        }
                    }
                }
                Direction::South => {
                    for y in (0..self.0.len() - 1).rev() {
                        for x in 0..self.0[0].len() {
                            moved |= self.move_pos(x, y, 0, 1);
                        }
                    }
                }
                Direction::East => {
                    for x in (0..self.0[0].len() - 1).rev() {
                        for y in 0..self.0.len() {
                            moved |= self.move_pos(x, y, 1, 0);
                        }
                    }
                }
                Direction::West => {
                    for x in 0..self.0[0].len() {
                        for y in 0..self.0.len() {
                            moved |= self.move_pos(x, y, -1, 0);
                        }
                    }
                }
            }

            if !moved {
                memo.insert(key, (self.clone(), i));
                return None;
            }
        }
    }

    fn move_pos(&mut self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let c = self.0[y][x];
        let new_y = y as isize + dy;
        let new_x = x as isize + dx;
        if c == Tile::Rock
            && new_y >= 0
            && new_x >= 0
            && self.0[new_y as usize][new_x as usize] == Tile::Empty
        {
            self.0[new_y as usize][new_x as usize] = c;
            self.0[y][x] = Tile::Empty;
            return true;
        }
        false
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
