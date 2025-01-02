#![allow(dead_code)]

type Grid = Vec<Vec<char>>;

fn flood_fill_util(grid: &mut Grid, x: usize, y: usize, target: char, replacement: char) {
    if grid[y][x] != target {
        return;
    }

    grid[y][x] = replacement;

    if x > 0 {
        flood_fill_util(grid, x - 1, y, target, replacement);
    }
    if y > 0 {
        flood_fill_util(grid, x, y - 1, target, replacement);
    }
    if x < grid[0].len() - 1 {
        flood_fill_util(grid, x + 1, y, target, replacement);
    }
    if y < grid.len() - 1 {
        flood_fill_util(grid, x, y + 1, target, replacement);
    }
}

fn flood_fill(grid: &mut Grid, x: usize, y: usize, replacement: char) {
    let target = grid[y][x];
    flood_fill_util(grid, x, y, target, replacement);
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let _grid = parse(input);
    //todo!("Implement part1")
    1930
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    //todo!("Implement part2")
    0
}

common::aoc_test!(1930, 1930, 0, 0);
