/// A 2D point with x and y coordinates.
/// Responsibilities:
/// - Basic coordinate operations and conversions (add, subtraact, bounds checking, etc.)
/// - Coordinate system conversions
/// - Grid index conversion
/// - Distance calculations
use num_traits::{CheckedAdd, CheckedSub, NumCast, One, PrimInt, Signed, ToPrimitive, Zero};
use std::fmt::{self, Debug, Display};
use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Rem, Sub, SubAssign};

mod error;
use error::{PointError, Result};

use super::direction::DirectionBehaviour;

/// A 2D point with x and y coordinates.
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
    /// Maps both coordinates using the provided function
    pub fn map<U, F: FnMut(P) -> U>(self, mut f: F) -> Point<U> {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<P> Default for Point<P>
where
    P: Default,
{
    /// Create a new Point<P> with x and y coordinates set to the default value of P.
    fn default() -> Self {
        Self::new(P::default(), P::default())
    }
}

impl<P: Zero> Point<P> {
    /// Create a new Point<P> with x and y coordinates set to zero (0,0).
    pub fn zero() -> Self {
        Self::new(P::zero(), P::zero())
    }
    /// Check if the point is at the origin (0,0).
    pub fn is_origin(&self) -> bool
    where
        P: PartialEq,
    {
        self.x == P::zero() && self.y == P::zero()
    }
}

// Add array-like access for x,y coordinates
impl<P> Index<usize> for Point<P> {
    type Output = P;
    /// Access the x coordinate with index 0 and the y coordinate with index 1.
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}

impl<P> IndexMut<usize> for Point<P> {
    /// Access the x coordinate with index 0 and the y coordinate with index 1.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Point index out of bounds"),
        }
    }
}
/// A coordinate axis.
#[derive(Debug, Clone, Copy)]
pub enum Coordinate {
    X,
    Y,
}

impl<P> Index<Coordinate> for Point<P> {
    type Output = P;
    /// Access the x coordinate with Coordinate::X and the y coordinate with Coordinate::Y.
    fn index(&self, coord: Coordinate) -> &Self::Output {
        match coord {
            Coordinate::X => &self.x,
            Coordinate::Y => &self.y,
        }
    }
}

impl<P> IndexMut<Coordinate> for Point<P> {
    /// Access the x coordinate with Coordinate::X and the y coordinate with Coordinate::Y.
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
    /// Add two points together.
    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}
impl<P: AddAssign> AddAssign for Point<P> {
    /// Add two points together and assign the result to the first point.
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl<P: Add<Output = P> + Copy> Point<P> {
    /// Add a scalar to the x coordinate of the point.
    pub fn add_x(&self, x: P) -> Self {
        Self::new(self.x + x, self.y)
    }
    /// Add a scalar to the y coordinate of the point.
    pub fn add_y(&self, y: P) -> Self {
        Self::new(self.x, self.y + y)
    }
}
impl<P: Sub<Output = P>> Sub for Point<P> {
    type Output = Self;
    /// Subtract one point from another.
    fn sub(self, other: Self) -> Self::Output {
        Self::new(self.x - other.x, self.y - other.y)
    }
}
impl<P: SubAssign> SubAssign for Point<P> {
    /// Subtract one point from another and assign the result to the first point.
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}
impl<P: Sub<Output = P> + Copy> Point<P> {
    /// Subtract a scalar from the x coordinate of the point.
    pub fn sub_x(&self, x: P) -> Self {
        Self::new(self.x - x, self.y)
    }
    /// Subtract a scalar from the y coordinate of the point.
    pub fn sub_y(&self, y: P) -> Self {
        Self::new(self.x, self.y - y)
    }
}
impl<P: CheckedAdd> Point<P> {
    /// Add two points together and return the result if the operation is successful.
    pub fn checked_add(self, Self { x, y }: Self) -> Option<Self> {
        Some(Self::new(self.x.checked_add(&x)?, self.y.checked_add(&y)?))
    }
}
impl<P: CheckedSub> Point<P> {
    /// Subtract one point from another and return the result if the operation is successful.
    pub fn checked_sub(self, Self { x, y }: Self) -> Option<Self> {
        Some(Self::new(self.x.checked_sub(&x)?, self.y.checked_sub(&y)?))
    }
}
impl<P: CheckedAdd + Copy> Point<P> {
    /// Add a scalar to the x coordinate of the point and return the result if the operation is successful.
    pub fn checked_add_x(&self, x: P) -> Option<Self> {
        Some(Self::new(self.x.checked_add(&x)?, self.y))
    }
    /// Add a scalar to the y coordinate of the point and return the result if the operation is successful.
    pub fn checked_add_y(&self, y: P) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_add(&y)?))
    }
}
impl<P: CheckedSub + Copy> Point<P> {
    /// Subtract a scalar from the x coordinate of the point and return the result if the operation is successful.
    pub fn checked_sub_x(&self, x: P) -> Option<Self> {
        Some(Self::new(self.x.checked_sub(&x)?, self.y))
    }
    /// Subtract a scalar from the y coordinate of the point and return the result if the operation is successful.
    pub fn checked_sub_y(&self, y: P) -> Option<Self> {
        Some(Self::new(self.x, self.y.checked_sub(&y)?))
    }
}
// // Generic access methods
// impl<T> Grid<T> {
//     ///
//     pub fn point_to_index<P>(&self, point: Point<P>) -> Result<usize>
//     where
//         P: Zero + PartialOrd + Display + Clone + ToPrimitive,
//     {
//         let validate = self.validate_point(point)?;
//         Ok(validate.x * self.cols + validate.y)
//     }
//     pub fn grid_point_to_index(&self, point: GridPoint) -> Result<usize> {
//         self.validate_grid_point(point)
//             .map(|p| p.x * self.cols + p.y)
//     }
//     /// Generic coordinate validation
//     fn validate_point<P>(&self, point: Point<P>) -> Result<GridPoint>
//     where
//         P: Zero + PartialOrd + Display + ToPrimitive + Copy,
//     {
//         point.in_bounds((self.rows, self.cols))?;
//         let zero = P::zero();
//         if point.x < zero || point.y < zero {
//             return Err(GridError::OutOfBounds {
//                 x: point.x.to_string(),
//                 y: point.y.to_string(),
//                 rows: self.rows,
//                 cols: self.cols,
//             });
//         }
//         match point.cast::<usize>() {
//             Some(point) if point.x < self.rows && point.y < self.cols => Ok(point),
//             _ => {
//                 return Err(GridError::OutOfBounds {
//                     x: point.x.to_string(),
//                     y: point.y.to_string(),
//                     rows: self.rows,
//                     cols: self.cols,
//                 })
//             }
//         }
//     }
//     fn validate_grid_point(&self, point: GridPoint) -> Result<GridPoint> {
//         if point.x >= self.rows || point.y >= self.cols {
//             return Err(GridError::OutOfBounds {
//                 x: point.x.to_string(),
//                 y: point.y.to_string(),
//                 rows: self.rows,
//                 cols: self.cols,
//             });
//         }
//         Ok(point)
//     }

// Grid Related Operations
impl<P> Point<P>
where
    P: ToPrimitive + Copy + Display,
{
    /// Checks if point is within bounds and converts to target type U
    pub fn in_bounds_as<U>(&self, bounds: &Point<U>) -> Result<Point<U>>
    where
        U: Zero + PartialOrd + NumCast + Copy + Display,
    {
        let point = self.try_cast::<U>()?;
        if point.check_bounds(bounds) {
            Ok(point)
        } else {
            Err(PointError::out_of_bounds(self.x, self.y, bounds))
        }
    }
}

impl<P> Point<P>
where
    P: Copy + Zero + PartialOrd,
{
    pub fn check_bounds(&self, &Self { x, y }: &Self) -> bool {
        //let Self { x, y } = bounds.into();
        self.x >= P::zero() && self.y >= P::zero() && self.x < x && self.y < y
    }
}

impl<P> Point<P>
where
    P: Copy + Zero + PartialOrd + Display,
{
    /// Checks if the point is within the given bounds (exclusive)
    pub fn in_bounds(&self, bounds: &Self) -> Result<Self>
    where
        Self: Into<Self> + Display + Copy,
    {
        if self.check_bounds(bounds) {
            Ok(*self)
        } else {
            Err(PointError::out_of_bounds(self.x, self.y, bounds))
        }
    }
}

impl<P> Point<P>
where
    P: Copy + Add<Output = P> + Sub<Output = P> + One,
{
    // Ordered as (up, right, down, left)
    pub fn adjacent_points(&self) -> [Self; 4]
    where
        Self: Copy,
    {
        [
            // Up
            self.sub_x(P::one()),
            // Right
            self.add_y(P::one()),
            // Down
            self.add_x(P::one()),
            // Left
            self.sub_y(P::one()),
        ]
    }
}

impl<P> Point<P>
where
    P: CheckedAdd + CheckedSub + Copy + One + Zero + PartialOrd + Display,
{
    /// Ordered as (north,east,south,west) for grid coordinate system
    pub fn bounded_cardinals(&self, bounds: &Self) -> [Option<Self>; 4] {
        [
            // North (row-1,col)
            self.checked_sub_x(P::one())
                .and_then(|p| p.in_bounds(bounds).ok()),
            // East (row,col+1)
            self.checked_add_y(P::one())
                .and_then(|p| p.in_bounds(bounds).ok()),
            // South (row+1,col)
            self.checked_add_x(P::one())
                .and_then(|p| p.in_bounds(bounds).ok()),
            // West (row,col-1)
            self.checked_sub_y(P::one())
                .and_then(|p| p.in_bounds(bounds).ok()),
        ]
    }
    /// Ordered as (NW,NE,SW,SE) for grid coordinate system
    pub fn bounded_diagonals(&self, bounds: &Self) -> [Option<Self>; 4] {
        [
            // Northwest
            self.checked_sub_x(P::one()).and_then(|p| {
                p.checked_sub_y(P::one())
                    .and_then(|p| p.in_bounds(bounds).ok())
            }),
            // Northeast
            self.checked_sub_x(P::one()).and_then(|p| {
                p.checked_add_y(P::one())
                    .and_then(|p| p.in_bounds(bounds).ok())
            }),
            // Southwest
            self.checked_add_x(P::one()).and_then(|p| {
                p.checked_sub_y(P::one())
                    .and_then(|p| p.in_bounds(bounds).ok())
            }),
            // Southeast
            self.checked_add_x(P::one()).and_then(|p| {
                p.checked_add_y(P::one())
                    .and_then(|p| p.in_bounds(bounds).ok())
            }),
        ]
    }
    ///  Ordered as (N,NE,E,SE,S,SW,W,NW) for grid coordinate system
    pub fn bounded_neighbors(&self, bounds: &Self) -> [Option<Self>; 8] {
        let [north, east, south, west] = self.bounded_cardinals(bounds);
        let [northwest, northeast, southeast, southwest] = self.bounded_diagonals(bounds);
        [
            north, northeast, east, southeast, south, southwest, west, northwest,
        ]
    }
}

// Grid index conversion
impl<P> Point<P> {
    /// Converts to grid index given number of columns.
    pub fn to_index(&self, columns: P) -> P
    where
        P: Copy + Mul<Output = P> + Add<Output = P>,
    {
        (self.y * columns) + self.x
    }
    /// Creates point from grid index given number of columns.
    pub fn from_index(index: P, columns: P) -> Self
    where
        P: Copy + Rem<Output = P> + Div<Output = P>,
    {
        Self::new(index % columns, index / columns)
    }
    /// Calculates the area of the rectangle formed by the point and another point.
    pub fn area_with(&self, other: &Self) -> P
    where
        P: Copy + Signed,
    {
        (self.x - other.x).abs() * (self.y - other.y).abs()
    }
}

// Directional operations
impl<P> Point<P> {
    pub fn move_in<D: DirectionBehaviour>(&self, direction: D) -> Self
    where
        P: Signed + Copy,
    {
        *self + direction.as_delta()
    }
    pub fn rotate_90(&self) -> Self
    where
        P: Neg<Output = P> + Copy,
    {
        Point::new(-self.y, self.x)
    }
    pub fn scale(&self, factor: P) -> Self
    where
        P: Mul<Output = P> + Copy,
    {
        Point::new(self.x * factor, self.y * factor)
    }
}

// Distance calculations
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
impl<P> Point<P> {
    pub fn cast<U>(&self) -> Option<Point<U>>
    where
        P: ToPrimitive + Copy,
        U: NumCast,
    {
        Some(Point::new(U::from(self.x)?, U::from(self.x)?))
    }
    pub fn try_cast<U>(&self) -> Result<Point<U>>
    where
        P: ToPrimitive + Copy,
        U: NumCast,
    {
        Ok(Point::new(
            U::from(self.x).ok_or(PointError::conversion("x"))?,
            U::from(self.y).ok_or(PointError::conversion("y"))?,
        ))
    }
}

// Display
impl<P: fmt::Display> fmt::Display for Point<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
