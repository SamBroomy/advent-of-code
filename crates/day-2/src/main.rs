use anyhow::Result;
use common::get_input;
use itertools::Itertools;

fn day_2() -> Result<i32> {
    let input = get_input(2)?;

    let count = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|x| x.parse::<i32>().ok())
                .tuple_windows::<(_, _, _)>()
                .map(|(a, b, c)| {
                    // The levels are either all increasing or all decreasing.
                    // Any two adjacent levels differ by at least one and at most three.
                    let x = a - b;
                    let y = b - c;

                    if (x.abs() > 3) || (y.abs() > 3) {
                        return false;
                    }

                    if ((x > 0) && (y > 0)) || ((x < 0) && (y < 0)) {
                        return true;
                    }
                    false
                })
                .all(|x| x)
        })
        .filter(|x| *x)
        .count();

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
