use anyhow::Result;
use common::get_input;
use std::collections::HashMap;

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
    if num_digits % 2 == 0 {
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

fn part_1(input: &str) -> u64 {
    part_n(input, 25)
}

fn part_2(input: &str) -> u64 {
    part_n(input, 75)
}
fn main() -> Result<()> {
    let input = get_input(11)?;

    let start = std::time::Instant::now();
    let p1 = part_1(&input);
    println!("Part 1: {} (took {:?})", p1, start.elapsed());

    let start = std::time::Instant::now();
    let p2 = part_2(&input);
    println!("Part 2: {} (took {:?})", p2, start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "125 17";
    #[test]
    fn part_1_example() {
        let total = part_1(EXAMPLE_INPUT);
        assert_eq!(total, 55312);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(11).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 228668);
    }

    #[test]
    fn part_2_example() {
        let total = part_2(EXAMPLE_INPUT);
        assert_eq!(total, 65601038650482);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(11).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 270673834779359);
    }
}
