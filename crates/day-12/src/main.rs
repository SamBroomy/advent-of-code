use anyhow::Result;
use common::get_input;

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

fn part_1(input: &str) -> i32 {
    let grid = parse_input(input);
    todo!()
}

fn part_2(input: &str) -> i32 {
    todo!()
}
fn main() -> Result<()> {
    let input = get_input(12)?;

    let start = std::time::Instant::now();
    let p1 = part_1(&input);
    println!("Part 1: {} (took {:?})", p1, start.elapsed());

    let start = std::time::Instant::now();
    let p2 = part_2(&input);
    println!("Part 2: {} (took {:?})", p2, start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT_1: &str = "AAAA
BBCD
BBCC
EEEC";

    const EXAMPLE_INPUT_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_INPUT_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part_1_example_1() {
        let total = part_1(EXAMPLE_INPUT_1);
        assert_eq!(total, 140);
    }

    #[test]
    fn part_1_example_2() {
        let total = part_1(EXAMPLE_INPUT_2);
        assert_eq!(total, 772);
    }

    #[test]
    fn part_1_example_3() {
        let total = part_1(EXAMPLE_INPUT_3);
        assert_eq!(total, 1930);
    }
    #[test]
    fn test_part_1() {
        let input = get_input(12).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 228668);
    }

    #[test]
    fn part_2_example() {
        let total = part_2(EXAMPLE_INPUT_1);
        assert_eq!(total, 0);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(12).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 0);
    }
}
