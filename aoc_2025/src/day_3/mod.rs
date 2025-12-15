fn parse(input: &str) -> Vec<Vec<u64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| u64::from(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let mut count = 0;
    for mut line in parse(input) {
        let mut tens = u64::MIN;
        let last = line.pop().unwrap();
        let mut units = last;
        for i in line.into_iter() {
            if i > tens {
                tens = i;
                units = last;
            } else if i > units {
                units = i;
            }
        }
        let c = (tens * 10) + units;
        count += c;
    }

    count
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let mut count = 0;

    for line in parse(input).into_iter() {
        let mut stack: Vec<u64> = Vec::with_capacity(12);

        let mut to_remove = line.len() - 12;
        for digit in line.into_iter() {
            while to_remove > 0 && !stack.is_empty() && *stack.last().unwrap() < digit {
                stack.pop();
                to_remove -= 1;
            }

            stack.push(digit);
        }
        stack.truncate(12);

        debug_assert!(stack.len() == 12);
        let joltage = stack.iter().fold(0, |acc, &digit| acc * 10 + digit);

        count += joltage;
    }

    count
}

common::aoc_test!(357, 17408, 3121910778619, 172740584266849);
