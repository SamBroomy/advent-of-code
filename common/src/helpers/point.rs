
use num_traits::{CheckedAdd, CheckedSub, NumCast, PrimInt, Signed, ToPrimitive, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    /// Create a new Point<T> with x and y coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Zero> Point<T> {
    pub fn zero() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Clone + Add<Output = T>> Point<T> {
    pub fn add_x_y(&self) -> T {
        self.x.clone() + self.y.clone()
    }
}

// Add array-like access for x,y coordinates
impl<T> Index<usize> for Point<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}

impl<T> IndexMut<usize> for Point<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}

// Numeric operations
impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl<T: SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl<T: PrimInt + CheckedAdd> Point<T> {
    pub fn checked_add(self, other: Self) -> Option<Self> {
        Some(Self::new(
            self.x.checked_add(&other.x)?,
            self.y.checked_add(&other.y)?,
        ))
    }
}

impl<T: PrimInt + CheckedSub> Point<T> {
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        Some(Self::new(
            self.x.checked_sub(&other.x)?,
            self.y.checked_sub(&other.y)?,
        ))
    }
}

impl<T: PrimInt> Point<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
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

    pub fn chebyshev_distance(&self, other: &Self) -> T {
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

impl<T: PrimInt + Signed> Point<T> {
    pub fn add_x(&self, x: T) -> Self {
        Self::new(self.x + x, self.y)
    }
    pub fn add_y(&self, y: T) -> Self {
        Self::new(self.x, self.y + y)
    }
    pub fn sub_x(&self, x: T) -> Self {
        Self::new(self.x - x, self.y)
    }
    pub fn sub_y(&self, y: T) -> Self {
        Self::new(self.x, self.y - y)
    }
}

impl<T: PrimInt + CheckedAdd> Point<T> {
    fn checked_add_x(&self, x: T) -> Option<Self> {
        Some(Self::new(self.x.checked_add(&x)?, self.y))
    }

    fn checked_add_y(&self, y: T) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_add(&y)?))
    }
}

impl<T: PrimInt + CheckedSub> Point<T> {
    pub fn checked_sub_x(&self, x: T) -> Option<Self> {
        Some(Self::new(self.x.checked_sub(&x)?, self.y))
    }

    pub fn checked_sub_y(&self, y: T) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_sub(&y)?))
    }
}

impl<T: PrimInt + CheckedAdd + CheckedSub> Point<T> {
    pub fn adjacent_cardinals(&self) -> [Option<Self>; 4] {
        [
            self.checked_sub_x(T::one()),
            self.checked_add_x(T::one()),
            self.checked_sub_y(T::one()),
            self.checked_add_y(T::one()),
        ]
    }

    pub fn bounded_cardinals(&self, bounds: impl Into<Self>) -> [Option<Self>; 4] {
        let bounds = bounds.into();
        [
            if self.x > T::zero() {
                self.checked_sub_x(T::one())
            } else {
                None
            },
            if self.x < bounds.x {
                self.checked_add_x(T::one())
            } else {
                None
            },
            if self.y > T::zero() {
                self.checked_sub_y(T::one())
            } else {
                None
            },
            if self.y < bounds.y {
                self.checked_add_y(T::one())
            } else {
                None
            },
        ]
    }
}

// Conversions
impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self::new(x, y)
    }
}
impl<T> From<Point<T>> for (T, T) {
    fn from(point: Point<T>) -> Self {
        (point.x, point.y)
    }
}

// Custom trait for point conversion
pub trait PointConversion<T> {
    type Error;
    fn try_convert(&self) -> std::result::Result<Point<T>, Self::Error>;
}

impl<T, U> PointConversion<U> for Point<T>
where
    T: PrimInt + ToPrimitive,
    U: PrimInt + NumCast,
{
    type Error = error::PointError;

    fn try_convert(&self) -> Result<Point<U>> {
        let x = NumCast::from(self.x)
            .ok_or_else(|| PointError::conversion_error("Failed to convert x coordinate"))?;
        let y = NumCast::from(self.y)
            .ok_or_else(|| PointError::conversion_error("Failed to convert y coordinate"))?;
        Ok(Point::new(x, y))
    }
}

// Display
impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

use error::{PointError, Result};
mod error {

    use std::fmt;

    pub type Result<T> = std::result::Result<T, PointError>;

    #[derive(Debug, Clone)]
    pub enum PointError {
        ConversionError(String),
        OutOfBounds { index: usize, max: usize },
    }

    impl PointError {
        pub fn conversion_error(msg: impl std::fmt::Display) -> Self {
            Self::ConversionError(msg.to_string())
        }
    }

    impl From<&str> for PointError {
        fn from(msg: &str) -> Self {
            Self::ConversionError(msg.to_string())
        }
    }

    impl fmt::Display for PointError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                PointError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
                PointError::OutOfBounds { index, max } => {
                    write!(f, "Index {} out of bounds (max: {})", index, max)
                }
            }
        }
    }

    impl std::error::Error for PointError {}
}
