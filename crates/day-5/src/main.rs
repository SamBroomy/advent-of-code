use anyhow::Result;
use common::get_input;
use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let (page_ordering_rules, pages_to_produce) = input.split_once("\n\n").unwrap();
    let mut page_ordering_rules_hash_map = HashMap::new();
    page_ordering_rules
        .lines()
        .filter_map(|line| {
            line.split_once("|").and_then(|(lhs, rhs)| {
                let lhs = lhs.trim().parse::<i32>().ok()?;
                let rhs = rhs.trim().parse::<i32>().ok()?;
                Some((lhs, rhs))
            })
        })
        .for_each(|(l, r)| {
            page_ordering_rules_hash_map
                .entry(l)
                .and_modify(|e: &mut HashSet<i32>| {
                    e.insert(r);
                })
                .or_insert_with(|| {
                    let mut set = HashSet::new();
                    set.insert(r);
                    set
                });
        });

    let pages_to_produce_vec = pages_to_produce
        .lines()
        .filter_map(|line| {
            line.split_terminator(',')
                .map(|s| s.trim().parse::<i32>().ok())
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    (page_ordering_rules_hash_map, pages_to_produce_vec)
}

fn part_1(input: &str) -> i32 {
    let (page_ordering_rules, pages_to_produce) = parse_input(input);

    let mut count = 0;

    'outer: for pages in pages_to_produce.iter() {
        // Loop over in reverse order, don't need to check the last page since it has no preceding pages
        for i in (1..pages.len()).rev() {
            // if the page has no proceeding page rules, skip
            let Some(invalid_preceding_pages) = page_ordering_rules.get(&pages[i]) else {
                continue;
            };
            let preceding_pages = pages
                .get(0..i)
                .unwrap()
                .iter()
                .cloned()
                .collect::<HashSet<i32>>();
            // if the page has any invalid preceding pages, these pages are invalid
            if invalid_preceding_pages
                .intersection(&preceding_pages)
                .count()
                != 0
            {
                continue 'outer;
            }
        }
        // If we reach this point, all pages are valid so we can find the center page number and total it to the count
        count += pages[pages.len() / 2];
    }
    count
}

fn main() -> Result<()> {
    let input = get_input(5)?;
    let p1 = part_1(&input);
    println!("Part1: {}", p1);

    // let p2 = part_2(&input);
    // println!("Part2: {}", p2);

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
}
