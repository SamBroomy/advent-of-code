fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .collect::<Vec<i32>>()
}

fn line_increasing_or_decreasing(line: &[i32]) -> bool {
    line.windows(2).all(|w| w[0] < w[1]) || line.windows(2).all(|w| w[0] > w[1])
}

fn within_diff(line: &[i32]) -> bool {
    line.windows(2).all(|w| (w[0] - w[1]).abs() <= 3)
}

fn is_valid_sequence(line: &[i32]) -> bool {
    line_increasing_or_decreasing(line) && within_diff(line)
}

fn is_valid_line_1(line: &str) -> bool {
    is_valid_sequence(&parse_line(line))
}

fn is_valid_line_2(line: &str) -> bool {
    let line = parse_line(line);
    if is_valid_sequence(&line) {
        return true;
    }
    for i in 0..line.len() {
        let mut line = line.clone();
        line.remove(i);
        if is_valid_sequence(&line) {
            return true;
        }
    }
    false
}

#[inline]
pub fn part1(input: &str) -> i32 {
    input.lines().filter(|line| is_valid_line_1(line)).count() as i32
}

#[inline]
pub fn part2(input: &str) -> i32 {
    input.lines().filter(|line| is_valid_line_2(line)).count() as i32
}

common::aoc_test!(2, 402, 4, 455);
