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

fn completed_journeys<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    node: &'a str,
    end: &str,
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if node == end {
        return 1;
    }
    if !visited.insert(node) {
        return 0;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(node) {
        for &neighbour in neighbors {
            total += completed_journeys(graph, neighbour, end, visited);
        }
    }
    visited.remove(node);
    total
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let graph = parse(input);

    const START: &str = "you";
    const END: &str = "out";

    let mut visited = HashSet::new();
    completed_journeys(&graph, START, END, &mut visited)
}

const MUST_VISIT: [&str; 2] = ["fft", "dac"];
fn count_paths_with_must_visit<'a>(
    graph: &HashMap<&str, Vec<&'a str>>,
    node: &'a str,
    end: &str,
    visited: &mut HashSet<&'a str>,
    mask: u8,
    cache: &mut HashMap<(&'a str, u8), u64>,
) -> u64 {
    if node == end {
        return if mask == 0b11 { 1 } else { 0 };
    }

    if visited.contains(node) {
        return 0;
    }

    if let Some(&cached) = cache.get(&(node, mask)) {
        return cached;
    }

    visited.insert(node);

    let mut next_mask = mask;
    if let Some(idx) = MUST_VISIT.iter().position(|&n| n == node).map(|i| i as u8) {
        next_mask |= 1 << idx;
    }

    let mut total = 0;
    if let Some(neighbors) = graph.get(node) {
        for &neighbour in neighbors {
            total += count_paths_with_must_visit(graph, neighbour, end, visited, next_mask, cache);
        }
    }
    visited.remove(node);
    cache.insert((node, mask), total);
    total
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let graph = parse(input);

    const START: &str = "svr";
    const END: &str = "out";

    let mut visited = HashSet::new();

    count_paths_with_must_visit(&graph, START, END, &mut visited, 0, &mut HashMap::new())
}

common::aoc_test!(5, 571, 2, 511378159390560);
