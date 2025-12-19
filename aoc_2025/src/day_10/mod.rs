use core::fmt;
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::RwLock,
};

use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Light(u16);

impl Light {
    fn from_str(s: &str) -> Self {
        let bits = s.chars().enumerate().fold(0u16, |acc, (i, c)| match c {
            '#' => acc | (1 << i),
            '.' => acc,
            _ => panic!("unexpected char: {}", c),
        });
        Light(bits)
    }

    fn from_buttons(buttons: &[&Button]) -> Self {
        let bits = buttons.iter().fold(0u16, |acc, &n| acc ^ **n);
        Light(bits)
    }

    fn from_joltage(joltages: &[u16]) -> Self {
        let bits =
            joltages.iter().enumerate().fold(
                0u16,
                |acc, (i, &j)| {
                    if j % 2 == 1 { acc | (1 << i) } else { acc }
                },
            );
        Light(bits)
    }

    fn press_button(&mut self, button: &Button) {
        self.0 ^= **button;
    }

    fn press_buttons(mut self, buttons: &[Button]) -> Self {
        for &button in buttons {
            self.press_button(&button);
        }
        self
    }
}

impl fmt::Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in (0..16).rev() {
            let bit = (self.0 >> i) & 1;
            let c = if bit == 1 { '#' } else { '.' };
            write!(f, "{}", c)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Joltage(Vec<u16>);

impl Joltage {
    /// Assumes that the buttons can be subtracted from the joltage are buttons that solve the joltages light mask
    fn sub_buttons(self, buttons: &Buttons) -> Option<Self> {
        let Self(mut inner) = self;

        // subtract 1 from each joltage represented by the button. We should be left with an entirely even set of joltages
        // Once we have an even set we can then divide by 2 to get the next state
        for button in buttons.iter() {
            for (i, j) in inner.iter_mut().enumerate() {
                if ((**button >> i) & 1) == 1 {
                    if *j == 0 {
                        return None;
                    }
                    *j -= 1;
                }
            }
        }
        for j in inner.iter_mut() {
            if *j % 2 != 0 {
                return None;
            }
            *j /= 2;
        }

        Some(Joltage(inner))
    }

    fn is_solved(&self) -> bool {
        self.0.iter().all(|&j| j == 0)
    }
}

impl Deref for Joltage {
    type Target = Vec<u16>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Joltage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Button(u16);

impl Deref for Button {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        write!(f, "(")?;
        for i in 0..16 {
            if ((self.0 >> i) & 1) == 1 {
                if !first {
                    write!(f, ",")?;
                }
                write!(f, "{}", i)?;
                first = false;
            }
        }
        write!(f, ")")
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Buttons(Vec<Button>);

impl Deref for Buttons {
    type Target = Vec<Button>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Buttons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, button) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", button)?;
        }
        write!(f, "]")
    }
}

fn parse(input: &str) -> Vec<(Light, Buttons, Joltage)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();

            let lights_str = line.next().expect("should exist").trim_matches(['[', ']']);
            let lights = Light::from_str(lights_str);
            // reverse to get last part
            let mut line = line.rev();

            let joltage = line
                .next()
                .expect("should exist")
                .trim_matches(['{', '}'])
                .split(',')
                .map(|s| {
                    s.parse::<u16>()
                        .unwrap_or_else(|_| panic!("should parse: {}", s))
                })
                .collect::<Vec<u16>>();

            let buttons = line
                .map(|s| {
                    s.trim_matches(['(', ')'])
                        .split(',')
                        .map(|s| s.parse::<u8>().unwrap())
                        .fold(0u16, |acc, n| acc ^ (1 << n))
                })
                .map(Button)
                .collect();
            (lights, Buttons(buttons), Joltage(joltage))
        })
        .collect()
}

fn fewest_buttons(light: &Light, buttons: &[Button]) -> usize {
    buttons
        .iter()
        .powerset()
        // .par_bridge()
        .filter_map(|pressed: Vec<&Button>| {
            let on = Light::from_buttons(pressed.as_ref());
            if on == *light {
                Some(pressed.len())
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

#[inline]
pub fn part1(input: &str) -> u32 {
    let puzzles = parse(input);

    puzzles
        .par_iter()
        .map(|(lights, buttons, _)| fewest_buttons(lights, buttons) as u32)
        .sum()
}

fn valid_combination(light: &Light, buttons: &Buttons) -> Vec<Buttons> {
    buttons
        .iter()
        .powerset()
        .filter_map(|pressed| {
            let on = Light::from_buttons(pressed.as_ref());
            if on == *light {
                Some(Buttons(pressed.into_iter().copied().collect()))
            } else {
                None
            }
        })
        .collect()
}
// Recursive function that takes in a joltage and a set of buttons and returns the number of buttons pressed to solve the puzzle
fn search(
    joltage: Joltage,
    buttons: &Buttons,
    cache: &RwLock<HashMap<(Joltage, Buttons), u32>>,
) -> u32 {
    let cache_key = (joltage, buttons.clone());
    if let Some(&cached) = cache.read().unwrap().get(&cache_key) {
        return cached;
    }
    let lights = Light::from_joltage(&cache_key.0);

    let button_combinations = valid_combination(&lights, buttons);

    if button_combinations.is_empty() {
        {
            cache.write().unwrap().insert(cache_key, u32::MAX);
        }
        return u32::MAX;
    }

    let min_button_presses = button_combinations
        .iter()
        .map(|combo| {
            debug_assert_eq!(lights.press_buttons(combo), Light(0));
            if let Some(new_joltage) = cache_key.0.clone().sub_buttons(combo) {
                let button_presses = combo.len() as u32;
                if new_joltage.is_solved() {
                    button_presses
                } else {
                    search(new_joltage, buttons, cache)
                        .saturating_mul(2)
                        .saturating_add(button_presses)
                }
            } else {
                u32::MAX
            }
        })
        .min()
        .unwrap();
    {
        cache.write().unwrap().insert(cache_key, min_button_presses);
    }
    min_button_presses
}

#[inline]
pub fn part2(input: &str) -> u64 {
    // This solution uses this idea from [this reddit post](https://old.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/)

    let puzzles = parse(input).into_iter().collect::<Vec<_>>();

    let cache = RwLock::new(HashMap::new());
    puzzles
        .into_par_iter()
        .map(|(_, buttons, joltage)| search(joltage, &buttons, &cache) as u64)
        .sum()
}

common::aoc_test!(7, 455, 33, 16978);
