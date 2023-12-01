use itertools::Itertools;

// We don't use replace over here
#[derive(Clone, Copy, Debug)]
struct Parser;

#[derive(Clone, Copy, Debug)]
struct ParserState {
    token: ParserToken,
    index: usize,
}

#[derive(Debug)]
enum ParserResult {
    MatchAlpha(u32),
    MatchNumeric(u32),
    Continue(ParserState),
    Skip,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
enum ParserToken {
    Unknown = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let (first, last) = l
                .chars()
                .enumerate()
                .filter(|(_, c)| c.is_ascii_digit())
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
    pub fn parse_complete(st: &[u8]) -> (usize, Option<u32>) {
        let mut state = ParserState::new(ParserToken::Unknown, 0);
        let mut consumed = 0;
        while consumed < st.len() {
            match Parser::consume(state, st[consumed]) {
                // Alpha strings can overlap
                ParserResult::MatchAlpha(n) => return (consumed, Some(n)),
                // Numeric strings can't
                ParserResult::MatchNumeric(n) => return (consumed + 1, Some(n)),
                ParserResult::Continue(p) => state = p,
                // We only skip on alpha strings that can overlap
                ParserResult::Skip => return (consumed, None),
            }
            consumed += 1;
        }
        (st.len(), None)
    }

    fn consume(mut state: ParserState, ch: u8) -> ParserResult {
        use ParserToken::*;

        let ch = char::from(ch);
        if ch.is_ascii_digit() {
            return ParserResult::MatchNumeric(ch.to_digit(10).unwrap());
        }

        if let Unknown = state.token {
            return ParserResult::Continue(ParserState::new(
                match ch {
                    'o' => One,
                    't' => Two,
                    'f' => Four,
                    's' => Six,
                    'e' => Eight,
                    'n' => Nine,
                    _ => Unknown,
                },
                1,
            ));
        }

        let current_char = state.current_char();

        if !state.next(ch) {
            // Branching states
            return ParserResult::Continue(match (current_char, ch) {
                // When part of a number can be the start of another
                ('e', 'i') => ParserState::new(Eight, 2),
                ('n', 'i') => ParserState::new(Nine, 2),
                ('o', 'n') => ParserState::new(One, 2),

                // Branch Seven, Five, Three (from Six, Four, Two)
                ('s', 'e') => ParserState::new(Seven, 2),
                ('f', 'i') => ParserState::new(Five, 2),
                ('t', 'h') => ParserState::new(Three, 2),
                _ => return ParserResult::Skip,
            });
        }

        if state.is_full_match() {
            return ParserResult::MatchAlpha(state.token.result());
        }

        ParserResult::Continue(state)
    }
}

impl ParserState {
    pub fn new(token: ParserToken, index: usize) -> Self {
        Self { token, index }
    }

    pub fn next(&mut self, next_char: char) -> bool {
        if char::from(self.token.pattern()[self.index]) != next_char {
            return false;
        }
        self.index += 1;
        true
    }

    pub fn is_full_match(&self) -> bool {
        self.index >= self.token.pattern().len()
    }

    fn current_char(&self) -> char {
        char::from(self.token.pattern()[self.index - 1])
    }
}

impl ParserToken {
    const fn result(self) -> u32 {
        self as u32
    }

    const fn pattern(self) -> &'static [u8] {
        use ParserToken::*;
        match self {
            Unknown => "",
            One => "one",
            Two => "two",
            Three => "three",
            Four => "four",
            Five => "five",
            Six => "six",
            Seven => "seven",
            Eight => "eight",
            Nine => "nine",
        }
        .as_bytes()
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
        assert_eq!(part2("sevenine"), 79);
        assert_eq!(part2("oneight"), 18);
        assert_eq!(part2("threeeeight"), 38);
        assert_eq!(part2("ninine2"), 92);
    }
}
