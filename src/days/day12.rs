use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Sheet(Vec<Line>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Line {
    springs: Vec<Spring>,
    key: Vec<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Spring {
    Unknown,
    Operational,
    Damaged,
}

#[aoc_generator(day12)]
fn parse(input: &str) -> Sheet {
    let regex = Regex::new(r#"(\d+)"#).unwrap();
    Sheet(
        input
            .lines()
            .map(|l| {
                let key = regex
                    .captures_iter(l)
                    .map(|c| c[1].parse().unwrap())
                    .collect_vec();
                let springs = l
                    .chars()
                    .take_while(|c| *c != ' ')
                    .map(|c| match c {
                        '?' => Spring::Unknown,
                        '.' => Spring::Operational,
                        '#' => Spring::Damaged,
                        c => panic!("unknown {c}"),
                    })
                    .collect_vec();
                Line { key, springs }
            })
            .collect_vec(),
    )
}

#[aoc(day12, part1)]
pub fn part1(input: &Sheet) -> usize {
    input
        .0
        .clone()
        .into_par_iter()
        .map(|line| {
            let mut memo = FxHashMap::default();
            line_arrangements(&line.springs, &line.key, &mut memo)
        })
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &Sheet) -> usize {
    input
        .0
        .clone()
        .into_par_iter()
        .map(|line| {
            let mut memo = FxHashMap::default();
            let mut new = line.springs.clone();
            new.push(Spring::Unknown);
            new = new.repeat(5);
            new.pop();
            line_arrangements(&new, &line.key.repeat(5), &mut memo)
        })
        .sum()
}

fn line_arrangements(
    springs: &[Spring],
    key: &[usize],
    memo: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    let memo_key = (springs.len(), key.len());
    if let Some(n) = memo.get(&memo_key) {
        return *n;
    }
    if springs.is_empty() {
        if !key.is_empty() {
            return 0;
        }
        return 1;
    }
    if key.is_empty() {
        // No groups to match, though it can still be invalid if there are unmatched
        // damaged springs
        if springs.iter().all(|s| *s != Spring::Damaged) {
            return 1;
        } else {
            return 0;
        }
    }
    match springs[0] {
        Spring::Operational => return line_arrangements(&springs[1..], key, memo),
        Spring::Damaged => {
            let len = key[0];

            if len <= springs.len() && springs[..len].iter().all(|&c| c != Spring::Operational) {
                // Need to check the next item as well
                if len >= springs.len() {
                    return line_arrangements(&springs[len..], &key[1..], memo);
                }
                match springs[len] {
                    Spring::Operational | Spring::Unknown => {
                        // Has to be a dot, skip
                        return line_arrangements(&springs[len + 1..], &key[1..], memo);
                    }
                    // Invalid, we matched the group in its entirety and there is another damaged next to it
                    Spring::Damaged => return 0,
                }
            }
            return 0;
        }
        Spring::Unknown => {
            let mut op = springs.to_owned();
            op[0] = Spring::Operational;
            let mut broken = springs.to_owned();
            broken[0] = Spring::Damaged;

            let res = line_arrangements(&op, key, memo) + line_arrangements(&broken, key, memo);
            memo.insert(memo_key, res);
            return res;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(part1(&parse(&input)), 21);
    }

    #[test]
    fn part2_example() {
        let input = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;
        assert_eq!(part2(&parse(&input)), 525152);
    }
}
