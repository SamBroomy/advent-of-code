use anyhow::Result;
use common::get_input;
use itertools::Itertools;

fn is_valid_line(line: &str) -> bool {
    line.split_whitespace()
        .filter_map(|x| x.parse::<i32>().ok())
        .tuple_windows()
        .all(|(a, b, c)| {
            let x = a - b;
            let y = b - c;
            (x.abs() <= 3 && y.abs() <= 3) && ((x > 0) && (y > 0) || (x < 0) && (y < 0))
        })
}
fn day_2() -> Result<i32> {
    let input = get_input(2)?;

    let count = input.lines().filter(|line| is_valid_line(line)).count();

    println!("{}", count);

    Ok(count as i32)
}

fn main() -> Result<()> {
    day_2().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let total = day_2().unwrap();
        assert_eq!(total, 402);
    }
}
