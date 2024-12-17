use std::char;

use crate::common::{match_next, to_grid, Direction, Position, Word};

#[derive(Debug, Clone, Copy)]
enum XMAS {
    X,
    M,
    A,
    S,
}

impl Word for XMAS {
    fn new() -> Self {
        XMAS::X
    }

    fn match_char(&self, c: &char) -> bool {
        match self {
            XMAS::X => *c == 'X',
            XMAS::M => *c == 'M',
            XMAS::A => *c == 'A',
            XMAS::S => *c == 'S',
        }
    }

    fn get_next(self) -> Option<Self> {
        match self {
            XMAS::X => Some(XMAS::M),
            XMAS::M => Some(XMAS::A),
            XMAS::A => Some(XMAS::S),
            XMAS::S => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum AllDirections {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction for AllDirections {
    fn get_next_position(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
        match self {
            AllDirections::Up => (i.checked_sub(1), Some(j)),
            AllDirections::Down => (i.checked_add(1), Some(j)),
            AllDirections::Left => (Some(i), j.checked_sub(1)),
            AllDirections::Right => (Some(i), j.checked_add(1)),
            AllDirections::UpRight => (i.checked_sub(1), j.checked_add(1)),
            AllDirections::UpLeft => (i.checked_sub(1), j.checked_sub(1)),
            AllDirections::DownRight => (i.checked_add(1), j.checked_add(1)),
            AllDirections::DownLeft => (i.checked_add(1), j.checked_sub(1)),
        }
    }

    fn iter() -> impl Iterator<Item = AllDirections> {
        const ALLDIRECTIONS: [AllDirections; 8] = [
            AllDirections::Up,
            AllDirections::Down,
            AllDirections::Left,
            AllDirections::Right,
            AllDirections::UpRight,
            AllDirections::UpLeft,
            AllDirections::DownRight,
            AllDirections::DownLeft,
        ];
        ALLDIRECTIONS.iter().copied()
    }
}

pub fn part_1(input: &str) -> i32 {
    let lines_grid = to_grid(input);
    let mut total = 0;

    for i in 0..lines_grid.len() {
        for j in 0..lines_grid[i].len() {
            let word = XMAS::new();
            if !word.match_char(&lines_grid[i][j]) {
                continue;
            }
            let next_word = word.get_next().unwrap();
            for direction in AllDirections::iter() {
                let position = Position::new(next_word, direction);
                if match_next(&lines_grid, i, j, position) {
                    total += 1;
                }
            }
        }
    }
    total
}
