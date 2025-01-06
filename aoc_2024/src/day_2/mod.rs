use rayon::prelude::*;

fn parse(input: &str) -> impl ParallelIterator<Item = Vec<i8>> + '_ {
    input
        .par_lines()
        .map(|line| line.split_whitespace())
        .map(|parts| { parts.flat_map(|part| part.parse::<i8>()) }.collect())
}

fn is_valid_sequence(line: &[i8]) -> bool {
    let mut windows = line.windows(2);
    (windows.all(|w| w[0] < w[1]) || windows.all(|w| w[0] > w[1]))
        && windows.all(|w| (w[0] - w[1]).abs() <= 3)
}

fn is_valid_line_2(line: &[i8]) -> bool {
    if is_valid_sequence(line) {
        return true;
    }
    (0..line.len()).any(|i| {
        let seq: Vec<_> = line[..i].iter().chain(line[i + 1..].iter()).collect();
        let mut w = seq.windows(2);
        w.all(|w| w[0] < w[1]) || w.all(|w| w[0] > w[1]) && w.all(|w| (w[0] - w[1]).abs() <= 3)
    })
}

#[inline]
pub fn part1(input: &str) -> usize {
    parse(input).filter(|line| is_valid_sequence(line)).count()
}

#[inline]
pub fn part2(input: &str) -> usize {
    parse(input).filter(|line| is_valid_line_2(line)).count()
}

common::aoc_test!(2, 402, 4, 455);
