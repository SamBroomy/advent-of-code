use anyhow::Result;
use common::get_input;
use regex::Regex;

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(input)
        .map(|cap| {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            a * b
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
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

fn main() -> Result<()> {
    let input = get_input(3)?;
    let p1 = part_1(&input);
    println!("Part1: {}", p1);

    let p2 = part_2(&input);
    println!("Part2: {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "mul(3,2)malformedmul[3,2]mul(3,4)";
        let total = part_1(input);
        assert_eq!(total, (3 * 2) + (3 * 4));
    }

    #[test]
    fn part_1_example() {
        let input = "mul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let total = part_1(input);
        assert_eq!(total, 161);
    }
    #[test]
    fn day_3_part_1() {
        let input = get_input(3).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 157621318);
    }

    #[test]
    fn test_part_2_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let total = part_2(input);
        assert_eq!(total, 48);
    }

    #[test]
    fn day_3_part_2() {
        let input = get_input(3).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 79845780);
    }
}
