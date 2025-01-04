use num_traits::{CheckedAdd, CheckedSub, NumCast, One, PrimInt, Signed, ToPrimitive, Zero};
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<P> {
    pub x: P,
    pub y: P,
}

impl<P> Point<P> {
    /// Create a new Point<P> with x and y coordinates.
    pub fn new(x: P, y: P) -> Self {
        Self { x, y }
    }
}

impl<P: Zero> Point<P> {
    pub fn zero() -> Self {
        Self::new(P::zero(), P::zero())
    }
}

impl<P: Copy + Add<Output = P>> Point<P> {
    pub fn add_x_y(&self) -> P {
        self.x + self.y
    }
}

// Add array-like access for x,y coordinates
impl<P> Index<usize> for Point<P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}

impl<P> IndexMut<usize> for Point<P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Coordinate {
    X,
    Y,
}

impl<P> Index<Coordinate> for Point<P> {
    type Output = P;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        match coord {
            Coordinate::X => &self.x,
            Coordinate::Y => &self.y,
        }
    }
}

impl<P> IndexMut<Coordinate> for Point<P> {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        match coord {
            Coordinate::X => &mut self.x,
            Coordinate::Y => &mut self.y,
        }
    }
}

// Numeric operations
impl<P: Add<Output = P>> Add for Point<P> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<P: AddAssign> AddAssign for Point<P> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<P: Sub<Output = P>> Sub for Point<P> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<P: SubAssign> SubAssign for Point<P> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<P: CheckedAdd> Point<P> {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self::new(
            self.x.checked_add(&other.x)?,
            self.y.checked_add(&other.y)?,
        ))
    }
}

impl<P: CheckedSub> Point<P> {
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self::new(
            self.x.checked_sub(&other.x)?,
            self.y.checked_sub(&other.y)?,
        ))
    }
}

impl<P: PrimInt> Point<P> {
    pub fn manhattan_distance(&self, other: &Self) -> P {
        (if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        }) + (if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        })
    }

    pub fn chebyshev_distance(&self, other: &Self) -> P {
        std::cmp::max(
            if self.x > other.x {
                self.x - other.x
            } else {
                other.x - self.x
            },
            if self.y > other.y {
                self.y - other.y
            } else {
                other.y - self.y
            },
        )
    }
}

impl<P: Signed + Copy> Point<P> {
    pub fn add_x(&self, x: P) -> Self {
        Self::new(self.x + x, self.y)
    }
    pub fn add_y(&self, y: P) -> Self {
        Self::new(self.x, self.y + y)
    }
    pub fn sub_x(&self, x: P) -> Self {
        Self::new(self.x - x, self.y)
    }
    pub fn sub_y(&self, y: P) -> Self {
        Self::new(self.x, self.y - y)
    }
}

impl<P: CheckedAdd + Copy> Point<P> {
    pub fn checked_add_x(&self, x: P) -> Option<Self> {
        Some(Self::new(self.x.checked_add(&x)?, self.y))
    }

    pub fn checked_add_y(&self, y: P) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_add(&y)?))
    }
}

impl<P: CheckedSub + Copy> Point<P> {
    pub fn checked_sub_x(&self, x: P) -> Option<Self> {
        Some(Self::new(self.x.checked_sub(&x)?, self.y))
    }

    pub fn checked_sub_y(&self, y: P) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_sub(&y)?))
    }
}

impl<P: CheckedAdd + CheckedSub + Copy + One> Point<P> {
    pub fn adjacent_cardinals(&self) -> [Option<Self>; 4] {
        [
            self.checked_sub_x(P::one()),
            self.checked_add_x(P::one()),
            self.checked_sub_y(P::one()),
            self.checked_add_y(P::one()),
        ]
    }
}

impl<P: CheckedAdd + CheckedSub + Copy + Zero + One + PartialOrd> Point<P> {
    pub fn bounded_cardinals(&self, bounds: impl Into<Self>) -> [Option<Self>; 4] {
        let bounds = bounds.into();
        [
            if self.x > P::zero() {
                self.checked_sub_x(P::one())
            } else {
                None
            },
            if self.x < bounds.x {
                self.checked_add_x(P::one())
            } else {
                None
            },
            if self.y > P::zero() {
                self.checked_sub_y(P::one())
            } else {
                None
            },
            if self.y < bounds.y {
                self.checked_add_y(P::one())
            } else {
                None
            },
        ]
    }
}

// Conversions
impl<P> From<(P, P)> for Point<P> {
    fn from((x, y): (P, P)) -> Self {
        Self::new(x, y)
    }
}
impl<P> From<Point<P>> for (P, P) {
    fn from(Point { x, y }: Point<P>) -> Self {
        (x, y)
    }
}

// Custom trait for point conversion
pub trait PointConversion<P> {
    type Error;
    fn try_convert(&self) -> std::result::Result<Point<P>, Self::Error>;
}

impl<P, U> PointConversion<U> for Point<P>
where
    P: ToPrimitive + Debug + Copy,
    U: NumCast,
{
    type Error = PointError;

    fn try_convert(&self) -> Result<Point<U>> {
        let x = NumCast::from(self.x).ok_or_else(|| {
            PointError::conversion(format!(
                "Cannot convert x coordinate {:?} to target type",
                self.x
            ))
        })?;

        let y = NumCast::from(self.y).ok_or_else(|| {
            PointError::conversion(format!(
                "Cannot convert y coordinate {:?} to target type",
                self.y
            ))
        })?;

        Ok(Point::new(x, y))
    }
}

// Display
impl<P: fmt::Display> fmt::Display for Point<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

use error::{PointError, Result};
mod error {
    use std::fmt;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PointError {
        #[error("Point conversion failed: {message}")]
        ConversionError { message: String },

        #[error("Point ({x}, {y}) is outside valid bounds {bounds}")]
        OutOfBounds {
            x: String,
            y: String,
            bounds: String,
        },

        #[error("Arithmetic error in {operation}: {message}")]
        ArithmeticError { operation: String, message: String },

        #[error("Invalid point operation: {message}")]
        InvalidOperation { message: String },
    }

    pub type Result<T> = std::result::Result<T, PointError>;

    impl PointError {
        pub fn conversion<M: fmt::Display>(message: M) -> Self {
            Self::ConversionError {
                message: message.to_string(),
            }
        }

        pub fn out_of_bounds<T: fmt::Display>(x: T, y: T, bounds: impl fmt::Display) -> Self {
            Self::OutOfBounds {
                x: x.to_string(),
                y: y.to_string(),
                bounds: bounds.to_string(),
            }
        }

        pub fn arithmetic<M: fmt::Display>(operation: &str, message: M) -> Self {
            Self::ArithmeticError {
                operation: operation.to_string(),
                message: message.to_string(),
            }
        }

        pub fn invalid_operation<M: fmt::Display>(message: M) -> Self {
            Self::InvalidOperation {
                message: message.to_string(),
            }
        }
    }

    impl From<std::num::TryFromIntError> for PointError {
        fn from(err: std::num::TryFromIntError) -> Self {
            Self::conversion(err.to_string())
        }
    }
}
