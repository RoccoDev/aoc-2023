use itertools::Itertools;

#[derive(Debug, Clone)]
pub struct Lens {
    label: String,
    op: Operation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Operation {
    Remove,
    Set(u32),
}

#[aoc_generator(day15, part2)]
fn parse(input: &str) -> Vec<Lens> {
    input
        .split(',')
        .map(|s| {
            let last = s.chars().last().unwrap();
            let op = if last == '-' {
                Operation::Remove
            } else {
                Operation::Set(last.to_digit(10).unwrap())
            };
            Lens {
                label: if op == Operation::Remove {
                    &s[..s.len() - 1]
                } else {
                    &s[..s.len() - 2]
                }
                .to_string(),
                op,
            }
        })
        .collect_vec()
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> u32 {
    input.split(',').map(hash).sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &[Lens]) -> usize {
    let mut boxes: [Vec<(&str, u32)>; 256] = vec![vec![]; 256].try_into().unwrap();
    for lens in input {
        let id = hash(&lens.label) as usize;
        match lens.op {
            Operation::Remove => boxes.iter_mut().for_each(|v| {
                if let Some(pos) = v.iter().position(|i| i.0 == lens.label) {
                    v.remove(pos);
                }
            }),
            Operation::Set(val) => {
                if let Some(pos) = boxes[id].iter().position(|i| i.0 == lens.label) {
                    boxes[id][pos] = (&lens.label, val);
                } else {
                    boxes[id].push((&lens.label, val));
                }
            }
        }
    }
    boxes
        .into_iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.into_iter()
                .enumerate()
                .map(move |(j, (_, v))| (i + 1) * (j + 1) * v as usize)
        })
        .sum()
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |mut val, c| {
        val += c as u8 as u32;
        val *= 17;
        val & 255
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn part2_example() {
        let input = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;
        assert_eq!(part2(&parse(input)), 145);
    }
}
