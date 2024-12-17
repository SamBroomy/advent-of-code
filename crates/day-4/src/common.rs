pub trait Word {
    fn new() -> Self;
    fn match_char(&self, c: &char) -> bool;
    fn get_next(self) -> Option<Self>
    where
        Self: Sized;
}

pub trait Direction {
    fn get_next_position(&self, i: usize, j: usize) -> (Option<usize>, Option<usize>);
    fn iter() -> impl Iterator<Item = Self>;
}

pub struct Position<W: Word, D: Direction> {
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
}

pub fn to_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

pub fn match_next<W: Word, D: Direction>(
    outer_vec: &[Vec<char>],
    i: usize,
    j: usize,
    p: Position<W, D>,
) -> bool {
    // get next cords
    let (Some(i), Some(j)) = p.next_position(i, j) else {
        return false;
    };
    let Some(c) = outer_vec.get(i).and_then(|v| v.get(j)) else {
        return false;
    };
    if !p.word.match_char(c) {
        return false;
    };
    match p.next_word() {
        Some(p) => match_next(outer_vec, i, j, p),
        None => true,
    }
}
