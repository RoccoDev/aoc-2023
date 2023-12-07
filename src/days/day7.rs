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
                    c @ '2'..='9' => 5 + 9 - c.to_digit(10).unwrap() as i64,
                    c => panic!("invalid digit {c}"),
                };
                10i64.pow((5 - i as u32) * 2) * res
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
            &[1, 2, 2] => 2,
            &[1, 1, 1, 2] => 1,
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
                    c @ '2'..='9' => 4 + 9 - c.to_digit(10).unwrap() as i64,
                    'J' => 12,
                    c => panic!("invalid digit {c}"),
                };
                10i64.pow((5 - i as u32) * 2) * res
            })
            .sum();

        chars.sort_unstable();
        let new_chars = chars.iter().copied().filter(|&c| c != 'J').collect_vec();
        let js = chars.len() - new_chars.len();
        chars = new_chars;

        let mut max_score = -1;
        let mut buf = vec![];
        for perm in repeat_n(
            ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'],
            js,
        )
        .multi_cartesian_product().chain(std::iter::once(vec![]))
        {
            if perm.is_empty() && js != 0 {
                continue;
            } 
            chars.clone_into(&mut buf);
            buf.extend(perm);
            buf.sort_unstable();
            let mut max_dups = buf
                .iter()
                .dedup_with_count()
                .map(|(num, _)| num)
                .collect_vec();
            max_dups.sort_unstable();
            let dup_score = match &max_dups[..] {
                &[5] =>  {
                    max_score = 6;
                    break;
                },
                &[1, 4] => 5,
                &[2, 3] => 4,
                &[1, 1, 3] => 3,
                &[1, 2, 2] => 2,
                &[1, 1, 1, 2] => 1,
                &[1, 1, 1, 1, 1] => 0,
                c => panic!("invalid config {c:?}"),
            };
            if dup_score > max_score {
                max_score = dup_score;
            }
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
        assert_eq!(part1(&parse(input)), 6440);
    }

    #[test]
    fn part2_example() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(part2(&parse(input)), 5905);
    }
}
