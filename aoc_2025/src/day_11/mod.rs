use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (start, rest) = line.trim().split_once(':').unwrap();
            let rest = rest.split_whitespace().collect();
            (start, rest)
        })
        .collect()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let graph = parse(input);
    const START: &str = "you";
    const END: &str = "out";
    let mut completed_journeys = 0;

    let mut stack = vec![START];
    loop {
        let mut found_path = false;
        while let Some(node) = stack.pop() {
            if node == END {
                found_path = true;
                break;
            }

            if let Some(neighbors) = graph.get(node) {
                for &neighbour in neighbors {
                    stack.push(neighbour);
                }
            }
        }
        if found_path {
            completed_journeys += 1;
        } else {
            break;
        }
    }

    completed_journeys
}

#[inline]
pub fn part2(input: &str) -> i32 {
    todo!("Implement part2")
}

common::aoc_test!(5, 571, 1234, 1234);
