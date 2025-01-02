fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter_map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();

            let first = line.first()?.parse::<i32>().ok()?;
            let second = line.last()?.parse::<i32>().ok()?;
            Some((first, second))
        })
        .unzip()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse(input);
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let (left, right) = parse(input);

    left.iter()
        .map(|l| l * right.iter().filter(|r| l == *r).count() as i32)
        .sum()
}

common::aoc_test!(11, 1579939, 31, 20351745);
