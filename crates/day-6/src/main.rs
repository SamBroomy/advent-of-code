use std::char;

use anyhow::Result;
use common::get_input;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            _ => None,
        }
    }

    fn next(&mut self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_next_pos(&self, (i, j): (usize, usize)) -> (Option<usize>, Option<usize>) {
        match self {
            Direction::Up => (i.checked_sub(1), Some(j)),
            Direction::Down => (i.checked_add(1), Some(j)),
            Direction::Left => (Some(i), j.checked_sub(1)),
            Direction::Right => (Some(i), j.checked_add(1)),
        }
    }
}

struct Guard {
    pos: (usize, usize),
    dir: Direction,
    visited: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(pos: (usize, usize), dir: Direction) -> Self {
        Guard {
            pos,
            dir,
            visited: HashSet::new(),
        }
    }

    fn move_guard(&mut self, grid: &Vec<Vec<char>>) -> Option<()> {
        let (i, j) = self.pos;

        self.visited.insert((i, j));

        let (Some(next_i), Some(next_j)) = self.dir.get_next_pos((i, j)) else {
            return None;
        };
        // get next char
        let char = grid.get(next_i).and_then(|v| v.get(next_j))?;
        // check if it's a wall
        if *char == '#' {
            self.dir = self.dir.next();
        } else {
            self.pos = (next_i, next_j);
        }
        return self.move_guard(grid);
    }

    fn patrol(&mut self, grid: &Vec<Vec<char>>) -> i32 {
        self.move_guard(grid);
        self.visited.len() as i32
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<Guard> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if let Some(d) = Direction::from_char(grid[i][j]) {
                return Some(Guard::new((i, j), d));
            }
        }
    }
    None
}

fn part_1(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut guard = find_start(&grid).unwrap();

    guard.patrol(&grid)
}

fn main() -> Result<()> {
    let input = get_input(6)?;
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
    fn part_1_example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let total = part_1(input);
        assert_eq!(total, 41);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(6).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 5145);
    }
}
