use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Brick {
    id: usize,
    min: (isize, isize, isize),
    max: (isize, isize, isize),
}

#[aoc_generator(day22)]
fn parse(input: &str) -> Vec<Brick> {
    let regex = Regex::new(r#"(-?\d+)"#).unwrap();
    input
        .lines()
        .enumerate()
        .map(|(id, l)| {
            let (ax, ay, az, bx, by, bz) = regex
                .captures_iter(l)
                .map(|s| s[1].parse::<isize>().unwrap())
                .collect_tuple()
                .unwrap();
            Brick {
                id,
                min: (ax.min(bx), ay.min(by), az.min(bz)),
                max: (ax.max(bx), ay.max(by), az.max(bz)),
            }
        })
        .collect_vec()
}

#[aoc(day22, part1)]
pub fn part1(input: &[Brick]) -> usize {
    let mut bricks = input.to_vec();
    move_down(&mut bricks);

    let mut supported_by_count = vec![0; bricks.len()];
    let mut supported = vec![vec![]; bricks.len()];

    for brick in &bricks {
        let down = brick.moved_down();
        for other in &bricks {
            if other.id == brick.id {
                continue;
            }
            if other.intersects(&down) {
                supported_by_count[down.id] += 1;
                supported[other.id].push(down.id);
            }
        }
    }
    supported
        .into_iter()
        .filter(|supported| supported.iter().all(|sub| supported_by_count[*sub] > 1))
        .count()
}

#[aoc(day22, part2)]
pub fn part2(input: &[Brick]) -> usize {
    let mut bricks = input.to_vec();
    move_down(&mut bricks);

    let mut supported_by_count = vec![0; bricks.len()];
    let mut supported = vec![vec![]; bricks.len()];

    for brick in &bricks {
        let down = brick.moved_down();
        for other in &bricks {
            if other.id == brick.id {
                continue;
            }
            if other.intersects(&down) {
                supported_by_count[down.id] += 1;
                supported[other.id].push(down.id);
            }
        }
    }

    let unsafe_ids = supported
        .clone()
        .into_iter()
        .enumerate()
        .filter(|(_, supported)| supported.iter().any(|sub| supported_by_count[*sub] <= 1))
        .map(|(i, _)| i)
        .collect_vec();

    let mut total = 0;
    for id in unsafe_ids {
        let mut chain = VecDeque::new();
        chain.push_back(id);
        let mut supported_by_count = supported_by_count.clone();

        while let Some(id) = chain.pop_back() {
            for i in &supported[id] {
                let old = supported_by_count[*i];
                supported_by_count[*i] -= 1;
                if old != 0 && supported_by_count[*i] == 0 {
                    total += 1;
                    chain.push_back(*i);
                }
            }
        }
    }

    total
}

fn move_down(bricks: &mut [Brick]) {
    bricks.sort_unstable_by_key(|b| b.min.2);
    loop {
        let mut changed = false;
        for i in 0..bricks.len() {
            loop {
                let brick = bricks[i];
                let moved = brick.moved_down();
                if moved.min.2 <= 0 {
                    break;
                }
                if bricks
                    .iter()
                    .all(|b| b.id == brick.id || !b.intersects(&moved))
                {
                    bricks[i] = moved;
                    changed = true;
                } else {
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }
}

impl Brick {
    fn intersects(&self, other: &Self) -> bool {
        (other.max.0 >= self.min.0 && other.min.0 <= self.max.0)
            && (other.max.1 >= self.min.1 && other.min.1 <= self.max.1)
            && (other.max.2 >= self.min.2 && other.min.2 <= self.max.2)
    }

    fn moved_down(&self) -> Self {
        let mut cpy = *self;
        cpy.min.2 -= 1;
        cpy.max.2 -= 1;
        cpy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
        assert_eq!(part1(&parse(&input)), 5);
    }

    #[test]
    fn part2_example() {
        let input = r#"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"#;
        assert_eq!(part2(&parse(&input)), 7);
    }
}
