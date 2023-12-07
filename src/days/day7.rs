use itertools::{repeat_n, Itertools};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hand {
    pub dup_score: i32,
    pub card_score: i64,
    pub bid: i32,
}

#[derive(Debug, Clone)]
pub struct HandDef {
    pub name: Vec<char>,
    pub bid: i32,
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Vec<HandDef> {
    input
        .lines()
        .map(|l| {
            let (chars, bid) = l.split_whitespace().collect_tuple().unwrap();
            let chars = chars.chars().collect_vec();
            let bid = bid.parse().unwrap();
            HandDef { name: chars, bid }
        })
        .collect_vec()
}

#[aoc(day7, part1)]
pub fn part1(input: &[HandDef]) -> usize {
    let mut hands = input.iter().map(HandDef::calc).collect_vec();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, c)| c.bid as usize * (i + 1))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[HandDef]) -> usize {
    let mut hands = input.iter().map(HandDef::calc_p2).collect_vec();
    hands.sort_unstable();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, c)| c.bid as usize * (i + 1))
        .sum()
}

// Very ugly, quite slow (~24ms), don't have much time now, will fix later
impl HandDef {
    pub fn calc(&self) -> Hand {
        let mut chars = self.name.clone();
        let score: i64 = chars
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let res = match c {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'J' => 3,
                    'T' => 4,
                    '9' => 5,
                    '8' => 6,
                    '7' => 7,
                    '6' => 8,
                    '5' => 9,
                    '4' => 10,
                    '3' => 11,
                    '2' => 12,
                    c => panic!("invalid digit {c}"),
                };
                10i64.pow((6 - i as u32) * 2) * res
            })
            .sum();
        chars.sort_unstable();

        let mut max_dups = chars
            .into_iter()
            .dedup_with_count()
            .map(|(num, _)| num)
            .collect_vec();
        max_dups.sort_unstable();
        let dup_score = match &max_dups[..] {
            &[5] => 6,
            &[1, 4] => 5,
            &[2, 3] => 4,
            &[1, 1, 3] => 3,
            v if v.iter().filter(|&&c| c == 2).count() == 2 => 2,
            v if v.iter().filter(|&&c| c == 2).count() == 1 => 1,
            &[1, 1, 1, 1, 1] => 0,
            c => panic!("invalid config {c:?}"),
        };
        Hand {
            dup_score,
            card_score: -score,
            bid: self.bid,
        }
    }

    pub fn calc_p2(&self) -> Hand {
        let mut chars = self.name.clone();
        let score: i64 = chars
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let res = match c {
                    'A' => 0,
                    'K' => 1,
                    'Q' => 2,
                    'T' => 3,
                    '9' => 4,
                    '8' => 5,
                    '7' => 6,
                    '6' => 7,
                    '5' => 8,
                    '4' => 9,
                    '3' => 10,
                    '2' => 11,
                    'J' => 12,
                    c => panic!("invalid digit {c}"),
                };
                10i64.pow((6 - i as u32) * 2) * res
            })
            .sum();
        chars.sort_unstable();

        chars = chars.iter().copied().filter(|&c| c != 'J').collect_vec();

        let mut max_score = -1;
        for perm in repeat_n(
            ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'].iter(),
            5 - chars.len(),
        )
        .multi_cartesian_product()
        {
            let mut chars = chars.clone();
            chars.extend(perm);
            chars.sort_unstable();
            let mut max_dups = chars
                .iter()
                .dedup_with_count()
                .map(|(num, _)| num)
                .collect_vec();
            max_dups.sort_unstable();
            let dup_score = match &max_dups[..] {
                &[5] => 6,
                &[1, 4] => 5,
                &[2, 3] => 4,
                &[1, 1, 3] => 3,
                v if v.iter().filter(|&&c| c == 2).count() == 2 => 2,
                v if v.iter().filter(|&&c| c == 2).count() == 1 => 1,
                &[1, 1, 1, 1, 1] => 0,
                c => panic!("invalid config {c:?}"),
            };
            if dup_score > max_score {
                max_score = dup_score;
            }
        }

        if max_score < 0 {
            let mut max_dups = chars
                .iter()
                .dedup_with_count()
                .map(|(num, _)| num)
                .collect_vec();
            max_dups.sort_unstable();
            max_score = match &max_dups[..] {
                &[5] => 6,
                &[1, 4] => 5,
                &[2, 3] => 4,
                &[1, 1, 3] => 3,
                v if v.iter().filter(|&&c| c == 2).count() == 2 => 2,
                v if v.iter().filter(|&&c| c == 2).count() == 1 => 1,
                &[1, 1, 1, 1, 1] => 0,
                c => panic!("invalid config {c:?}"),
            };
        }

        Hand {
            dup_score: max_score,
            card_score: -score,
            bid: self.bid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part1(&parse(&input)), 288);
    }

    #[test]
    fn part2_example() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part2(&parse(&input)), 71503);
    }
}
