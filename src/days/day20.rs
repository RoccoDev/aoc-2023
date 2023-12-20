use std::collections::VecDeque;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

use super::day8::lcm;

#[derive(Debug, Clone, PartialEq)]
pub struct Sheet {
    modules: Vec<Module>,
    low_count: usize,
    high_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,
    ty: ModuleType,
    outputs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(FxHashMap<String, bool>),
    Broadcast,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> Sheet {
    let mut modules = input
        .lines()
        .map(|l| {
            let ty = match l.chars().next().unwrap() {
                '%' => ModuleType::FlipFlop(false),
                '&' => ModuleType::Conjunction(FxHashMap::default()),
                _ => ModuleType::Broadcast,
            };
            let (name, outputs) = l.split(" -> ").collect_tuple().unwrap();
            let name = if let ModuleType::Broadcast = ty {
                name.to_string()
            } else {
                name[1..].to_string()
            };
            let outputs = outputs.split(", ").map(|s| s.to_string()).collect_vec();

            (
                Module {
                    ty,
                    name,
                    outputs: vec![],
                },
                outputs,
            )
        })
        .collect_vec();
    let mut res: Vec<Module> = vec![];
    let mod_clone = modules.clone();
    for i in 0..modules.len() {
        let name = modules[i].0.name.clone();
        if let ModuleType::Conjunction(c) = &mut modules[i].0.ty {
            for module in &mod_clone {
                if module.1.contains(&name) {
                    c.insert(module.0.name.clone(), false);
                }
            }
        }
        let outputs = modules[i]
            .1
            .iter()
            .map(|s| {
                modules
                    .iter()
                    .position(|m| &m.0.name == s)
                    .unwrap_or(usize::MAX)
            })
            .collect_vec();
        let (module, _) = &mut modules[i];
        module.outputs = outputs;
        res.push(module.clone());
    }
    Sheet {
        modules: res,
        low_count: 0,
        high_count: 0,
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &Sheet) -> usize {
    let mut state = input.clone();
    let broadcaster = state
        .modules
        .iter()
        .position(|m| matches!(m.ty, ModuleType::Broadcast))
        .unwrap();
    let mut res = (0, 0);
    for _ in 0..1000 {
        let (nl, nh) = state.send(0, broadcaster, false);
        res.0 += nl;
        res.1 += nh;
    }
    res.0 * res.1
}

#[aoc(day20, part2)]
pub fn part2(input: &Sheet) -> usize {
    // Yet another LCM puzzle with assumptions over the input :)
    // I was testing with manual queries and LCM calc, but my first answer
    // was too high because I copied the wrong value, smh (4403 vs 4003)

    // Assume rx is the output of a & module, the high-intervals of each
    // dependency seem to have a fixed cycle.

    let first_dep = input
        .modules
        .iter()
        .position(|m| m.outputs.contains(&usize::MAX))
        .unwrap();
    let mut deps: FxHashSet<usize> = input
        .modules
        .iter()
        .enumerate()
        .filter(|(_, m)| m.outputs.contains(&first_dep))
        .map(|(i, _)| i)
        .collect();
    let broadcaster = input
        .modules
        .iter()
        .position(|m| matches!(m.ty, ModuleType::Broadcast))
        .unwrap();
    let mut cycles = vec![];
    let mut state = input.clone();

    let mut i = 1;
    while !deps.is_empty() {
        let found = state.send_2(0, broadcaster, false, first_dep, &deps);
        for dest in found {
            if deps.remove(&dest) {
                cycles.push(i);
            }
        }
        i += 1;
    }
    lcm(cycles)
}

impl Sheet {
    fn send(&mut self, sender_idx: usize, module_idx: usize, signal: bool) -> (usize, usize) {
        let mut handle_queue = VecDeque::new();
        handle_queue.push_back((sender_idx, module_idx, signal));
        let mut res = (0, 0);

        while let Some((sender, dest, signal)) = handle_queue.pop_back() {
            if signal {
                res.1 += 1;
            } else {
                res.0 += 1;
            }
            if dest == usize::MAX {
                continue;
            }
            let sender_name = self.modules[sender].name.clone();
            let dest_module = &mut self.modules[dest];
            let out = match &mut dest_module.ty {
                ModuleType::FlipFlop(old) => {
                    if !signal {
                        *old = !*old;
                        Some(*old)
                    } else {
                        None
                    }
                }
                ModuleType::Conjunction(map) => {
                    map.insert(sender_name, signal);
                    Some(map.values().any(|v| !*v))
                }
                ModuleType::Broadcast => Some(signal),
            };

            if let Some(out) = out {
                for module in &dest_module.outputs {
                    handle_queue.push_front((dest, *module, out));
                }
            }
        }

        res
    }

    fn send_2(
        &mut self,
        sender_idx: usize,
        module_idx: usize,
        signal: bool,
        filter_dest: usize,
        filter_high: &FxHashSet<usize>,
    ) -> FxHashSet<usize> {
        let mut handle_queue = VecDeque::new();
        handle_queue.push_back((sender_idx, module_idx, signal));

        let mut res = FxHashSet::default();

        while let Some((sender, dest, signal)) = handle_queue.pop_back() {
            if dest == usize::MAX {
                continue;
            }
            if signal && dest == filter_dest && filter_high.contains(&sender) {
                res.insert(sender);
            }
            let sender_name = self.modules[sender].name.clone();

            let dest_module = &mut self.modules[dest];
            let out = match &mut dest_module.ty {
                ModuleType::FlipFlop(old) => {
                    if !signal {
                        *old = !*old;
                        Some(*old)
                    } else {
                        None
                    }
                }
                ModuleType::Conjunction(map) => {
                    map.insert(sender_name.clone(), signal);
                    Some(map.values().any(|v| !*v))
                }
                ModuleType::Broadcast => Some(signal),
            };

            if let Some(out) = out {
                for module in &dest_module.outputs {
                    handle_queue.push_front((dest, *module, out));
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;
        assert_eq!(part1(&parse(&input)), 32000000);
    }
}
