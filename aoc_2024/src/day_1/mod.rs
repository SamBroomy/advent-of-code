use ahash::AHashMap as HashMap;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .filter_map(|(left, right)| {
            let left = left.parse::<i32>().ok();
            let right = right.parse::<i32>().ok();
            left.zip(right)
        })
        .unzip()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort_unstable();
    right.sort_unstable();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);

    let mut counts = HashMap::with_capacity(right.len());
    right
        .iter()
        .for_each(|r| *counts.entry(*r).or_insert(0) += 1);
    left.iter()
        .map(|l| l * counts.get(l).copied().unwrap_or(0))
        .sum()
}

common::aoc_test!(11, 1579939, 31, 20351745);
