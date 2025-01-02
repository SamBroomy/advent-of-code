use regex::Regex;

#[inline]
pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut capture = true;
    let mut total = 0;

    for cap in re.captures_iter(input) {
        if &cap[0] == "do()" {
            capture = true;
        } else if &cap[0] == "don't()" {
            capture = false;
        } else if capture {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            total += a * b;
        }
    }
    total
}

common::aoc_test!(161, 157621318, 48, 79845780);
