use anyhow::Result;
use common::get_input;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
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

fn part_1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs())
}

fn part_2(input: &str) -> i32 {
    let (left, right) = parse_input(input);

    left.iter()
        .map(|l| l * right.iter().filter(|r| l == *r).count() as i32)
        .fold(0, |acc, x| acc + x)
}

fn main() -> Result<()> {
    let input = get_input(1)?;

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
    fn test_parse_input() {
        let input = "1   2\n3   4\n5   6";
        let (left, right) = parse_input(input);
        assert_eq!(left, vec![1, 3, 5]);
        assert_eq!(right, vec![2, 4, 6]);
    }

    #[test]
    fn test_part_1_example() {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let total = part_1(input);
        assert_eq!(total, 11);
    }

    #[test]
    fn test_day_1_part_1() {
        let input = get_input(1).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 1579939);
    }
    
    #[test]
    fn test_part_2_example() {
        let input = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let total = part_2(input);
        assert_eq!(total, 31);
    }

    #[test]
    fn test_day_2_part_2() {
        let input = get_input(1).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 20351745);
    }
}
