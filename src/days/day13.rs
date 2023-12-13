use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Pattern {
    rows: Vec<Vec<usize>>,
    columns: Vec<Vec<usize>>,
}

#[aoc_generator(day13)]
fn parse(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|block| {
            let mut rows = vec![];
            let mut columns = vec![vec![]; block.lines().next().unwrap().len()];

            for (y, line) in block.lines().enumerate() {
                let mut row = vec![];
                for (x, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        row.push(x);
                        columns[x].push(y);
                    }
                }
                rows.push(row);
            }

            Pattern { rows, columns }
        })
        .collect_vec()
}

#[aoc(day13, part1)]
pub fn part1(input: &[Pattern]) -> usize {
    input
        .iter()
        .map(|p| {
            find_reflection(&p.columns, None).unwrap_or_default()
                + 100 * find_reflection(&p.rows, None).unwrap_or_default()
        })
        .inspect(|p| assert_ne!(0, *p))
        .sum()
}

#[aoc(day13, part2)]
pub fn part2(input: &[Pattern]) -> usize {
    let mut new = input.to_vec();
    let mut sum = 0;
    'pat: for pat in &mut new {
        let orig_r = find_reflection(&pat.rows, None);
        let orig_c = find_reflection(&pat.columns, None);
        for y in 0..pat.rows.len() {
            for x in 0..pat.columns.len() {
                let old = pat.get(y, x);
                pat.set(y, x, !old);
                let new_r = find_reflection(&pat.rows, orig_r);
                let new_c = find_reflection(&pat.columns, orig_c);
                pat.set(y, x, old);
                if let Some(new_r) = new_r {
                    sum += 100 * new_r;
                    continue 'pat;
                }
                if let Some(new_c) = new_c {
                    sum += new_c;
                    continue 'pat;
                }
            }
        }
        unreachable!();
    }
    sum
}

fn find_reflection(lines: &[Vec<usize>], skip: Option<usize>) -> Option<usize> {
    let mut to_match = &lines[0..0];
    let mut matched = vec![];

    let mut match_idx: Option<usize> = None;
    let mut prev: Option<&Vec<usize>> = None;
    let mut match_i = 0;

    let mut i = 0;
    while i < lines.len() {
        let line = &lines[i];
        if let Some(idx) = match_idx {
            if &to_match[idx] != line {
                i = to_match.len();
                match_idx = None;
                prev = Some(&lines[i]);
            } else {
                matched.push(line);
                if idx == 0 {
                    break;
                }
                match_idx = Some(idx - 1);
            }
        } else {
            if prev.is_some_and(|l| l == line) {
                if skip.is_none() || skip.unwrap() != i {
                    match_i = i;
                    if to_match.len() < 1 {
                        return Some(1);
                    }
                    match_idx = Some(to_match.len() - 1);
                }
            }
            to_match = &lines[..i];
            prev = Some(line);
        }
        i += 1;
    }
    match_idx.map(|_| match_i)
}

impl Pattern {
    pub fn get(&self, row: usize, column: usize) -> bool {
        self.rows[row].binary_search(&column).is_ok()
    }

    pub fn set(&mut self, row: usize, column: usize, valid: bool) {
        let col_ref = &mut self.columns[column];
        let row_ref = &mut self.rows[row];
        if let Ok(pos) = col_ref.binary_search(&row) {
            col_ref.remove(pos);
        }
        if let Ok(pos) = row_ref.binary_search(&column) {
            row_ref.remove(pos);
        }
        if valid {
            row_ref.insert(row_ref.binary_search(&column).unwrap_err(), column);
            col_ref.insert(col_ref.binary_search(&row).unwrap_err(), row);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(part1(&parse(&input)), 405);
    }

    #[test]
    fn part2_example() {
        let input = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#;
        assert_eq!(part2(&parse(&input)), 400);
    }
}
