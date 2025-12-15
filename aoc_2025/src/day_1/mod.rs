fn parse(input: &str) -> std::str::Lines<'_> {
    input.lines()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    let lines = parse(input);
    for line in lines {
        let mut chars = line.chars();
        let rotation = chars.next().unwrap();

        let amount = chars.as_str().parse::<i32>().unwrap();

        if rotation == 'L' {
            pos -= amount;
        } else {
            pos += amount;
        }
        if pos % 100 == 0 {
            count += 1;
        }
    }
    count
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let mut pos: i32 = 50;
    let mut count = 0;
    let lines = parse(input);
    for line in lines {
        let mut chars = line.chars();
        let rotation = chars.next().unwrap();

        let amount = chars.as_str().parse::<i32>().unwrap();

        let dir = if rotation == 'L' { -1 } else { 1 };

        let start_mod = pos.rem_euclid(100);
        let mut first_t = (-dir * start_mod).rem_euclid(100);
        if first_t == 0 {
            first_t = 100;
        }
        if first_t <= amount {
            count += 1 + (amount - first_t) / 100;
        }

        pos += dir * amount;
    }
    count
}

common::aoc_test!(3, 1100, 6, 6358);
