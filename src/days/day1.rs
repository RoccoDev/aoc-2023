use itertools::Itertools;

// We don't use replace over here
#[derive(Clone, Copy, Debug)]
struct Parser {
    state: ParserState,
    sub: usize,
}

#[derive(Debug)]
enum ParserResult {
    MatchAlpha(u32),
    MatchNumeric(u32),
    Continue(Parser),
    Skip,
}

#[derive(Clone, Copy, Debug)]
enum ParserState {
    Unknown,
    // with sub: 0: 'o', 1: 'on'
    One,
    // 0: 't', 1: 'tw'
    Two,
    // 0: 'th', 1: 'thr', 2: 'thre'
    Three,
    // 0: 'f', 1: 'fo', 2: 'fou'
    Four,
    // 0: 'fi', 1: 'fiv'
    Five,
    // 0: 's', 1: 'si'
    Six,
    // 0: 'se', 1: 'sev', 2: 'seve'
    Seven,
    // 0: 'e', 1: 'ei', 2: 'eig', 3: 'eigh'
    Eight,
    // 0: 'n', 1: 'ni', 2: 'nin'
    Nine,
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (first, last) = l
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_numeric())
                .minmax_by_key(|(i, _)| *i)
                .into_option()
                .unwrap();
            first.1.to_digit(10).unwrap() * 10 + last.1.to_digit(10).unwrap()
        })
        .sum::<u32>()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let l = l.as_bytes();
            let mut first = None;
            let mut last = None;

            let mut i = 0;

            while i < l.len() {
                let (end, res) = Parser::parse_complete(&l[i..]);

                if let Some(res) = res {
                    if first.is_none() {
                        first = Some(res);
                    }
                    last = Some(res);
                }

                i += end;
            }
            first.unwrap() * 10 + last.unwrap()
        })
        .sum::<u32>()
}

impl Parser {
    pub fn new() -> Self {
        Self {
            state: ParserState::Unknown,
            sub: 0,
        }
    }

    pub fn parse_complete(st: &[u8]) -> (usize, Option<u32>) {
        let mut parser = Self::new();
        let mut consumed = 0;
        while consumed < st.len() {
            match parser.consume(st[consumed]) {
                // Alpha strings can overlap
                ParserResult::MatchAlpha(n) => return (consumed, Some(n)),
                // Numeric strings can't
                ParserResult::MatchNumeric(n) => return (consumed + 1, Some(n)),
                ParserResult::Continue(p) => parser = p,
                // We only skip on alpha strings that can overlap
                ParserResult::Skip => return (consumed, None),
            }
            consumed += 1;
        }
        (st.len(), None)
    }

    fn consume(self, ch: u8) -> ParserResult {
        use ParserState::*;

        let (next, len) = match (self.state, self.sub, char::from(ch)) {
            (_, _, ch) if ch.is_numeric() => {
                return ParserResult::MatchNumeric(ch.to_digit(10).unwrap())
            }

            (Unknown, _, 'o') => (One, 0),
            (Unknown, _, 't') => (Two, 0),
            (Unknown, _, 'f') => (Four, 0),
            (Unknown, _, 's') => (Six, 0),
            (Unknown, _, 'e') => (Eight, 0),
            (Unknown, _, 'n') => (Nine, 0),
            (Unknown, _, _) => (Unknown, 0),

            (One, 0, 'n') => (One, 1),
            (One, 1, 'e') => return ParserResult::MatchAlpha(1),
            (Two, 0, 'w') => (Two, 1),
            (Two, 1, 'o') => return ParserResult::MatchAlpha(2),
            (Two, 0, 'h') => (Three, 0),
            (Three, 0, 'r') => (Three, 1),
            (Three, 1, 'e') => (Three, 2),
            (Three, 2, 'e') => return ParserResult::MatchAlpha(3),
            (Four, 0, 'o') => (Four, 1),
            (Four, 1, 'u') => (Four, 2),
            (Four, 2, 'r') => return ParserResult::MatchAlpha(4),
            (Four, 0, 'i') => (Five, 0),
            (Five, 0, 'v') => (Five, 1),
            (Five, 1, 'e') => return ParserResult::MatchAlpha(5),
            (Six, 0, 'i') => (Six, 1),
            (Six, 1, 'x') => return ParserResult::MatchAlpha(6),
            (Six, 0, 'e') => (Seven, 0),
            (Seven, 0, 'v') => (Seven, 1),
            (Seven, 1, 'e') => (Seven, 2),
            (Seven, 2, 'n') => return ParserResult::MatchAlpha(7),
            (Eight, 1, 'g') => (Eight, 2),
            (Eight, 2, 'h') => (Eight, 3),
            (Eight, 3, 't') => return ParserResult::MatchAlpha(8),
            (Nine, 0, 'i') => (Nine, 1),
            (Nine, 1, 'n') => (Nine, 2),
            (Nine, 2, 'e') => return ParserResult::MatchAlpha(9),

            // When part of a number can be the start of another
            (_, _, 'i') if self.ends_with_e() => (Eight, 1),
            (_, _, 'i') if self.ends_with_n() => (Nine, 1),
            (Four, 1, 'n') /* o */ => (One, 1),

            (_, _, _) => return ParserResult::Skip,
        };

        ParserResult::Continue(Parser {
            state: next,
            sub: len,
        })
    }

    fn ends_with_e(&self) -> bool {
        use ParserState::*;

        // see above comments
        matches!(
            (self.state, self.sub),
            (Eight, 0) | (Seven, 0) | (Seven, 2) | (Three, 2)
        )
    }

    fn ends_with_n(&self) -> bool {
        use ParserState::*;

        // see above comments
        matches!((self.state, self.sub), (Nine, 0) | (Nine, 2) | (One, 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn part2_example() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn part2_edge() {
        assert_eq!(part2("xgkfonethreexnlcptbgxhnine4fivetwosix"), 16);
    }
}
