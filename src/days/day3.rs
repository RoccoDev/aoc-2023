use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Slot {
    ty: SlotType,
    coords: (usize, usize),
    len: usize,
}

#[derive(Debug)]

enum SlotType {
    Number(i32),
    Symbol(char),
}

#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Slot> {
    let regex = Regex::new(r#"(\d+)|([^\d\.\s]{1})"#).unwrap();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            regex
                .captures_iter(l)
                .map(|m| {
                    let m = m.get(1).or_else(|| m.get(2)).unwrap();
                    let x = m.start();
                    let st = m.as_str();
                    let first = st.chars().next().unwrap();
                    let ty = if first.is_ascii_digit() {
                        SlotType::Number(st.parse().unwrap())
                    } else {
                        SlotType::Symbol(first)
                    };
                    Slot {
                        ty,
                        coords: (x, y),
                        len: st.len(),
                    }
                })
                .collect_vec()
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &[Slot]) -> i32 {
    let grid_max_x = input.iter().map(|s| s.coords.0).max().unwrap();
    let grid_max_y = input.iter().map(|s| s.coords.1).max().unwrap();

    let mut grid = vec![vec![false; grid_max_x + 1]; grid_max_y + 1];
    for slot in input {
        if let SlotType::Symbol(_) = slot.ty {
            grid[slot.coords.1][slot.coords.0] = true;
        }
    }

    let mut sum = 0;
    'slots: for slot in input {
        if let SlotType::Number(n) = slot.ty {
            let (min_x, min_y) = slot.start();
            let (max_x, max_y) = slot.end();
            for y in min_y.saturating_sub(1)..=max_y.saturating_add(1).min(grid_max_y) {
                for x in min_x.saturating_sub(1)..=max_x.saturating_add(1).min(grid_max_x) {
                    if grid[y][x] {
                        sum += n;
                        continue 'slots;
                    }
                }
            }
        }
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &[Slot]) -> i32 {
    let grid_max_x = input.iter().map(|s| s.coords.0).max().unwrap();
    let grid_max_y = input.iter().map(|s| s.coords.1).max().unwrap();

    let mut grid = vec![vec![(0i32, 0i32); grid_max_x + 1]; grid_max_y + 1];
    for slot in input {
        if let SlotType::Symbol(_) = slot.ty {
            grid[slot.coords.1][slot.coords.0] = (1, 0);
        }
    }

    for slot in input {
        if let SlotType::Number(n) = slot.ty {
            let (min_x, min_y) = slot.start();
            let (max_x, max_y) = slot.end();
            for y in min_y.saturating_sub(1)..=max_y.saturating_add(1).min(grid_max_y) {
                for x in min_x.saturating_sub(1)..=max_x.saturating_add(1).min(grid_max_x) {
                    let (prod, num) = grid[y][x];
                    grid[y][x] = (prod * n, num + 1);
                }
            }
        }
    }

    input
        .iter()
        .filter(|s| matches!(s.ty, SlotType::Symbol('*')) && grid[s.coords.1][s.coords.0].1 == 2)
        .map(|s| grid[s.coords.1][s.coords.0].0)
        .sum()
}

impl Slot {
    fn start(&self) -> (usize, usize) {
        (self.coords.0, self.coords.1)
    }

    fn end(&self) -> (usize, usize) {
        (self.coords.0 + self.len - 1, self.coords.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part1(&parse(&input)), 4361);
    }

    #[test]
    fn part2_example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(part2(&parse(&input)), 467835);
    }
}
