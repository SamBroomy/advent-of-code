use anyhow::Result;
use common::get_input;

fn day_one() -> Result<i32> {
    let input = get_input(1)?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .filter_map(|line| {
            let line = line.split_whitespace().collect::<Vec<&str>>();

            let first = line.first()?.parse::<i32>().ok()?;
            let second = line.last()?.parse::<i32>().ok()?;
            Some((first, second))
        })
        .unzip();
    left.sort();
    right.sort();

    let total = left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (l, r)| acc + (l - r).abs());
    println!("{}", total);
    Ok(total)
}

fn main() -> Result<()> {
    day_one().map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let total = day_one().unwrap();
        assert_eq!(total, 1579939);
    }
}
