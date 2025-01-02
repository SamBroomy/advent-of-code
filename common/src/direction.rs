use crate::point::Point;
type P = crate::point::Point<usize>;

pub trait Direction {
    fn get_next_position(&self, point: P) -> Option<P>;
    fn iter() -> impl Iterator<Item = Self>;
    fn opposite(&self) -> Self;
}

#[derive(Debug, Clone, Copy)]
enum CardinalGrid {
    Up,
    Down,
    Left,
    Right,
}

impl Direction for CardinalGrid {
    fn get_next_position(&self, point: P) -> Option<P> {
        match self {
            CardinalGrid::Up => point.1.checked_sub(1).map(|y| Point(point.0, y)),
            CardinalGrid::Down => Some(Point(point.0, point.1 + 1)),
            CardinalGrid::Left => point.0.checked_sub(1).map(|x| Point(x, point.1)),
            CardinalGrid::Right => Some(Point(point.0 + 1, point.1)),
        }
    }

    fn iter() -> impl Iterator<Item = CardinalGrid> {
        const CARDINALGRID: [CardinalGrid; 4] = [
            CardinalGrid::Up,
            CardinalGrid::Down,
            CardinalGrid::Left,
            CardinalGrid::Right,
        ];
        CARDINALGRID.iter().copied()
    }

    fn opposite(&self) -> CardinalGrid {
        match self {
            CardinalGrid::Up => CardinalGrid::Down,
            CardinalGrid::Down => CardinalGrid::Up,
            CardinalGrid::Left => CardinalGrid::Right,
            CardinalGrid::Right => CardinalGrid::Left,
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
    fn get_next_position(&self, point: P) -> Option<P> {
        match self {
            AllDirections::Up => point.1.checked_sub(1).map(|y| Point(point.0, y)),
            AllDirections::Down => Some(Point(point.0, point.1 + 1)),
            AllDirections::Left => point.0.checked_sub(1).map(|x| Point(x, point.1)),
            AllDirections::Right => Some(Point(point.0 + 1, point.1)),
            AllDirections::UpRight => point
                .0
                .checked_sub(1)
                .zip(point.1.checked_sub(1))
                .map(|(x, y)| Point(x, y)),
            AllDirections::UpLeft => point
                .0
                .checked_sub(1)
                .zip(Some(point.1 + 1))
                .map(|(x, y)| Point(x, y)),
            AllDirections::DownRight => Some(Point(point.0 + 1, point.1 + 1)),
            AllDirections::DownLeft => point.0.checked_sub(1).map(|x| Point(x, point.1 + 1)),
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

    fn opposite(&self) -> Self {
        match self {
            AllDirections::Up => AllDirections::Down,
            AllDirections::Down => AllDirections::Up,
            AllDirections::Left => AllDirections::Right,
            AllDirections::Right => AllDirections::Left,
            AllDirections::UpRight => AllDirections::DownLeft,
            AllDirections::UpLeft => AllDirections::DownRight,
            AllDirections::DownRight => AllDirections::UpLeft,
            AllDirections::DownLeft => AllDirections::UpRight,
        }
    }
}
