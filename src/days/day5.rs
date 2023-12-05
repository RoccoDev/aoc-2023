use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub struct Sheet {
    seeds: Vec<i64>,
    conversions: Vec<Vec<Conversion>>,
}

#[derive(Debug, Clone, Copy)]
pub struct Conversion {
    dest: Range,
    start: Range,
}

#[derive(Debug, Clone, Copy)]
pub struct Range {
    start: i64,
    end: i64,
}

pub enum OverlapPoint {
    // xxxx[y--x]yyyy
    Left(Range),
    // yyyyy[x--y]xxxx
    Right(Range),
    // yyyy[x----x]yyyy
    Inner(Range),
    None,
}

#[aoc_generator(day5)]
fn parse(input: &str) -> Sheet {
    let regex = Regex::new(r#"(\d+)"#).unwrap();

    let seeds = regex
        .captures_iter(input.lines().next().unwrap())
        .map(|c| c.get(1).unwrap().as_str().parse().unwrap())
        .collect();

    let conv = input
        .split("\n\n")
        .skip(1)
        .map(|l| {
            l.lines()
                .skip(1)
                .map(|l| {
                    let (d, s, l) = regex.captures_iter(l).collect_tuple().unwrap();
                    let len = l.get(1).unwrap().as_str().parse().unwrap();
                    Conversion {
                        dest: Range::new_sl(d.get(1).unwrap().as_str().parse().unwrap(), len),
                        start: Range::new_sl(s.get(1).unwrap().as_str().parse().unwrap(), len),
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    Sheet {
        seeds,
        conversions: conv,
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &Sheet) -> i64 {
    let mut prog = input.seeds.clone();
    for step in &input.conversions {
        for seed in &mut prog {
            'conv: for conv in step {
                if conv.start.contains(*seed) {
                    *seed = conv.dest(*seed);
                    break 'conv;
                }
            }
        }
    }
    prog.into_iter().min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &Sheet) -> i64 {
    let mut seeds = input
        .seeds
        .clone()
        .into_iter()
        .tuples()
        .map(|(start, len)| Range::new_sl(start, len))
        .collect_vec();
    for step in &input.conversions {
        let mut candidates = vec![];

        for mut seed in seeds {
            for Conversion {
                dest,
                start: source,
            } in step
            {
                let dest_idx = dest.start - source.start;
                match seed.overlap(source) {
                    OverlapPoint::Left(inters) => {
                        candidates.push(Range::new(seed.start, source.start - 1));
                        seed.start = inters.start;
                        if seed.end < source.end {
                            // not fully contained: skip to end
                            candidates.push(seed.translate(dest_idx));
                            seed.start = source.end + 1;
                        } else {
                            // fully contained - outer
                            candidates
                                .push(Range::new(seed.start + dest_idx, source.end + dest_idx));
                            seed.start = inters.end;
                        }
                    }
                    OverlapPoint::Right(_) => {
                        // Skip to end
                        candidates.push(seed.translate(dest_idx));
                        seed.start = source.end + 1;
                    }
                    OverlapPoint::Inner(inters) => {
                        candidates.push(Range::new(seed.start + dest_idx, inters.end + dest_idx));
                        seed.start = inters.end;
                    }
                    _ => {}
                }
            }
            // Unmatched
            if !seed.is_empty() {
                candidates.push(seed);
            }
        }
        seeds = candidates;
    }
    seeds.iter().map(|r| r.start).min().unwrap()
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    pub fn new_sl(start: i64, len: i64) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    pub fn contains(&self, seed: i64) -> bool {
        (self.start..self.end).contains(&seed)
    }

    pub fn translate(&self, offset: i64) -> Self {
        Self {
            start: self.start + offset,
            end: self.end + offset,
        }
    }

    pub fn len(&self) -> i64 {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() <= 0
    }

    pub fn overlap(&self, other: &Range) -> OverlapPoint {
        if self.start > other.end || other.start > self.end {
            return OverlapPoint::None;
        }
        if self.start <= other.start {
            return OverlapPoint::Left(Range::new(other.start, self.end));
        }
        if self.end <= other.end {
            return OverlapPoint::Right(Range::new(self.end, other.start));
        }
        OverlapPoint::Inner(Range::new(
            self.start.min(other.start),
            self.end.min(other.end),
        ))
    }
}

impl Conversion {
    pub fn dest(&self, seed: i64) -> i64 {
        self.dest.start + seed - self.start.start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 46);
    }
}
