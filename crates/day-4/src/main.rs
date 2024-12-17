use std::char;

use anyhow::Result;
use common::get_input;

#[derive(Debug, Clone, Copy)]
enum Word {
    X,
    M,
    A,
    S,
}

impl Word {
    fn match_char(&self, c: &char) -> bool {
        match self {
            Word::X => *c == 'X',
            Word::M => *c == 'M',
            Word::A => *c == 'A',
            Word::S => *c == 'S',
        }
    }

    fn get_next(&self) -> Option<Word> {
        match self {
            Word::X => Some(Word::M),
            Word::M => Some(Word::A),
            Word::A => Some(Word::S),
            Word::S => None,
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    fn get_next(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
        match self {
            Direction::Up => (i.checked_sub(1), Some(j)),
            Direction::Down => (i.checked_add(1), Some(j)),
            Direction::Left => (Some(i), j.checked_sub(1)),
            Direction::Right => (Some(i), j.checked_add(1)),
            Direction::UpRight => (i.checked_sub(1), j.checked_add(1)),
            Direction::UpLeft => (i.checked_sub(1), j.checked_sub(1)),
            Direction::DownRight => (i.checked_add(1), j.checked_add(1)),
            Direction::DownLeft => (i.checked_add(1), j.checked_sub(1)),
        }
    }

    fn iter() -> impl Iterator<Item = Direction> {
        const DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownRight,
            Direction::DownLeft,
        ];
        DIRECTIONS.iter().copied()
    }
}

struct Position {
    word: Word,
    direction: Direction,
}

impl Position {
    fn next(mut self) -> Option<Self> {
        match self.word.get_next() {
            Some(next_word) => {
                self.word = next_word;
                Some(self)
            }
            None => None,
        }
    }
}

fn match_next(outer_vec: &[Vec<char>], i: usize, j: usize, p: Position) -> bool {
    // get next cords
    let (Some(i), Some(j)) = p.direction.get_next(i, j) else {
        return false;
    };
    let Some(inner_vec) = outer_vec.get(i) else {
        return false;
    };
    let Some(c) = inner_vec.get(j) else {
        return false;
    };
    if !p.word.match_char(c) {
        return false;
    };
    match p.next() {
        Some(p) => match_next(outer_vec, i, j, p),
        None => true,
    }
}

fn part_1(input: &str) -> i32 {
    let lines_vec = input
        .lines()
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;

    for i in 0..lines_vec.len() {
        for j in 0..lines_vec[i].len() {
            let word = Word::X;

            if !word.match_char(&lines_vec[i][j]) {
                continue;
            }
            let next_word = word.get_next().unwrap();
            for direction in Direction::iter() {
                let position = Position {
                    word: next_word,
                    direction,
                };
                if match_next(&lines_vec, i, j, position) {
                    total += 1;
                }
            }
        }
    }
    total
}

fn main() -> Result<()> {
    let input = get_input(4)?;
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
    fn day_4_part_1() {
        let input = get_input(4).unwrap();
        let total = part_1(&input);
        assert_eq!(total, 2718);
    }
}
