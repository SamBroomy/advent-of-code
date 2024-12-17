use anyhow::Result;
use common::get_input;
use std::collections::HashSet;

fn parse_input(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let (page_ordering_rules, pages_to_produce) = input.split_once("\n\n").unwrap();
    let rules = page_ordering_rules
        .lines()
        .filter_map(|line| {
            line.split_once("|").and_then(|(lhs, rhs)| {
                let lhs = lhs.trim().parse::<i32>().ok()?;
                let rhs = rhs.trim().parse::<i32>().ok()?;
                Some((lhs, rhs))
            })
        })
        .collect::<HashSet<_>>();

    let pages_to_produce_vec = pages_to_produce
        .lines()
        .filter_map(|line| {
            line.split_terminator(',')
                .map(|s| s.trim().parse::<i32>().ok())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    (rules, pages_to_produce_vec)
}

fn sorted(x: &i32, y: &i32, rules: &HashSet<(i32, i32)>) -> bool {
    !rules.contains(&(*y, *x))
}

fn compare(x: &i32, y: &i32, rules: &HashSet<(i32, i32)>) -> std::cmp::Ordering {
    if rules.contains(&(*x, *y)) {
        return std::cmp::Ordering::Less;
    } else if rules.contains(&(*y, *x)) {
        return std::cmp::Ordering::Greater;
    } else {
        return std::cmp::Ordering::Equal;
    }
}

fn part_1(input: &str) -> i32 {
    let (rules, pages_to_produce) = parse_input(input);

    pages_to_produce
        .into_iter()
        .map(|pages| {
            if pages.is_sorted_by(|a, b| sorted(a, b, &rules)) {
                pages[pages.len() / 2]
            } else {
                0
            }
        })
        .sum()
}

fn part_2(input: &str) -> i32 {
    let (rules, pages_to_produce) = parse_input(input);

    pages_to_produce
        .into_iter()
        .filter(|pages| !pages.is_sorted_by(|a, b| sorted(a, b, &rules)))
        .map(|mut pages| {
            pages.sort_by(|a, b| compare(a, b, &rules));
            pages[pages.len() / 2]
        })
        .sum()
}

fn main() -> Result<()> {
    let input = get_input(5)?;
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
    fn day_5_part_1() {
        let input = get_input(5).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 7024);
    }

    #[test]
    fn day_5_part_2() {
        let input = get_input(5).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 4151);
    }
}
