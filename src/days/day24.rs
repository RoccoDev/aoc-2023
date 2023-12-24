use std::ops::RangeInclusive;

use itertools::Itertools;
use num_bigint::BigInt;
use num_traits::{One, Zero};
use regex::Regex;

#[derive(Debug)]
pub struct Stone {
    pos: (f64, f64, f64),
    velocity: (f64, f64, f64),
}

#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Stone> {
    let regex = Regex::new(r#"(-?\d+)"#).unwrap();
    input
        .lines()
        .map(|l| {
            let (px, py, pz, vx, vy, vz) = regex
                .captures_iter(l)
                .map(|c| c[1].parse().unwrap())
                .collect_tuple()
                .unwrap();
            Stone {
                pos: (px, py, pz),
                velocity: (vx, vy, vz),
            }
        })
        .collect_vec()
}

#[aoc(day24, part1)]
pub fn part1(input: &[Stone]) -> usize {
    solve_p1(input, 200000000000000f64..=400000000000000f64)
}

#[aoc(day24, part2)]
pub fn part2(input: &[Stone]) -> i64 {
    let positions = input
        .iter()
        .map(|s| (s.pos.0 + s.pos.1 + s.pos.2) as i64)
        .collect_vec();
    let velocities = input
        .iter()
        .map(|s| (s.velocity.0 + s.velocity.1 + s.velocity.2) as i64)
        .collect_vec();
    let (vel_min, vel_max) = velocities.iter().minmax().into_option().unwrap();
    for residue in *vel_min..*vel_max {
        let mut congruences = positions
            .iter()
            .zip(velocities.iter())
            .filter_map(|(pos, vel)| {
                let modu = vel - residue;
                (modu != 0).then(|| (modu.abs(), pos.rem_euclid(modu)))
            })
            .collect_vec();

        let mut congruences_big = vec![];

        while let Some((mod1, res)) = congruences.pop() {
            congruences_big.push((BigInt::from(res), BigInt::from(mod1)));
            congruences.retain(|(mod2, _)| egcd_small(*mod2, mod1).0 == 1);
        }

        let Some(sol) = chinese_remainder(&congruences_big) else {
            continue;
        };

        let Ok(sol) = i64::try_from(sol) else {
            // discard obviously wrong solutions
            continue;
        };

        return sol;
    }
    0
}

fn solve_p1(input: &[Stone], range: RangeInclusive<f64>) -> usize {
    input
        .iter()
        .combinations(2)
        .filter_map(|comb| {
            let &[a, b] = comb.as_slice() else { panic!() };

            let slope_a = a.velocity.1 / a.velocity.0;
            let slope_b = b.velocity.1 / b.velocity.0;

            // y - y0 = slope (x - x0)
            // y = sx - sx0 + y0
            // y - sx = -sx0+y0
            // (x = (y+SXY)/s)

            // 1y - sAx = SXYA
            // 1y - sBx = SXYB

            let s_x0_y0_a = -slope_a * a.pos.0 + a.pos.1;
            let s_x0_y0_b = -slope_b * b.pos.0 + b.pos.1;

            let det_yn = (s_x0_y0_a * -slope_b) - (-slope_a * s_x0_y0_b);
            let det_yd = (-slope_b) - (-slope_a);
            let y = det_yn / det_yd;

            let inters = ((y - s_x0_y0_a) / slope_a, y);
            ((inters.0 - a.pos.0).signum() == a.velocity.0.signum()
                && (inters.0 - b.pos.0).signum() == b.velocity.0.signum()
                && (inters.1 - a.pos.1).signum() == a.velocity.1.signum()
                && (inters.1 - b.pos.1).signum() == b.velocity.1.signum())
            .then_some(inters)
        })
        .filter(|(x, y)| x.is_finite() && y.is_finite() && range.contains(&x) && range.contains(&y))
        .count()
}

fn egcd_small(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd_small(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn egcd_big(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    if a.is_zero() {
        (b.clone(), Zero::zero(), One::one())
    } else {
        let (g, x, y) = egcd_big(&(b % a), a);
        (g, y - (b / a) * (&x), x)
    }
}

fn mod_inv(x: &BigInt, n: &BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd_big(x, n);
    if g.is_one() {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(res_mod: &[(BigInt, BigInt)]) -> Option<BigInt> {
    let prod = res_mod.iter().map(|(_, m)| m).product::<BigInt>();

    let mut sum: BigInt = Zero::zero();

    for (residue, modulus) in res_mod {
        let p = (&prod) / modulus;
        sum += residue * mod_inv(&p, modulus)? * p
    }

    Some(sum % prod)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#;
        assert_eq!(solve_p1(&parse(&input), 7.0..=27.0), 2);
    }
}
