use std::char;

use anyhow::Result;
use common::get_input;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

enum Finish {
    Visited,
    OutOfBounds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    visited: HashSet<((usize, usize), Direction)>,
}

impl Guard {
    fn new(pos: (usize, usize), dir: Direction) -> Self {
        Guard {
            pos,
            dir,
            visited: HashSet::new(),
        }
    }

    fn new_visited(&mut self) -> bool {
        self.visited.insert((self.pos, self.dir))
    }

    fn patrol(&mut self, grid: &Vec<Vec<char>>) -> Finish {
        let (i, j) = self.pos;
        if !self.new_visited() {
            return Finish::Visited;
        }
        self.visited.insert((self.pos, self.dir));
        // check if we've visited this position before, if so infinite loop
        // get next position
        let (Some(next_i), Some(next_j)) = self.dir.get_next_pos((i, j)) else {
            return Finish::OutOfBounds;
        };

        let Some(char) = grid.get(next_i).and_then(|v| v.get(next_j)) else {
            return Finish::OutOfBounds;
        };
        // check if it's a wall
        if *char == '#' {
            self.dir = self.dir.next();
        } else {
            self.pos = (next_i, next_j);
        }
        self.patrol(grid)
    }

    fn total_visited(&self) -> i32 {
        self.visited
            .iter()
            .map(|(p, _)| p)
            .collect::<HashSet<_>>()
            .len() as i32
    }
}

fn find_start(grid: &[Vec<char>]) -> Option<Guard> {
    for (i, v) in grid.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if let Some(d) = Direction::from_char(*c) {
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

    guard.patrol(&grid);
    guard.total_visited()
}

fn part_2(input: &str) -> i32 {
    let grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let mut guard = find_start(&grid).unwrap();
    let start = guard.pos;
    guard.patrol(&grid);

    let patrol_path = guard.visited.iter().map(|(p, _)| p).collect::<HashSet<_>>();

    let (tx, rx) = mpsc::channel();

    for (i, j) in patrol_path {
        let mut new_grid = grid.clone();
        if new_grid[*i][*j] == '^' {
            continue;
        } else {
            new_grid[*i][*j] = '#';
        }
        let mut new_guard = Guard::new(start, Direction::Up);

        let tx = tx.clone();

        thread::spawn(move || {
            if matches!(new_guard.patrol(&new_grid), Finish::Visited) {
                tx.send(1).unwrap();
            }
        });
    }
    drop(tx);
    let mut total_loops_found = 0;
    while rx.recv().is_ok() {
        total_loops_found += 1;
    }
    total_loops_found
}

fn main() -> Result<()> {
    let input = get_input(6)?;
    let p1 = part_1(&input);
    println!("Part1: {}", p1);

    // Time how long it takes to run part 2

    let start = std::time::Instant::now();
    let p2 = part_2(&input);
    let duration = start.elapsed();
    println!("Part2: {} [{}]", p2, duration.as_secs_f32());

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

    #[test]
    fn part_2_example() {
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
        let total = part_2(input);
        assert_eq!(total, 6);
    }

    #[test]
    fn test_part_2() {
        let input = get_input(6).unwrap();
        let total = part_2(&input);
        assert_eq!(total, 1523);
    }
}
