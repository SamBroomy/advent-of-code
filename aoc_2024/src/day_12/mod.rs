use ahash::AHashSet as HashSet;
use common::prelude::{CharGrid, DiagonalDirections, DirectionBehaviour, Grid, GridPoint, Point};
use std::collections::VecDeque;

fn calculate_perimeter_and_area<const DISCOUNT_ENABLED: bool>(
    grid: &CharGrid,
    seen_points: &mut HashSet<GridPoint>,
    start: GridPoint,
    colour: char,
) -> GridPoint {
    if seen_points.contains(&start) {
        return Point::zero();
    }

    let mut perimeter = 0;
    let mut perimeter_points: HashSet<GridPoint> = HashSet::new();
    let area_before = seen_points.len();
    seen_points.insert(start);

    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some(point) = q.pop_front() {
        let mut same_colour_adjacents = 0;

        point
            .bounded_cardinals(&(grid.rows, grid.cols).into())
            .into_iter()
            .flatten()
            .for_each(|p| {
                if let Ok(val) = grid.get_ref(p) {
                    if val == &colour {
                        same_colour_adjacents += 1;
                        if !seen_points.contains(&p) {
                            q.push_back(p);
                            seen_points.insert(p);
                        }
                    }
                }
            });

        perimeter += 4 - same_colour_adjacents;
        perimeter_points.insert(point);
    }
    let area_after = seen_points.len();
    let perimeter = if DISCOUNT_ENABLED {
        perimeter_corners(perimeter_points, grid.rows, grid.cols)
    } else {
        perimeter
    };
    Point::new(perimeter, area_after - area_before)
}

fn perimeter_corners(points: HashSet<GridPoint>, rows: usize, columns: usize) -> usize {
    let check_diagonal_corner = |point: &GridPoint, dir: DiagonalDirections| {
        dir.next_point(point)
            .map(|p| !points.contains(&p))
            .unwrap_or(false) as usize
    };
    let count_all_corners = |point: &GridPoint| {
        DiagonalDirections::ALL
            .iter()
            .map(|dir| check_diagonal_corner(point, *dir))
            .sum::<usize>()
    };
    let count_two_corners = |point: &GridPoint, dirs: &[DiagonalDirections; 2]| {
        dirs.iter()
            .map(|dir| check_diagonal_corner(point, *dir))
            .sum::<usize>()
    };

    points
        .iter()
        .map(|point| {
            let is_point_contained =
                point
                    .bounded_cardinals(&(rows, columns).into())
                    .map(|p| match p {
                        Some(p) => points.contains(&p),
                        _ => false,
                    });

            match is_point_contained {
                // All cardinals present - check all diagonals
                [true, true, true, true] => count_all_corners(point),

                // Three cardinals present - check two diagonals
                [true, true, true, false] => count_two_corners(
                    point,
                    &[DiagonalDirections::SouthEast, DiagonalDirections::NorthEast],
                ),
                [true, true, false, true] => count_two_corners(
                    point,
                    &[DiagonalDirections::NorthWest, DiagonalDirections::NorthEast],
                ),
                [true, false, true, true] => count_two_corners(
                    point,
                    &[DiagonalDirections::SouthWest, DiagonalDirections::NorthWest],
                ),
                [false, true, true, true] => count_two_corners(
                    point,
                    &[DiagonalDirections::SouthWest, DiagonalDirections::SouthEast],
                ),

                // Two cardinals present - check one diagonal
                [true, true, false, false] => {
                    1 + check_diagonal_corner(point, DiagonalDirections::NorthEast)
                }
                [true, false, false, true] => {
                    1 + check_diagonal_corner(point, DiagonalDirections::NorthWest)
                }
                [false, true, true, false] => {
                    1 + check_diagonal_corner(point, DiagonalDirections::SouthEast)
                }
                [false, false, true, true] => {
                    1 + check_diagonal_corner(point, DiagonalDirections::SouthWest)
                }

                // Special cases
                [true, false, true, false] | [false, true, false, true] => 0,
                [true, false, false, false]
                | [false, true, false, false]
                | [false, false, true, false]
                | [false, false, false, true] => 2,
                [false, false, false, false] => 4,
            }
        })
        .sum()
}

fn parse(input: &str) -> CharGrid {
    Grid::build_raw_input(input).unwrap()
}

#[inline]
pub fn part1(input: &str) -> i32 {
    let grid = parse(input);
    let mut seen_points = HashSet::with_capacity(grid.size());

    grid.iter_points()
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

    grid.iter_points()
        .map(|(start, colour)| {
            calculate_perimeter_and_area::<true>(&grid, &mut seen_points, start, colour)
        })
        .map(|Point { x, y }| (x * y) as i32)
        .sum()
}

common::aoc_test!(1930, 1450422, 1206, 906606);
