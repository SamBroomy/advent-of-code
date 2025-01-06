/// A 2D grid containing values of type T.
/// Responsibilities:
/// - Value storage and access
/// - Boundry enforcement
/// - Neighbourhood operations
/// - Search operations
/// - Traversal operations
use super::direction::DirectionBehaviour;
use super::direction::DirectionalMove;
use super::point::Point;
use num_iter::{range_step, RangeStep};
use num_traits::{CheckedAdd, CheckedSub, One, ToPrimitive, Zero};
use std::convert::From;
use std::fmt::{self, Display};
use std::ops::{Index, IndexMut, Range};

mod error;
use error::{GridError, Result};
/// Type aliases for common grid patterns
pub type GridPoint = Point<usize>;
pub type WorldPoint = Point<i32>;
/// Grid types for common use cases
pub type BoolGrid = Grid<bool>;
pub type CharGrid = Grid<char>;
pub type IntGrid = Grid<i32>;
#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T> {
    fn validate_dimensions(rows: usize, cols: usize) -> Result<()> {
        if rows == 0 {
            return Err(GridError::BuilderError("Empty input".into()));
        }
        if cols == 0 {
            return Err(GridError::BuilderError("Empty line in input".into()));
        }
        Ok(())
    }

    fn validate_data_length<U>(data: &[U], expected_cols: usize) -> Result<()> {
        if data.len() != expected_cols {
            return Err(GridError::BuilderError("Inconsistent line lengths".into()));
        }
        Ok(())
    }

    pub fn build_default(rows: usize, cols: usize, value: T) -> Result<Grid<T>>
    where
        T: Clone,
    {
        Self::validate_dimensions(rows, cols)?;
        Ok(Grid::new(rows, cols, value))
    }

    pub fn build_vec(data: Vec<T>, rows: usize, cols: usize) -> Result<Grid<T>> {
        Self::validate_data_length(&data, rows * cols)?;
        Ok(Grid { data, rows, cols })
    }

    pub fn build_2d_vec(data: Vec<Vec<T>>) -> Result<Grid<T>> {
        let rows = data.len();
        let cols = data[0].len();
        Self::validate_dimensions(rows, cols)?;
        if !data.iter().all(|row| row.len() == cols) {
            return Err(GridError::BuilderError("Inconsistent row lengths".into()));
        }
        Ok(Grid {
            data: data.into_iter().flatten().collect(),
            rows,
            cols,
        })
    }

    pub fn build_raw_input(input: &str) -> Result<Grid<T>>
    where
        T: From<char>,
    {
        Self::build_mapped(input, T::from)
    }

    pub fn build_mapped(input: &str, mapper: impl Fn(char) -> T) -> Result<Grid<T>> {
        let rows = input.lines().count();
        let cols = if input.is_empty() {
            0
        } else {
            input.lines().next().unwrap().len()
        };
        Self::validate_dimensions(rows, cols)?;
        if !input.lines().all(|line: &str| line.len() == cols) {
            return Err(GridError::BuilderError("Inconsistent line lengths".into()));
        }
        let data = input
            .lines()
            .flat_map(|line| line.chars())
            .map(mapper)
            .collect();
        Ok(Grid { data, rows, cols })
    }
}

pub struct GridView<'a, T> {
    grid: &'a Grid<T>,
    bounds: Rectangle,
}

impl<T> GridView<'_, T> {
    pub fn iter(&self) -> impl Iterator<Item = (GridPoint, &T)> {
        self.bounds
            .points()
            .filter_map(|p| self.grid.get_ref(p).ok().map(|v| (p, v)))
    }
}
impl<T> Index<GridPoint> for GridView<'_, T> {
    type Output = T;

    fn index(&self, point: GridPoint) -> &Self::Output {
        self.grid.get_ref(point).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub top_left: GridPoint,
    pub bottom_right: GridPoint,
}

impl Rectangle {
    pub fn new(top_left: impl Into<GridPoint>, bottom_right: impl Into<GridPoint>) -> Self {
        Self {
            top_left: top_left.into(),
            bottom_right: bottom_right.into(),
        }
    }

    pub fn entire_grid(bound: impl Into<GridPoint>) -> Self {
        Self {
            top_left: GridPoint::zero(),
            bottom_right: bound.into(),
        }
    }

    pub fn points(&self) -> impl Iterator<Item = GridPoint> + use<'_> {
        (self.top_left.x..self.bottom_right.x)
            .flat_map(move |x| (self.top_left.y..self.bottom_right.y).map(move |y| (x, y).into()))
    }
    pub fn around_point(center: impl Into<GridPoint>, radius: impl Into<Option<usize>>) -> Self {
        let center = center.into();
        let radius = radius.into().unwrap_or(1);
        Self {
            top_left: (
                center.x.saturating_sub(radius),
                center.y.saturating_sub(radius),
            )
                .into(),
            bottom_right: (center.x + radius, center.y + radius).into(),
        }
    }
}
impl From<(GridPoint, GridPoint)> for Rectangle {
    fn from((top_left, bottom_right): (GridPoint, GridPoint)) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl<T> Grid<T> {
    pub fn new(rows: usize, cols: usize, default: T) -> Self
    where
        T: Clone,
    {
        Self {
            data: vec![default; rows * cols],
            rows,
            cols,
        }
    }
    pub fn size(&self) -> usize {
        self.rows * self.cols
    }
}

// Generic access methods
impl<T> Grid<T> {
    ///
    /// Generic coordinate validation
    fn validate_point<P>(&self, point: Point<P>) -> Result<GridPoint>
    where
        P: Copy + ToPrimitive + Display,
    {
        point
            .in_bounds_as(&(self.rows, self.cols).into())
            .map_err(|_| GridError::OutOfBounds {
                x: point.x.to_string(),
                y: point.y.to_string(),
                rows: self.rows,
                cols: self.cols,
            })
    }

    pub fn point_to_index<P>(&self, point: Point<P>) -> Result<usize>
    where
        P: Copy + ToPrimitive + Display,
    {
        let validate = self.validate_point(point)?;
        Ok(validate.x * self.cols + validate.y)
    }

    pub fn get_ref<P>(&self, point: Point<P>) -> Result<&T>
    where
        P: Copy + ToPrimitive + Display,
    {
        self.point_to_index(point).map(|idx| &self.data[idx])
    }

    pub fn set<P>(&mut self, point: Point<P>, val: T) -> Option<()>
    where
        P: Copy + ToPrimitive + Display,
    {
        self.point_to_index(point)
            .ok()
            .map(|idx| self.data[idx] = val)
    }

    pub fn row_range(&self, row: usize) -> Range<usize> {
        row * self.cols..(row + 1) * self.cols
    }

    pub fn row_ref(&self, row: usize) -> Option<&[T]> {
        if row >= self.rows {
            return None;
        }
        Some(&self.data[self.row_range(row)])
    }

    pub fn column_range(&self, column: usize) -> RangeStep<usize> {
        range_step(column, self.data.len(), self.cols)
    }

    pub fn column_ref(&self, column: usize) -> Option<impl Iterator<Item = &T>> {
        if column >= self.cols {
            return None;
        }
        Some(self.data.iter().skip(column).step_by(self.cols))
    }

    fn idx_to_point(&self, idx: usize) -> GridPoint {
        (idx / self.cols, idx % self.cols).into()
    }

    pub fn step_in_direction<P, D>(&self, mover: DirectionalMove<P, D>) -> Option<GridPoint>
    where
        D: DirectionBehaviour,
        P: CheckedAdd + CheckedSub + Copy + One + ToPrimitive + Display + Zero,
    {
        let next = mover.get_next_point()?;
        self.validate_point(next).ok()
    }

    pub fn value_in_direction<P, D>(&self, mover: DirectionalMove<P, D>) -> Option<&T>
    where
        D: DirectionBehaviour,
        P: CheckedAdd + CheckedSub + Copy + One + ToPrimitive + Display + Zero,
    {
        self.step_in_direction(mover)
            .and_then(|p| self.get_ref(p).ok())
    }
}

impl<T, P> Index<Point<P>> for Grid<T>
where
    P: Copy + ToPrimitive + Display,
{
    type Output = T;

    fn index(&self, point: Point<P>) -> &Self::Output {
        self.point_to_index(point)
            .map(move |idx| &self.data[idx])
            .unwrap()
    }
}

impl<T, P> IndexMut<Point<P>> for Grid<T>
where
    P: Copy + ToPrimitive + Display,
{
    fn index_mut(&mut self, point: Point<P>) -> &mut Self::Output {
        self.point_to_index(point)
            .map(move |idx| &mut self.data[idx])
            .unwrap()
    }
}

impl<T> Grid<T>
where
    T: Copy,
{
    pub fn iter(&self) -> impl Iterator<Item = (Point<usize>, T)> + '_ {
        self.data
            .iter()
            .enumerate()
            .map(|(idx, c)| (self.idx_to_point(idx), *c))
    }

    pub fn get<P>(&self, point: Point<P>) -> Result<T>
    where
        P: Copy + ToPrimitive + Display,
    {
        self.get_ref(point).copied()
    }

    pub fn row(&self, row: usize) -> Option<Vec<T>> {
        self.row_ref(row).map(|row| row.to_vec())
    }

    pub fn column(&self, column: usize) -> Option<Vec<T>> {
        if column >= self.cols {
            return None;
        }
        Some(
            self.data
                .iter()
                .skip(column)
                .step_by(self.cols)
                .copied()
                .collect(),
        )
    }
}

impl<T> Grid<T>
where
    T: PartialEq,
{
    pub fn search(&self, needle: T) -> Option<Point<usize>> {
        self.data.iter().enumerate().find_map(|(idx, c)| {
            if *c == needle {
                return Some(self.idx_to_point(idx));
            }
            None
        })
    }
    pub fn search_all(&self, needle: T) -> Vec<Point<usize>> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(idx, c)| {
                if *c == needle {
                    return Some(self.idx_to_point(idx));
                }
                None
            })
            .collect()
    }
}

impl<T: fmt::Display> fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "Row {} |", i)?;
            for j in 0..self.cols {
                let val = self.get_ref((i, j).into()).unwrap();
                write!(f, " {} |", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
