use fxhash::FxHashSet;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Card {
    id: i32,
    winners: FxHashSet<i32>,
    own: FxHashSet<i32>,
}

#[aoc_generator(day4)]
fn parse(input: &str) -> Vec<Card> {
    let regex = Regex::new(r#"(\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            let (winners, own) = l.split(" | ").collect_tuple().unwrap();
            let winners = regex
                .captures_iter(winners)
                .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
                .collect_vec();
            let own = regex
                .captures_iter(own)
                .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
                .collect();

            Card {
                id: winners[0],
                winners: winners[1..].into_iter().copied().collect(),
                own,
            }
        })
        .collect()
}

#[aoc(day4, part1)]
pub fn part1(input: &[Card]) -> i32 {
    input
        .iter()
        .map(|c| {
            let count = c.own.intersection(&c.winners).count();
            if count > 0 {
                1 << count - 1
            } else {
                0
            }
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &[Card]) -> usize {
    let min_id = input.iter().map(|c| c.id).min().unwrap() as usize;
    let max_id = input.iter().map(|c| c.id).max().unwrap() as usize;

    let mut counts = vec![1usize; max_id];

    for i in min_id - 1..max_id {
        for _ in 0..counts[i] {
            let card = &input[i];
            let winners = card.own.intersection(&card.winners).count();
            for w in 1..=winners {
                counts[i + w] += 1;
            }
        }
    }

    counts.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(part1(&parse(&input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(part2(&parse(&input)), 30);
    }
}
