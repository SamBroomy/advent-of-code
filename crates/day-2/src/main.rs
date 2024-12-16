use anyhow::Result;
use common::get_input;

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

fn part_1(input: &str) -> i32 {
    input.lines().filter(|line| is_valid_line_1(line)).count() as i32
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

fn part_2(input: &str) -> i32 {
    input.lines().filter(|line| is_valid_line_2(line)).count() as i32
}

fn main() -> Result<()> {
    let input = get_input(2)?;
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
    fn test_parse_line() {
        let line = "1 2 3 4";
        let parsed = parse_line(line);
        assert_eq!(parsed, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_line_increasing_or_decreasing() {
        let line = vec![1, 2, 3, 4];
        assert!(line_increasing_or_decreasing(&line));
        let line = vec![4, 3, 2, 1];
        assert!(line_increasing_or_decreasing(&line));
        let line = vec![1, 2, 1, 4];
        assert!(!line_increasing_or_decreasing(&line));
    }

    #[test]
    fn test_within_diff() {
        let line = vec![1, 2, 3, 4];
        assert!(within_diff(&line));
        let line = vec![1, 2, 3, 5];
        assert!(within_diff(&line));
        let line = vec![1, 2, 3, 7];
        assert!(!within_diff(&line));
        let line = vec![1, 7, 8, 10];
        assert!(!within_diff(&line));
    }

    #[test]
    fn test_is_valid_sequence() {
        let line = vec![1, 2, 3, 4];
        assert!(is_valid_sequence(&line));
        let line = vec![4, 3, 2, 1];
        assert!(is_valid_sequence(&line));
        let line = vec![1, 2, 1, 4];
        assert!(!is_valid_sequence(&line));
        let line = vec![1, 2, 3, 7];
        assert!(!is_valid_sequence(&line));
    }

    #[test]
    fn test_is_valid_line_1() {
        let line = "1 2 3 4";
        assert!(is_valid_line_1(line));
        let line = "4 3 2 1";
        assert!(is_valid_line_1(line));
        let line = "1 2 1 4";
        assert!(!is_valid_line_1(line));
        let line = "1 2 3 7";
        assert!(!is_valid_line_1(line));
    }

    #[test]
    fn test_is_valid_line_2() {
        let line = "1 2 3 4";
        assert!(is_valid_line_2(line));
        let line = "4 3 2 1";
        assert!(is_valid_line_2(line));
        let line = "1 2 1 4";
        assert!(is_valid_line_2(line));
        let line = "1 2 3 7";
        assert!(is_valid_line_2(line));
        let line = "1 7 8 10";
        assert!(is_valid_line_2(line));
        let line = "1 235, 3, 5";
        assert!(is_valid_line_2(line));
        let line = "1 235 3 5 2345 8";
        assert!(!is_valid_line_2(line));
    }

    #[test]
    fn test_part_1() {
        let input = "1 2 3 4\n4 3 2 1\n1 2 1 4\n1 2 3 7";
        let total = part_1(input);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_part_2() {
        let input = "1 2 3 4\n4 3 2 1\n1 2 1 4\n1 2 3 7\n1 7 8 10\n1 235, 3, 5\n1 235 3 5 2345 8";
        let total = part_2(input);
        assert_eq!(total, 6);
    }

    #[test]
    fn test_day_2_part_1() {
        let input = get_input(2).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 402);
    }

    #[test]
    fn test_day_2_part_2() {
        let input = get_input(2).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 455);
    }
}
