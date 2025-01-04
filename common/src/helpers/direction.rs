use num_traits::PrimInt;
use std::fmt;
use std::ops::Neg;

use super::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

impl Direction {
    pub const CARDINALS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
        Direction::NorthEast,
        Direction::SouthEast,
        Direction::SouthWest,
        Direction::NorthWest,
    ];

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
            Direction::NorthWest => Direction::SouthEast,
        }
    }

    pub fn rotate_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::NorthEast => Direction::SouthEast,
            Direction::SouthEast => Direction::SouthWest,
            Direction::SouthWest => Direction::NorthWest,
            Direction::NorthWest => Direction::NorthEast,
        }
    }

    pub fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::NorthEast,
            Direction::NorthEast => Direction::East,
            Direction::East => Direction::SouthEast,
            Direction::SouthEast => Direction::South,
            Direction::South => Direction::SouthWest,
            Direction::SouthWest => Direction::West,
            Direction::West => Direction::NorthWest,
            Direction::NorthWest => Direction::North,
        }
    }

    pub fn previous(&self) -> Direction {
        match self {
            Direction::North => Direction::NorthWest,
            Direction::NorthWest => Direction::West,
            Direction::West => Direction::SouthWest,
            Direction::SouthWest => Direction::South,
            Direction::South => Direction::SouthEast,
            Direction::SouthEast => Direction::East,
            Direction::East => Direction::NorthEast,
            Direction::NorthEast => Direction::North,
        }
    }

    pub fn rotate_anti_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::NorthEast => Direction::NorthWest,
            Direction::SouthEast => Direction::NorthEast,
            Direction::SouthWest => Direction::SouthEast,
            Direction::NorthWest => Direction::SouthWest,
        }
    }

    pub fn as_point<T: PrimInt + Neg<Output = T>>(&self) -> Point<T> {
        let one = T::one();
        let neg_one = one.neg();
        let zero = T::zero();

        match self {
            Direction::North => Point::new(neg_one, zero),
            Direction::East => Point::new(zero, one),
            Direction::South => Point::new(one, zero),
            Direction::West => Point::new(zero, neg_one),
            Direction::NorthEast => Point::new(neg_one, one),
            Direction::SouthEast => Point::new(one, one),
            Direction::SouthWest => Point::new(one, neg_one),
            Direction::NorthWest => Point::new(neg_one, neg_one),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Direction::North => '↑',
            Direction::East => '→',
            Direction::South => '↓',
            Direction::West => '←',
            Direction::NorthEast => '↗',
            Direction::SouthEast => '↘',
            Direction::SouthWest => '↙',
            Direction::NorthWest => '↖',
        };
        write!(f, "{}", c)
    }
}
