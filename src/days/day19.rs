use std::rc::Rc;

use fxhash::FxHashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Sheet {
    branches: FxHashMap<Rc<str>, Vec<Branch>>,
    vars: Vec<FxHashMap<Rc<str>, i32>>,
}

#[derive(Debug, Clone)]
enum Branch {
    If {
        var: Rc<str>,
        lt: bool,
        param: i32,
        dest: Rc<str>,
    },
    Else {
        dest: Rc<str>,
    },
}

// P2
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct VarBounds {
    var: Rc<str>,
    min: i32,
    max: i32,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> Sheet {
    let val_regex = Regex::new(r#"(\w)=(\d+)"#).unwrap();
    let branch_regex = Regex::new(r#"(\w+)([><])(\d+):(\w+)"#).unwrap();

    let (branches, vars) = input.split("\n\n").collect_tuple().unwrap();

    let branches = branches
        .lines()
        .map(|l| {
            let mut branches = vec![];
            let k = Rc::from(l.split('{').next().unwrap());
            let els = l.split(',').last().unwrap();
            let els = Rc::from(&els[..els.len() - 1]);

            for caps in branch_regex.captures_iter(l) {
                branches.push(Branch::If {
                    var: Rc::from(&caps[1]),
                    lt: &caps[2] == "<",
                    param: caps[3].parse().unwrap(),
                    dest: Rc::from(&caps[4]),
                })
            }

            branches.push(Branch::Else { dest: els });
            (k, branches)
        })
        .collect();

    let vars = vars
        .lines()
        .map(|l| {
            val_regex
                .captures_iter(l)
                .map(|caps| (Rc::from(&caps[1]), caps[2].parse().unwrap()))
                .collect()
        })
        .collect_vec();

    Sheet { branches, vars }
}

#[aoc(day19, part1)]
pub fn part1(input: &Sheet) -> i32 {
    input
        .vars
        .iter()
        .filter(|v| input.is_accepted(v))
        .map(|ps| ps.values().copied().sum::<i32>())
        .sum()
}

#[aoc(day19, part2)]
pub fn part2(input: &Sheet) -> usize {
    input
        .calc_combinations()
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|b| b.max as usize + 1 - b.min as usize)
                .product::<usize>()
        })
        .sum()
}

impl Sheet {
    pub fn is_accepted(&self, params: &FxHashMap<Rc<str>, i32>) -> bool {
        let mut key: Rc<str> = Rc::from("in");
        loop {
            key = self.branch(key.clone(), params);
            if key.as_ref() == "A" {
                return true;
            }
            if key.as_ref() == "R" {
                return false;
            }
        }
    }

    pub fn branch(&self, key: Rc<str>, params: &FxHashMap<Rc<str>, i32>) -> Rc<str> {
        let branches = &self.branches[&key];
        for branch in &branches[..branches.len() - 1] {
            let Branch::If {
                var,
                lt,
                param,
                dest,
            } = branch
            else {
                panic!()
            };
            if !match (params[var], lt) {
                (p, true) => p < *param,
                (p, false) => p > *param,
            } {
                continue;
            }
            return dest.clone();
        }
        let Branch::Else { dest } = &branches[branches.len() - 1] else {
            panic!()
        };
        dest.clone()
    }

    // Or<And>
    fn calc_combinations(&self) -> Vec<Vec<VarBounds>> {
        let mut all = vec![];
        let bounds = self.vars[0]
            .keys()
            .map(|k| VarBounds {
                var: k.clone(),
                min: 1,
                max: 4000,
            })
            .collect_vec();

        self.calc_bounds(Rc::from("in"), &bounds, &mut all);
        all
    }

    fn calc_bounds(&self, key: Rc<str>, bounds: &Vec<VarBounds>, total: &mut Vec<Vec<VarBounds>>) {
        if key.as_ref() == "A" {
            total.push(bounds.clone());
            return;
        }
        if key.as_ref() == "R" {
            return;
        }
        let branches = &self.branches[&key];
        let mut bounds = bounds.clone();
        for branch in &branches[..branches.len() - 1] {
            let Branch::If {
                var,
                lt,
                param,
                dest,
            } = branch
            else {
                panic!()
            };
            let mut child_bounds = bounds.clone();

            let bound_idx = child_bounds.iter_mut().position(|b| b.var == *var).unwrap();
            let bound = &mut child_bounds[bound_idx];
            if *lt {
                bound.max = (*param).min(bound.max) - 1;
                bounds[bound_idx].min = *param;
            } else {
                bound.min = (*param).max(bound.min) + 1;
                bounds[bound_idx].max = *param;
            }

            self.calc_bounds(dest.clone(), &child_bounds, total);
        }
        let Branch::Else { dest } = &branches[branches.len() - 1] else {
            panic!()
        };
        self.calc_bounds(dest.clone(), &bounds, total);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
        assert_eq!(part1(&parse(&input)), 19114);
    }

    #[test]
    fn part2_example() {
        let input = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;
        assert_eq!(part2(&parse(&input)), 167409079868000);
    }
}
