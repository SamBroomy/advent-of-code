use crate::common::to_grid;
use std::char;

enum Diagonal {
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Diagonal {
    fn get_next_position(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
        match self {
            Diagonal::UpRight => (i.checked_sub(1), j.checked_add(1)),
            Diagonal::UpLeft => (i.checked_sub(1), j.checked_sub(1)),
            Diagonal::DownRight => (i.checked_add(1), j.checked_add(1)),
            Diagonal::DownLeft => (i.checked_add(1), j.checked_sub(1)),
        }
    }

    fn iter() -> impl Iterator<Item = (Self, Self)> {
        vec![
            (Diagonal::UpRight, Diagonal::DownLeft),
            (Diagonal::UpLeft, Diagonal::DownRight),
        ]
        .into_iter()
    }
}

pub fn part_2(input: &str) -> i32 {
    let lines_grid = to_grid(input);
    let mut total = 0;

    let start_char = 'A';
    for i in 0..lines_grid.len() {
        for j in 0..lines_grid[i].len() {
            if start_char != lines_grid[i][j] {
                continue;
            }

            let mut words: Vec<[char; 2]> = Vec::with_capacity(2);
            for (d1, d2) in Diagonal::iter() {
                match (d1.get_next_position(i, j), d2.get_next_position(i, j)) {
                    ((Some(i1), Some(j1)), (Some(i2), Some(j2))) => {
                        match (
                            lines_grid.get(i1).and_then(|v| v.get(j1)),
                            lines_grid.get(i2).and_then(|v| v.get(j2)),
                        ) {
                            (Some(c1), Some(c2)) => {
                                words.push([*c1, *c2]);
                            }
                            _ => break,
                        }
                    }
                    _ => break,
                }
            }
            if !words.is_empty()
                && words.iter().all(|w| match w {
                    ['M', 'S'] | ['S', 'M'] => true,
                    _ => false,
                })
            {
                total += 1;
            }
        }
    }
    total
}
