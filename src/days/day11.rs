use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Galaxies {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Galaxies {
    let w = input.lines().next().unwrap().len();
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect();
    let empty_rows = input
        .lines()
        .enumerate()
        .filter(|(_, c)| c.chars().all(|c| c == '.'))
        .map(|(y, _)| y)
        .collect_vec();
    let empty_columns = (0..w)
        .filter(|x| input.lines().all(|l| l.as_bytes()[*x] == b'.'))
        .collect_vec();
    Galaxies {
        galaxies,
        empty_columns,
        empty_rows,
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &Galaxies) -> usize {
    solve(input, 2)
}

#[aoc(day11, part2)]
pub fn part2(input: &Galaxies) -> usize {
    solve(input, 1_000_000)
}

pub fn solve(input: &Galaxies, add: usize) -> usize {
    let mut galaxies = input.galaxies.clone();
    let add = add - 1;
    for galaxy in &mut galaxies {
        let orig_x = galaxy.0;
        let orig_y = galaxy.1;
        for x in &input.empty_columns {
            if orig_x >= *x {
                galaxy.0 += add;
            }
        }
        for y in &input.empty_rows {
            if orig_y >= *y {
                galaxy.1 += add;
            }
        }
    }

    galaxies
        .iter()
        .combinations(2)
        .into_iter()
        .map(|comb| {
            let &[a, b] = &comb[..] else { panic!() };
            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(part1(&parse(&input)), 374);
    }

    #[test]
    fn part2_example() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        assert_eq!(solve(&parse(&input), 10), 1030);
        assert_eq!(solve(&parse(&input), 100), 8410);
    }
}
