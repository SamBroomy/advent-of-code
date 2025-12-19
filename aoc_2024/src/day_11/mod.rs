//use std::collections::HashMap;
use ahash::AHashMap as HashMap;

fn parse_input(input: &str) -> HashMap<u128, u64> {
    let a = input.split_whitespace().map(|s| s.parse().unwrap());
    let mut map = HashMap::new();
    for i in a {
        *map.entry(i).or_insert(0) += 1;
    }
    map
}

fn first_rule(i: u128) -> Option<u128> {
    //If the stone is engraved with the number 0, it is replaced by a stone engraved with the number 1.
    if i == 0 {
        Some(1)
    } else {
        None
    }
}

fn digits(num: u128) -> u128 {
    num.to_string().len() as u128
}

fn second_rule(i: u128) -> Option<(u128, u128)> {
    // If the stone is engraved with a number that has an even number of digits, it is replaced by two stones. The left half of the digits are engraved on the new left stone, and the right half of the digits are engraved on the new right stone. (The new numbers don't keep extra leading zeroes: 1000 would become stones 10 and 0.)
    let num_digits = digits(i);
    if num_digits.is_multiple_of(2) {
        let half = num_digits / 2;
        let left = i / 10_u128.pow(half as u32);
        let right = i % 10_u128.pow(half as u32);
        Some((left, right))
    } else {
        None
    }
}

fn third_rule(i: u128) -> u128 {
    // If none of the other rules apply, the stone is replaced by a new stone; the old stone's number multiplied by 2024 is engraved on the new stone.
    i * 2024
}

fn ruleset(i: u128) -> (u128, Option<u128>) {
    if let Some(i) = first_rule(i) {
        (i, None)
    } else if let Some((left, right)) = second_rule(i) {
        (left, Some(right))
    } else {
        (third_rule(i), None)
    }
}

fn blink(mut input: HashMap<u128, u64>) -> HashMap<u128, u64> {
    let changes = input
        .drain()
        .map(|(k, v)| {
            let (left, right) = ruleset(k);
            (left, right, v)
        })
        .collect::<Vec<_>>();

    for (left, right, v) in changes {
        *input.entry(left).or_insert(0) += v;
        if let Some(right) = right {
            *input.entry(right).or_insert(0) += v;
        }
    }
    input
}

fn part_n(input: &str, n: usize) -> u64 {
    let mut input = parse_input(input);

    for _ in 0..n {
        input = blink(input);
    }

    input.values().sum::<u64>()
}
#[inline]
pub fn part1(input: &str) -> u64 {
    part_n(input, 25)
}

#[inline]
pub fn part2(input: &str) -> u64 {
    part_n(input, 75)
}

common::aoc_test!(55312, 228668, 65601038650482, 270673834779359);
