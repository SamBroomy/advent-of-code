//Button A: X+94, Y+34
//Button B: X+22, Y+67
//Prize: X=8400, Y=5400

// Solves simultaneous equations of the form:
// a1, b1, c1, a2, b2, c2

// a1x + b1y = c1
// a2x + b2y = c2

// a2(a1x + b1y = c1)
// a1(a2x + b2y = c2)

// a1a2x + a1b2y = a1c2
// a1a2x + a2b1y = a2c1

// b_coefficient = a1b2y - a2b1y

// if abs(b_coefficient) < 1e-10 {
//     return None;
// }

// c_value = a1c2 - a2c1

// y = c_value.safe_div(b_coefficient)

// x = (c1 - b1y).safe_div(a1)

use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
struct Equation {
    a: i64,
    b: i64,
    c: i64,
}

impl Mul<i64> for Equation {
    type Output = Self;

    fn mul(self, rhs: i64) -> Self::Output {
        Equation {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}

impl Equation {
    fn new<T: Into<i64>>(a: T, b: T, c: T) -> Self {
        Equation {
            a: a.into(),
            b: b.into(),
            c: c.into(),
        }
    }

    fn into_part_2(self) -> Self {
        Equation {
            a: self.a,
            b: self.b,
            c: self.c + 10000000000000,
        }
    }

    fn solve(&self, other: &Self) -> Option<(f64, f64)> {
        let eq_1 = *self * other.a;
        let eq_2 = *other * self.a;

        let b_coefficient = (eq_1.b - eq_2.b) as f64;
        if b_coefficient.abs() < 1e-6 {
            return None;
        }

        let c_value = (eq_1.c - eq_2.c) as f64;
        let y = c_value / b_coefficient;
        if y.fract() != 0.0 {
            return None;
        }

        let x = (self.c as f64 - (self.b as f64 * y)) / (self.a as f64);
        if x.fract() != 0.0 {
            return None;
        }
        Some((x, y))
    }
}

#[derive(Debug)]
struct SimultaneousEquation {
    e1: Equation,
    e2: Equation,
}

impl SimultaneousEquation {
    fn into_part_2(self) -> Self {
        SimultaneousEquation {
            e1: self.e1.into_part_2(),
            e2: self.e2.into_part_2(),
        }
    }

    fn parse_input(input: &str) -> Vec<Self> {
        input.split("\n\n").map(Self::from_lines).collect()
    }

    fn from_lines(group: &str) -> Self {
        let get_numbers = |split: &str| {
            split
                .matches(char::is_numeric)
                .collect::<String>()
                .parse::<i64>()
                .ok()
        };
        let get_inputs = |line: &str| {
            line.split_whitespace()
                .filter_map(get_numbers)
                .collect::<Vec<_>>()
        };

        let mut lines = group.lines();
        let e1 = get_inputs(lines.next().unwrap());
        let e2 = get_inputs(lines.next().unwrap());
        let e3 = get_inputs(lines.next().unwrap());

        SimultaneousEquation {
            e1: Equation::new(e1[0], e2[0], e3[0]),
            e2: Equation::new(e1[1], e2[1], e3[1]),
        }
    }

    fn solve(&self) -> Option<(f64, f64)> {
        self.e1.solve(&self.e2)
    }

    fn cost(&self) -> Option<i64> {
        if let Some((a, b)) = self.solve() {
            Some(((a * 3.0) + b) as i64)
        } else {
            None
        }
    }
}
#[inline]
pub fn part1(input: &str) -> i64 {
    let equations = SimultaneousEquation::parse_input(input);

    equations
        .iter()
        .filter_map(SimultaneousEquation::cost)
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i64 {
    let equations = SimultaneousEquation::parse_input(input);
    equations
        .into_iter()
        .map(SimultaneousEquation::into_part_2)
        .filter_map(|sim: SimultaneousEquation| sim.cost())
        .sum()
}

common::aoc_test!(480, 31552, 875318608908, 95273925552482);
