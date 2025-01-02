use ahash::AHashSet as HashSet;
// use std::collections::HashSet;

fn parse(input: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
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
        std::cmp::Ordering::Less
    } else if rules.contains(&(*y, *x)) {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Equal
    }
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let (rules, pages_to_produce) = parse(input);

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

#[inline]
pub fn part2(input: &str) -> i32 {
    let (rules, pages_to_produce) = parse(input);

    pages_to_produce
        .into_iter()
        .filter(|pages| !pages.is_sorted_by(|a, b| sorted(a, b, &rules)))
        .map(|mut pages| {
            pages.sort_by(|a, b| compare(a, b, &rules));
            pages[pages.len() / 2]
        })
        .sum()
}

common::aoc_test!(143, 7024, 123, 4151);
