use super::direction::Direction;
use super::point::Point;
use super::point::PointConversion;
use num::iter::RangeStep;
use num::range_step;
use num_traits::PrimInt;
use std::convert::TryFrom;
use std::fmt;
use std::ops::{Index, IndexMut, Range};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    pub rows: usize,
    pub cols: usize,
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

    pub fn from_vec(data: Vec<T>, rows: usize, cols: usize) -> Self {
        assert_eq!(data.len(), rows * cols);
        Self { data, rows, cols }
    }

    pub fn from_2d_vec(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data.first().map_or(0, Vec::len);
        assert!(data.iter().all(|row| row.len() == cols));

        let data = data.into_iter().flatten().collect();
        Self { data, rows, cols }
    }

    pub fn construct(input: &str, mapper: impl Fn(char) -> T) -> Self {
        let cols = input.lines().next().map_or(0, str::len);
        let rows = input.lines().count();
        let data = input
            .lines()
            .flat_map(|line| line.chars().map(&mapper))
            .collect();

        Self { data, rows, cols }
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }
}

// Generic access methods
impl<T> Grid<T> {
    pub fn point_to_index<P>(&self, point: Point<P>) -> Result<usize>
    where
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        let p = self.validate_coords(point)?;
        Ok(p.x * self.cols + p.y)
    }
    // Generic coordinate validation
    fn validate_coords<P>(&self, point: Point<P>) -> Result<Point<usize>>
    where
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        if point.x < P::zero() || point.y < P::zero() {
            return Err(GridError::OutOfBounds {
                x: format!("{:?}", point.x),
                y: format!("{:?}", point.y),
                rows: self.rows,
                cols: self.cols,
            });
        }

        let x = usize::try_from(point.x).map_err(|_| {
            GridError::ConversionError(format!("Failed to convert x coordinate: {:?}", point.x))
        })?;
        let y = usize::try_from(point.y).map_err(|_| {
            GridError::ConversionError(format!("Failed to convert y coordinate: {:?}", point.y))
        })?;
        if x >= self.rows || y >= self.cols {
            return Err(GridError::OutOfBounds {
                x: format!("{:?}", point.x),
                y: format!("{:?}", point.y),
                rows: self.rows,
                cols: self.cols,
            });
        }
        Ok((x, y).into())
    }

    pub fn get_ref<P>(&self, point: Point<P>) -> Result<&T>
    where
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        self.point_to_index(point).map(|idx| &self.data[idx])
    }

    pub fn set<P>(&mut self, point: Point<P>, val: T) -> Option<()>
    where
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        self.point_to_index(point)
            .map(|idx| {
                self.data[idx] = val;
            })
            .ok()
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

    pub fn column_ref(&self, column: usize) -> Option<Vec<&T>> {
        if column >= self.cols {
            return None;
        }
        Some(self.data.iter().skip(column).step_by(self.cols).collect())
    }

    fn idx_to_point(&self, idx: usize) -> Point<usize> {
        (idx / self.cols, idx % self.cols).into()
    }

    pub fn step_in_direction<P>(&self, from: &Point<P>, dir: Direction) -> Option<Point<usize>>
    where
        P: PrimInt,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        let delta = dir.as_point::<i32>();
        let next = from.try_convert().ok()? + delta;
        if next.x < 0 || next.y < 0 {
            return None;
        }
        let next = next.try_convert().ok()?;
        if next.x >= self.rows || next.y >= self.cols {
            None
        } else {
            Some(next)
        }
    }

    pub fn value_in_direction<P>(&self, from: &Point<P>, dir: Direction) -> Option<&T>
    where
        P: PrimInt,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        self.step_in_direction(from, dir)
            .and_then(|next| self.get_ref::<usize>(next).ok())
    }

    fn direction_iter<'a, P>(
        &'a self,
        point: &'a Point<P>,
        directions: &'a [Direction],
    ) -> impl Iterator<Item = Point<usize>> + use<'a, P, T>
    where
        P: PrimInt,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        directions
            .iter()
            .filter_map(move |&dir| self.step_in_direction(point, dir))
    }

    fn adjacent_points_iter<'a, P, const INCLUDE_DIAGONALS: bool>(
        &'a self,
        point: &'a Point<P>,
    ) -> impl Iterator<Item = Point<usize>> + 'a
    where
        P: PrimInt + 'a,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        let directions = if INCLUDE_DIAGONALS {
            &Direction::ALL[..]
        } else {
            &Direction::CARDINALS[..]
        };
        self.direction_iter(point, directions)
    }

    pub fn adjacent_points<P, const INCLUDE_DIAGONALS: bool>(
        &self,
        point: &Point<P>,
    ) -> Vec<Point<usize>>
    where
        P: PrimInt,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        self.adjacent_points_iter::<P, INCLUDE_DIAGONALS>(point)
            .collect()
    }

    pub fn adjacent_values<P, const INCLUDE_DIAGONALS: bool>(&self, point: &Point<P>) -> Vec<&T>
    where
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
    {
        self.adjacent_points_iter::<P, INCLUDE_DIAGONALS>(point)
            .filter_map(|p| self.get_ref::<usize>(p).ok())
            .collect()
    }
}

impl<T, P> Index<Point<P>> for Grid<T>
where
    P: PrimInt + fmt::Debug,
    usize: TryFrom<P>,
    <usize as TryFrom<P>>::Error: fmt::Debug,
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
    P: PrimInt + fmt::Debug,
    usize: TryFrom<P>,
    <usize as TryFrom<P>>::Error: fmt::Debug,
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
        P: PrimInt + fmt::Debug,
        usize: TryFrom<P>,
        <usize as TryFrom<P>>::Error: fmt::Debug,
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

mod error {

    use std::fmt;

    pub type Result<T> = std::result::Result<T, GridError>;

    #[derive(Debug, Clone)]
    pub enum GridError {
        ConversionError(String),
        OutOfBounds {
            x: String,
            y: String,
            rows: usize,
            cols: usize,
        },
    }

    impl GridError {
        pub fn conversion_error(msg: impl std::fmt::Display) -> Self {
            Self::ConversionError(msg.to_string())
        }
    }

    impl From<&str> for GridError {
        fn from(msg: &str) -> Self {
            Self::ConversionError(msg.to_string())
        }
    }

    impl fmt::Display for GridError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                GridError::ConversionError(msg) => write!(f, "Conversion error: {}", msg),
                GridError::OutOfBounds { x, y, rows, cols } => {
                    write!(
                        f,
                        "Index ({}, {}) out of bounds (rows: {}, cols: {})",
                        x, y, rows, cols
                    )
                }
            }
        }
    }

    impl std::error::Error for GridError {}
}
use error::{GridError, Result};
