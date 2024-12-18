mod common;
mod part_1;
mod part_2;

use ::common::get_input;
use anyhow::Result;
use part_1::part_1;
use part_2::part_2;

fn main() -> Result<()> {
    let input = get_input(4)?;
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
    fn part_1_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let total = part_1(input);
        assert_eq!(total, 18);
    }

    #[test]
    fn day_4_part_1() {
        let input = get_input(4).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 2718);
    }

    #[test]
    fn part_2_example() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let total = part_2(input);
        assert_eq!(total, 9);
    }

    #[test]
    fn day_4_part_2() {
        let input = get_input(4).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 2046);
    }
}
