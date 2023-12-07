use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Race {
    time: i64,
    distance: i64,
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<Race> {
    let regex = Regex::new(r#"(\d+)"#).unwrap();
    let (time, distance) = input
        .lines()
        .map(|l| {
            regex
                .captures_iter(l)
                .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        })
        .collect_tuple()
        .unwrap();
    time.zip(distance)
        .map(|(time, distance)| Race::new(time, distance))
        .collect_vec()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Race]) -> i64 {
    input.iter().map(Race::num_ways).product()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Race]) -> i64 {
    // Problem is still really easy, let's try to think of a way to merge the numbers without
    // having to parse the input again :')
    Race::new(
        stack_numbers(input.iter().map(|r| r.time)),
        stack_numbers(input.iter().map(|r| r.distance)),
    )
    .num_ways()
}

fn stack_numbers(nums: impl DoubleEndedIterator<Item = i64>) -> i64 {
    nums.rev()
        .fold((0, 1), |(res, pow), n| {
            let digits = (n as f64).log10().ceil() as u32;
            (res + pow * n, pow * 10i64.pow(digits))
        })
        .0
}

impl Race {
    pub fn new(time: i64, distance: i64) -> Self {
        assert!(time as f64 > 2.0 * (distance as f64).sqrt());
        Self { time, distance }
    }

    pub fn num_ways(&self) -> i64 {
        // Less dev time (initial solution): simple brute force, could even optimize with binary search
        // (P2 with no optimizations runs in ~25ms on real inputs)

        // https://www.wolframalpha.com/input?i=%28t-x%29*x+%3E+a%2C+t+%3E+0%2C+a+%3E+0
        let time = self.time as f64;
        let discrim_sqrt = ((self.time * self.time - 4 * self.distance) as f64).sqrt();
        let min = ((time - discrim_sqrt) * 0.5) as i64 + 1;
        let max = ((time + discrim_sqrt) * 0.5).ceil() as i64 - 1;

        max + 1 - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(part1(&parse(&input)), 288);
    }

    #[test]
    fn part2_example() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(part2(&parse(&input)), 71503);
    }
}
