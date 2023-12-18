use itertools::Itertools;

use super::day17::Direction;

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    len: isize,
    color_len: isize,
    color_dir: Direction,
}

#[aoc_generator(day18)]
fn parse(input: &str) -> Vec<Movement> {
    input
        .lines()
        .map(|l| {
            let (dir, len, color) = l.split_whitespace().collect_tuple().unwrap();
            Movement {
                direction: match dir {
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    c => panic!("unknown {c}"),
                },
                len: len.parse().unwrap(),
                color_len: isize::from_str_radix(&color[2..color.len() - 2], 16).unwrap(),
                color_dir: match &color[color.len() - 2..color.len() - 1] {
                    "3" => Direction::Up,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "0" => Direction::Right,
                    c => panic!("unknown {c}"),
                },
            }
        })
        .collect_vec()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Movement]) -> isize {
    solve(input, |m| m.direction, |m| m.len)
}

#[aoc(day18, part2)]
pub fn part2(input: &[Movement]) -> isize {
    solve(input, |m| m.color_dir, |m| m.color_len)
}

fn solve(
    input: &[Movement],
    dir_fn: fn(&Movement) -> Direction,
    len_fn: fn(&Movement) -> isize,
) -> isize {
    // An easier day 10, pretty much
    let mut vertices = vec![(0, 0)];
    let mut cur = (0, 0);
    let mut perim = 0;
    for mov in input {
        let (dx, dy) = dir_fn(mov).delta();
        let len = len_fn(mov);
        cur.0 += dx * len;
        cur.1 += dy * len;
        vertices.push(cur);
        perim += len;
    }
    vertices.push((0, 0));
    let area = vertices
        .windows(2)
        .map(|w| {
            let &[a, b] = w else { panic!() };
            a.0 * b.1 - a.1 * b.0
        })
        .sum::<isize>()
        .abs()
        / 2;
    perim + area - perim / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(part1(&parse(&input)), 62);
    }

    #[test]
    fn part2_example() {
        let input = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#;
        assert_eq!(part2(&parse(&input)), 952408144115);
    }
}
