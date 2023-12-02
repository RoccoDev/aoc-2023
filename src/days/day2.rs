use itertools::Itertools;

pub struct Game {
    id: i32,
    turns: Vec<Vec<Cube>>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Cube {
    ty: CubeType,
    amount: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CubeType {
    Red,
    Blue,
    Green,
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split(": ");
            let id = parts.next().unwrap();
            let turns = parts.next().unwrap();

            let id = id.strip_prefix("Game ").unwrap().parse().unwrap();

            let turns = turns
                .split("; ")
                .map(|turn| {
                    turn.split(", ")
                        .map(|cube| {
                            let mut parts = cube.split(" ");
                            let amount = parts.next().unwrap().parse().unwrap();
                            let ty = parts.next().unwrap();
                            let ty = match ty {
                                "red" => CubeType::Red,
                                "blue" => CubeType::Blue,
                                "green" => CubeType::Green,
                                t => panic!("invalid type {t}"),
                            };
                            Cube { ty, amount }
                        })
                        .collect()
                })
                .collect();

            Game { id, turns }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Game]) -> i32 {
    use CubeType::*;

    input
        .iter()
        .filter(|g| {
            g.turns.iter().all(|t| {
                !t.iter().any(|c| {
                    c.ty == Red && c.amount > 12
                        || c.ty == Green && c.amount > 13
                        || c.ty == Blue && c.amount > 14
                })
            })
        })
        .map(|g| g.id)
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|g| {
            let mut flattened = g.turns.iter().flatten().collect_vec();
            flattened.sort_unstable_by_key(|c| c.ty);
            flattened
                .into_iter()
                .group_by(|c| c.ty)
                .into_iter()
                .map(|(_, c)| c.map(|c| c.amount).max().unwrap())
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part1(&parse(&input)), 8);
    }

    #[test]
    fn part2_example() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(part2(&parse(&input)), 2286);
    }
}
