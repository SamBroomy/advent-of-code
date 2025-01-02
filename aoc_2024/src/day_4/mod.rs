trait Word {
    fn first() -> Self;
    fn match_char(&self, c: &char) -> bool;
    fn get_next(self) -> Option<Self>
    where
        Self: Sized;
}

trait Direction {
    fn get_next_position(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>);
    fn iter() -> impl Iterator<Item = Self>;
}

#[derive(Debug, Clone, Copy)]
enum Xmas {
    X,
    M,
    A,
    S,
}

impl Word for Xmas {
    fn first() -> Self {
        Xmas::X
    }
    fn match_char(&self, c: &char) -> bool {
        match self {
            Xmas::X => *c == 'X',
            Xmas::M => *c == 'M',
            Xmas::A => *c == 'A',
            Xmas::S => *c == 'S',
        }
    }

    fn get_next(self) -> Option<Self> {
        match self {
            Xmas::X => Some(Xmas::M),
            Xmas::M => Some(Xmas::A),
            Xmas::A => Some(Xmas::S),
            Xmas::S => None,
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

struct Position<W: Word, D: Direction> {
    word: W,
    direction: D,
}

impl<W: Word, D: Direction> Position<W, D> {
    pub fn new(word: W, direction: D) -> Self {
        Position { word, direction }
    }

    fn next_position(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
        self.direction.get_next_position(i, j)
    }

    fn next_word(mut self) -> Option<Self> {
        match self.word.get_next() {
            Some(next_word) => {
                self.word = next_word;
                Some(self)
            }
            None => None,
        }
    }

    fn match_next(self, outer_vec: &[Vec<char>], i: usize, j: usize) -> bool {
        let (Some(i), Some(j)) = self.next_position(i, j) else {
            return false;
        };
        let Some(c) = outer_vec.get(i).and_then(|v| v.get(j)) else {
            return false;
        };
        if !self.word.match_char(c) {
            return false;
        };
        match self.next_word() {
            Some(p) => p.match_next(outer_vec, i, j),
            None => true,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let grid = parse(input);
    let mut count = 0;

    let word = Xmas::first();
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !word.match_char(&grid[i][j]) {
                continue;
            }
            let next_word = word.get_next().unwrap();
            for direction in AllDirections::iter() {
                let position = Position::new(next_word, direction);
                if position.match_next(&grid, i, j) {
                    count += 1;
                }
            }
        }
    }

    count
}

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

#[inline]
pub fn part2(input: &str) -> i32 {
    let grid = parse(input);

    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 'A' {
                continue;
            }

            let mut words: Vec<[char; 2]> = Vec::with_capacity(2);
            for (d1, d2) in Diagonal::iter() {
                match (d1.get_next_position(i, j), d2.get_next_position(i, j)) {
                    ((Some(i1), Some(j1)), (Some(i2), Some(j2))) => {
                        match (
                            grid.get(i1).and_then(|v| v.get(j1)),
                            grid.get(i2).and_then(|v| v.get(j2)),
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
            if !words.is_empty() && words.iter().all(|w| matches!(w, ['M', 'S'] | ['S', 'M'])) {
                count += 1;
            }
        }
    }
    count
}

common::aoc_test!(18, 2718, 9, 2046);
