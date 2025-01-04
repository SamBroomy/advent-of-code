use common::prelude::PointConversion;
use std::collections::{HashSet, VecDeque};

type Grid = common::prelude::Grid<char>;
type Point = common::prelude::Point<usize>;

fn calculate_perimeter_and_area<const DISCOUNT_ENABLED: bool>(
    grid: &Grid,
    seen_points: &mut HashSet<Point>,
    start: Point,
    colour: char,
) -> Point {
    if seen_points.contains(&start) {
        return Point::zero();
    }

    let mut perimeter = 0;
    let mut perimeter_points: HashSet<common::prelude::Point<i32>> = HashSet::new();
    let area_before = seen_points.len();
    seen_points.insert(start);

    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(point) = q.pop_front() {
        let mut same_colour_adjacents = 0;
        let adjacents = grid.adjacent_points::<usize, false>(&point);
        for p in adjacents {
            if let Ok(val) = grid.get(p) {
                if val == colour {
                    same_colour_adjacents += 1;
                    if !seen_points.contains(&p) {
                        q.push_back(p);
                        seen_points.insert(p);
                    }
                }
            }
        }
        perimeter += 4 - same_colour_adjacents;
        if let Ok(p) = point.try_convert() {
            perimeter_points.insert(p);
        }
    }
    let area_after = seen_points.len();
    let perimeter = if DISCOUNT_ENABLED {
        perimeter_corners(perimeter_points, grid.rows, grid.cols)
    } else {
        perimeter
    };
    Point::new(perimeter, area_after - area_before)
}

fn perimeter_corners(
    points: HashSet<common::prelude::Point<i32>>,
    rows: usize,
    columns: usize,
) -> usize {
    points
        .iter()
        .map(|point| {
            let up = (point.x > 0)
                .then(|| points.contains(&(point.add_x(-1))))
                .unwrap_or(false);
            let right = (point.y + 1 < columns as i32)
                .then(|| points.contains(&(point.add_y(1))))
                .unwrap_or(false);
            let down = (point.x + 1 < rows as i32)
                .then(|| points.contains(&(point.add_x(1))))
                .unwrap_or(false);
            let left = (point.y > 0)
                .then(|| points.contains(&(point.add_y(-1))))
                .unwrap_or(false);

            match [up, right, down, left] {
                [true, true, true, true] => {
                    !points.contains(&(point.x + 1, point.y + 1).into()) as usize
                        + !points.contains(&(point.x - 1, point.y - 1).into()) as usize
                        + !points.contains(&(point.x + 1, point.y - 1).into()) as usize
                        + !points.contains(&(point.x - 1, point.y + 1).into()) as usize
                }
                [true, true, true, false] => {
                    !points.contains(&(point.x + 1, point.y + 1).into()) as usize
                        + !points.contains(&(point.x - 1, point.y + 1).into()) as usize
                }
                [true, true, false, true] => {
                    !points.contains(&(point.x - 1, point.y - 1).into()) as usize
                        + !points.contains(&(point.x - 1, point.y + 1).into()) as usize
                }
                [true, false, true, true] => {
                    !points.contains(&(point.x + 1, point.y - 1).into()) as usize
                        + !points.contains(&(point.x - 1, point.y - 1).into()) as usize
                }
                [false, true, true, true] => {
                    !points.contains(&(point.x + 1, point.y - 1).into()) as usize
                        + !points.contains(&(point.x + 1, point.y + 1).into()) as usize
                }
                [true, true, false, false] => {
                    1 + !points.contains(&(point.x - 1, point.y + 1).into()) as usize
                }
                [true, false, true, false] => 0,
                [true, false, false, true] => {
                    1 + !points.contains(&(point.x - 1, point.y - 1).into()) as usize
                }
                [false, true, true, false] => {
                    1 + !points.contains(&(point.x + 1, point.y + 1).into()) as usize
                }
                [false, true, false, true] => 0,
                [false, false, true, true] => {
                    1 + !points.contains(&(point.x + 1, point.y - 1).into()) as usize
                }
                [true, false, false, false] => 2,
                [false, true, false, false] => 2,
                [false, false, true, false] => 2,
                [false, false, false, true] => 2,
                [false, false, false, false] => 4,
            }
        })
        .sum()
}

fn parse(input: &str) -> Grid {
    Grid::construct(input, |c| c)
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let grid = parse(input);
    let mut seen_points = HashSet::with_capacity(grid.size());

    grid.iter()
        .map(|(start, colour)| {
            calculate_perimeter_and_area::<false>(&grid, &mut seen_points, start, colour)
        })
        .map(|Point { x, y }| (x * y) as i32)
        .sum()
}

#[inline]
pub fn part2(_input: &str) -> i32 {
    let grid = parse(_input);
    let mut seen_points = HashSet::with_capacity(grid.size());

    grid.iter()
        .map(|(start, colour)| {
            calculate_perimeter_and_area::<true>(&grid, &mut seen_points, start, colour)
        })
        .map(|Point { x, y }| (x * y) as i32)
        .sum()
}

common::aoc_test!(1930, 1450422, 1206, 906606);
