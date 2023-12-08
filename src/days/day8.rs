use fxhash::FxHashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Sheet {
    directions: Vec<bool>,
    nodes: Vec<Node>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Node {
    id: String,
    left: String,
    right: String,
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Sheet {
    let directions = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'R')
        .collect_vec();
    let regex = Regex::new(r#"(\w+) = \((\w+), (\w+)\)"#).unwrap();
    let nodes = input
        .lines()
        .skip(2)
        .map(|l| {
            let cap = regex.captures_iter(l).next().unwrap();
            Node {
                id: cap[1].to_string(),
                left: cap[2].to_string(),
                right: cap[3].to_string(),
            }
        })
        .collect_vec();
    Sheet { directions, nodes }
}

#[aoc(day8, part1)]
pub fn part1(input: &Sheet) -> usize {
    let nodes: FxHashMap<_, _> = input.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut cur = &nodes["AAA"];
    let dest = &nodes["ZZZ"];
    for (i, &dir) in input.directions.iter().cycle().enumerate() {
        if dir {
            cur = &&nodes[&cur.right];
        } else {
            cur = &&nodes[&cur.left];
        }
        if std::ptr::eq(cur, dest) {
            return i + 1;
        }
    }
    unreachable!()
}

#[aoc(day8, part2)]
pub fn part2(input: &Sheet) -> usize {
    // This actually went like, let's find the first Z and see how many steps it takes for each A.
    // Printed the numbers, tried to pop them into WolframAlpha to quickly get the lcm, worked! :o
    // I thought you had to distinguish between R/L after the Z, but I guess not...

    let nodes: FxHashMap<_, _> = input.nodes.iter().map(|n| (n.id.clone(), n)).collect();
    let mut memo_state: FxHashMap<usize, usize> = FxHashMap::default();
    let mut cur = input
        .nodes
        .iter()
        .filter(|n| n.id.ends_with("A"))
        .collect_vec();
    for (i, &dir) in input.directions.iter().cycle().enumerate() {
        for (j, node) in cur.iter_mut().enumerate() {
            *node = &&nodes[if dir { &node.right } else { &node.left }];
            if node.id.ends_with("Z") {
                if !memo_state.contains_key(&j) {
                    memo_state.insert(j, i + 1);
                }
            }
        }
        if memo_state.len() == cur.len() {
            return lcm(memo_state.into_values());
        }
    }
    unreachable!()
}

fn gcd(mut n: usize, mut d: usize) -> usize {
    while n != 0 && d != 0 {
        n %= d;
        if n == 0 {
            break;
        }
        d %= n;
    }
    n + d
}

fn lcm(nums: impl IntoIterator<Item = usize>) -> usize {
    nums.into_iter().fold(0, |i, c| match (i, c) {
        (0, a) => a,
        (n, b) => n * b / gcd(n, b),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(part1(&parse(&input)), 2);
    }

    #[test]
    fn part1_example2() {
        let input = r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        assert_eq!(part1(&parse(&input)), 6);
    }

    #[test]
    fn lcm() {
        assert_eq!(12, super::lcm([1, 2, 3, 12]));
        assert_eq!(20, super::lcm([5, 4]));
    }

    #[test]
    fn part2_example() {
        let input = r#"LRLR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#;
        assert_eq!(part2(&parse(&input)), 6);
    }
}
