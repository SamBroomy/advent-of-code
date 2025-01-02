use num_traits::{CheckedAdd, CheckedSub, PrimInt};
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Point<T>(pub T, pub T);

impl<T> Point<T> {
    /// Creates a new Point from x and y coordinates.
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

// Implement Add trait for Point<T>
impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl<T> AddAssign for Point<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}
impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl<T> SubAssign for Point<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

impl<T> Point<T>
where
    T: PrimInt + CheckedAdd + Copy,
{
    /// Performs a checked addition of two Points.
    /// Returns None if any addition overflows.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        let x = self.0.checked_add(&other.0)?;
        let y = self.1.checked_add(&other.1)?;
        Some(Self(x, y))
    }
}

impl<T> Point<T>
where
    T: PrimInt + CheckedSub + Copy,
{
    /// Performs a checked subtraction of two Points.
    /// Returns None if any subtraction overflows.
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let x = self.0.checked_sub(&other.0)?;
        let y = self.1.checked_sub(&other.1)?;
        Some(Self(x, y))
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self(x, y)
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(point: Point<T>) -> Self {
        (point.0, point.1)
    }
}
// Implement TryFrom for converting Point<i32> -> Point<usize>
impl TryFrom<Point<i32>> for Point<usize> {
    type Error = &'static str;

    fn try_from(value: Point<i32>) -> Result<Self, Self::Error> {
        if value.0 < 0 || value.1 < 0 {
            Err("Cannot convert negative coordinates to usize")
        } else {
            Ok(Self(value.0 as usize, value.1 as usize))
        }
    }
}

// Implement TryFrom for converting Point<usize> -> Point<i32>
impl TryFrom<Point<usize>> for Point<i32> {
    type Error = &'static str;

    fn try_from(value: Point<usize>) -> Result<Self, Self::Error> {
        let max_i32 = i32::MAX as usize;
        if value.0 > max_i32 || value.1 > max_i32 {
            Err("Coordinates exceed i32::MAX")
        } else {
            Ok(Self(value.0 as i32, value.1 as i32))
        }
    }
}
// Implement Display trait for Point<T>
impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
