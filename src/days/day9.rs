use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Sequence(Vec<i32>);

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<Sequence> {
    let regex = Regex::new(r#"(-?\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            Sequence(
                regex
                    .captures_iter(l)
                    .map(|c| c[1].parse().unwrap())
                    .collect_vec(),
            )
        })
        .collect_vec()
}

#[aoc(day9, part1)]
pub fn part1(input: &[Sequence]) -> i32 {
    input
        .iter()
        .map(|s| {
            let mut next = s.0[s.0.len() - 1];
            let mut buf = s.0.clone();
            while buf.iter().any(|&n| n != 0) {
                let old = buf.clone();
                buf.clear();
                for window in old.windows(2) {
                    let &[a, b] = window else { panic!() };
                    buf.push(b - a);
                }
                next += buf[buf.len() - 1];
            }
            next
        })
        .sum()
}

#[aoc(day9, part2)]
pub fn part2(input: &[Sequence]) -> i32 {
    input
        .iter()
        .map(|s| {
            let mut next = s.0[0];
            let mut buf = s.0.clone();
            while buf.iter().any(|&n| n != 0) {
                let old = buf.clone();
                buf.clear();
                for window in old.windows(2) {
                    let &[a, b] = window else { panic!() };
                    buf.push(a - b);
                }
                next += buf[0];
            }
            next
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(part1(&parse(&input)), 114);
    }

    #[test]
    fn part2_example() {
        let input = r#"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"#;
        assert_eq!(part2(&parse(&input)), 2);
    }
}
