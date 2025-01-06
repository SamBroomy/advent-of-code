/// A direction in 2D space.
/// Responsibilities:
/// - Basic direction operations (opposite, next, previous, etc.)
/// - Directional movement
/// - Directional iteration
/// - Conversion to coordinate deltas
use super::point::Point;
use num_traits::{CheckedAdd, CheckedSub, One, Signed, Zero};
use std::fmt::{self, Debug, Display};
use std::slice::Iter;

pub trait DirectionBehaviour: Copy + Eq + Sized {
    const COUNT: usize;

    fn from_index(index: usize) -> Option<Self>;
    fn as_index(&self) -> usize;
    fn iter_all() -> Iter<'static, Self>;
    fn opposite(&self) -> Self;
    fn next(&self) -> Self;
    fn previous(&self) -> Self;
    fn as_delta<P>(&self) -> Point<P>
    where
        P: Signed;
    fn next_point<P>(&self, point: &Point<P>) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One;
    fn next_point_in_bounds<P>(&self, point: &Point<P>, bounds: &Point<P>) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One + PartialOrd + Zero + Display,
    {
        self.next_point(point)
            .and_then(|p| p.in_bounds(bounds).ok())
    }
    fn point_in_all_directions<P>(
        point: &Point<P>,
    ) -> impl Iterator<Item = (Self, Option<Point<P>>)>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
        Self: Sized + 'static,
    {
        Self::iter_all().map(move |&direction| (direction, direction.next_point(point)))
    }

    fn rotate_clockwise(&self) -> Self {
        self.next()
    }

    fn rotate_counter_clockwise(&self) -> Self {
        self.previous()
    }
}

pub trait RotationBehaviour: DirectionBehaviour {
    fn rotate(&self, degrees: i32) -> Self;

    fn rotate_90(&self) -> Self {
        self.rotate(90)
    }

    fn rotate_180(&self) -> Self {
        self.rotate(180)
    }

    fn rotate_270(&self) -> Self {
        self.rotate(270)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CardinalDirections {
    North,
    East,
    South,
    West,
}
impl CardinalDirections {
    pub const ALL: [Self; 4] = [
        CardinalDirections::North,
        CardinalDirections::East,
        CardinalDirections::South,
        CardinalDirections::West,
    ];
}
impl DirectionBehaviour for CardinalDirections {
    const COUNT: usize = 4;
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index % Self::COUNT).copied()
    }
    fn as_index(&self) -> usize {
        Self::ALL.iter().position(|&d| d == *self).unwrap()
    }

    fn iter_all() -> Iter<'static, Self> {
        Self::ALL.iter()
    }

    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn as_delta<P>(&self) -> Point<P>
    where
        P: Signed,
    {
        match self {
            Self::North => Point::new(-P::one(), P::zero()),
            Self::East => Point::new(P::zero(), P::one()),
            Self::South => Point::new(P::one(), P::zero()),
            Self::West => Point::new(P::zero(), -P::one()),
        }
    }
    fn next_point<P>(&self, point: &Point<P>) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
    {
        match self {
            Self::North => point.checked_sub_x(P::one()),
            Self::East => point.checked_add_y(P::one()),
            Self::South => point.checked_add_x(P::one()),
            Self::West => point.checked_sub_y(P::one()),
        }
    }
}
impl RotationBehaviour for CardinalDirections {
    fn rotate(&self, degrees: i32) -> Self {
        let index = self.as_index() as i32;
        let new_index = (index + degrees / 90).rem_euclid(Self::COUNT as i32) as usize;
        Self::from_index(new_index).unwrap()
    }
}

impl fmt::Display for CardinalDirections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            CardinalDirections::North => '↑',
            CardinalDirections::East => '→',
            CardinalDirections::South => '↓',
            CardinalDirections::West => '←',
        };
        write!(f, "{}", c)
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum OctalDirections {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}
impl OctalDirections {
    pub const ALL: [Self; 8] = [
        OctalDirections::North,
        OctalDirections::NorthEast,
        OctalDirections::East,
        OctalDirections::SouthEast,
        OctalDirections::South,
        OctalDirections::SouthWest,
        OctalDirections::West,
        OctalDirections::NorthWest,
    ];
}

impl DirectionBehaviour for OctalDirections {
    const COUNT: usize = 8;
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index % Self::COUNT).copied()
    }
    fn as_index(&self) -> usize {
        Self::ALL.iter().position(|&d| d == *self).unwrap()
    }
    fn iter_all() -> Iter<'static, Self> {
        Self::ALL.iter()
    }
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::North => Self::NorthEast,
            Self::NorthEast => Self::East,
            Self::East => Self::SouthEast,
            Self::SouthEast => Self::South,
            Self::South => Self::SouthWest,
            Self::SouthWest => Self::West,
            Self::West => Self::NorthWest,
            Self::NorthWest => Self::North,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Self::North => Self::NorthWest,
            Self::NorthWest => Self::West,
            Self::West => Self::SouthWest,
            Self::SouthWest => Self::South,
            Self::South => Self::SouthEast,
            Self::SouthEast => Self::East,
            Self::East => Self::NorthEast,
            Self::NorthEast => Self::North,
        }
    }

    fn as_delta<P>(&self) -> Point<P>
    where
        P: Signed,
    {
        match self {
            Self::North => Point::new(-P::one(), P::zero()),
            Self::NorthEast => Point::new(-P::one(), P::one()),
            Self::East => Point::new(P::zero(), P::one()),
            Self::SouthEast => Point::new(P::one(), P::one()),
            Self::South => Point::new(P::one(), P::zero()),
            Self::SouthWest => Point::new(P::one(), -P::one()),
            Self::West => Point::new(P::zero(), -P::one()),
            Self::NorthWest => Point::new(-P::one(), -P::one()),
        }
    }
    fn next_point<P>(&self, point: &Point<P>) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
    {
        match self {
            Self::North => point.checked_sub_x(P::one()),
            Self::NorthEast => point
                .checked_sub_x(P::one())
                .and_then(|p| p.checked_add_y(P::one())),
            Self::East => point.checked_add_y(P::one()),
            Self::SouthEast => point
                .checked_add_x(P::one())
                .and_then(|p| p.checked_add_y(P::one())),
            Self::South => point.checked_add_x(P::one()),
            Self::SouthWest => point
                .checked_add_x(P::one())
                .and_then(|p| p.checked_sub_y(P::one())),
            Self::West => point.checked_sub_y(P::one()),
            Self::NorthWest => point
                .checked_sub_x(P::one())
                .and_then(|p| p.checked_sub_y(P::one())),
        }
    }
}
impl RotationBehaviour for OctalDirections {
    fn rotate(&self, degrees: i32) -> Self {
        let index = self.as_index() as i32;
        let new_index = (index + degrees / 45).rem_euclid(Self::COUNT as i32) as usize;
        Self::from_index(new_index).unwrap()
    }
}

impl fmt::Display for OctalDirections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            OctalDirections::North => '↑',
            OctalDirections::East => '→',
            OctalDirections::South => '↓',
            OctalDirections::West => '←',
            OctalDirections::NorthEast => '↗',
            OctalDirections::SouthEast => '↘',
            OctalDirections::SouthWest => '↙',
            OctalDirections::NorthWest => '↖',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DiagonalDirections {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}
impl DiagonalDirections {
    pub const ALL: [Self; 4] = [
        DiagonalDirections::NorthEast,
        DiagonalDirections::SouthEast,
        DiagonalDirections::SouthWest,
        DiagonalDirections::NorthWest,
    ];
}
impl DirectionBehaviour for DiagonalDirections {
    const COUNT: usize = 4;
    fn from_index(index: usize) -> Option<Self> {
        Self::ALL.get(index % Self::COUNT).copied()
    }
    fn as_index(&self) -> usize {
        Self::ALL.iter().position(|&d| d == *self).unwrap()
    }
    fn iter_all() -> Iter<'static, Self> {
        Self::ALL.iter()
    }
    fn opposite(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthWest,
            Self::SouthEast => Self::NorthWest,
            Self::SouthWest => Self::NorthEast,
            Self::NorthWest => Self::SouthEast,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::NorthEast => Self::SouthEast,
            Self::SouthEast => Self::SouthWest,
            Self::SouthWest => Self::NorthWest,
            Self::NorthWest => Self::NorthEast,
        }
    }

    fn previous(&self) -> Self {
        match self {
            Self::NorthEast => Self::NorthWest,
            Self::NorthWest => Self::SouthWest,
            Self::SouthWest => Self::SouthEast,
            Self::SouthEast => Self::NorthEast,
        }
    }

    fn as_delta<P>(&self) -> Point<P>
    where
        P: Signed,
    {
        match self {
            Self::NorthEast => Point::new(-P::one(), P::one()),
            Self::SouthEast => Point::new(P::one(), P::one()),
            Self::SouthWest => Point::new(P::one(), -P::one()),
            Self::NorthWest => Point::new(-P::one(), -P::one()),
        }
    }
    fn next_point<P>(&self, point: &Point<P>) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
    {
        match self {
            Self::NorthEast => point
                .checked_sub_x(P::one())
                .and_then(|p| p.checked_add_y(P::one())),
            Self::SouthEast => point
                .checked_add_x(P::one())
                .and_then(|p| p.checked_add_y(P::one())),
            Self::SouthWest => point
                .checked_add_x(P::one())
                .and_then(|p| p.checked_sub_y(P::one())),
            Self::NorthWest => point
                .checked_sub_x(P::one())
                .and_then(|p| p.checked_sub_y(P::one())),
        }
    }
}
impl RotationBehaviour for DiagonalDirections {
    fn rotate(&self, degrees: i32) -> Self {
        let index = self.as_index() as i32;
        let new_index = (index + degrees / 45).rem_euclid(Self::COUNT as i32) as usize;
        Self::from_index(new_index).unwrap()
    }
}

impl fmt::Display for DiagonalDirections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            DiagonalDirections::NorthEast => '↗',
            DiagonalDirections::SouthEast => '↘',
            DiagonalDirections::SouthWest => '↙',
            DiagonalDirections::NorthWest => '↖',
        };
        write!(f, "{}", c)
    }
}

pub struct DirectionalMove<P, D: DirectionBehaviour> {
    pub point: Point<P>,
    pub direction: D,
    pub steps: usize,
}

impl<P, D: DirectionBehaviour> DirectionalMove<P, D> {
    pub fn new(point: Point<P>, direction: D) -> Self {
        Self {
            point,
            direction,
            steps: 1,
        }
    }
    pub fn with_steps(mut self, steps: usize) -> Self {
        self.steps = steps;
        self
    }
}
impl<P, D> From<(Point<P>, D)> for DirectionalMove<P, D>
where
    D: DirectionBehaviour,
{
    fn from((point, direction): (Point<P>, D)) -> Self {
        Self::new(point, direction)
    }
}
impl<P, D> From<(Point<P>, D, usize)> for DirectionalMove<P, D>
where
    D: DirectionBehaviour,
{
    fn from((point, direction, steps): (Point<P>, D, usize)) -> Self {
        Self {
            point,
            direction,
            steps,
        }
    }
}
impl<P, D> DirectionalMove<P, D>
where
    D: DirectionBehaviour,
    P: CheckedAdd + CheckedSub + Copy + One,
{
    pub fn get_next_point(&self) -> Option<Point<P>>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
    {
        (0..self.steps).try_fold(self.point, |current, _| self.direction.next_point(&current))
    }
    pub fn next_point(mut self) -> Option<Self>
    where
        P: CheckedAdd + CheckedSub + Copy + One,
    {
        self.point = self.get_next_point()?;
        Some(self)
    }
}
impl<P, D> DirectionalMove<P, D>
where
    D: DirectionBehaviour,
    P: CheckedAdd + CheckedSub + Copy + One + PartialOrd + Zero + Display,
{
    pub fn get_next_point_in_bounds(&self, bounds: &Point<P>) -> Option<Point<P>> {
        let next_point = self.get_next_point()?;
        next_point.in_bounds(bounds).ok()
    }
    pub fn next_point_in_bounds(mut self, bounds: &Point<P>) -> Option<Self> {
        self.point = self.get_next_point_in_bounds(bounds)?;
        Some(self)
    }
}
