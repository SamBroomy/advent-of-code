use itertools::Itertools;
use std::{cell::RefCell, collections::HashMap};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

fn parse(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_once(',').unwrap();
            let x = l.parse().unwrap();
            let y = r.parse().unwrap();
            Point { x, y }
        })
        .collect()
}

fn area_between(p1: &Point, p2: &Point) -> u64 {
    (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1)
}

#[inline]
pub fn part1(input: &str) -> u64 {
    let points = parse(input);

    let mut largest_area = 0;
    for (i, j) in points.iter().tuple_combinations() {
        let area = area_between(i, j);
        if area > largest_area {
            largest_area = area;
        }
    }
    largest_area
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        assert!(start.x == end.x || start.y == end.y);
        Self { start, end }
    }

    fn points(&self) -> Box<dyn Iterator<Item = Point>> {
        if self.start.x == self.end.x {
            let x = self.start.x;
            let (y_start, y_end) = if self.start.y <= self.end.y {
                (self.start.y, self.end.y)
            } else {
                (self.end.y, self.start.y)
            };
            Box::new((y_start..=y_end).map(move |y| Point { x, y }))
        } else {
            let y = self.start.y;
            let (x_start, x_end) = if self.start.x <= self.end.x {
                (self.start.x, self.end.x)
            } else {
                (self.end.x, self.start.x)
            };
            Box::new((x_start..=x_end).map(move |x| Point { x, y }))
        }
    }
}

struct Shape {
    edges: Vec<Line>,
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,
    // For each y in [min_y..=max_y] a vec of inclusive x-ranges that are inside the shape.
    spans: Vec<Vec<(u64, u64)>>,
    cache: RefCell<HashMap<Point, bool>>,
}

impl Shape {
    fn new(points: &[Point]) -> Self {
        let mut edges = Vec::new();
        for (i, j) in points.iter().circular_tuple_windows() {
            edges.push(Line::new(*i, *j));
        }

        let min_x = points.iter().map(|p| p.x).min().unwrap();
        let max_x = points.iter().map(|p| p.x).max().unwrap();
        let min_y = points.iter().map(|p| p.y).min().unwrap();
        let max_y = points.iter().map(|p| p.y).max().unwrap();

        let mut spans: Vec<Vec<(u64, u64)>> = Vec::with_capacity((max_y - min_y + 1) as usize);
        for y in min_y..=max_y {
            // Collect x intersections from vertical edges where y in [edge_y_min, edge_y_max)
            let mut xs: Vec<u64> = edges
                .iter()
                .filter_map(|edge| {
                    if edge.start.x == edge.end.x {
                        let ey_min = edge.start.y.min(edge.end.y);
                        let ey_max = edge.start.y.max(edge.end.y);
                        if y >= ey_min && y < ey_max {
                            Some(edge.start.x)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
            xs.sort_unstable();
            let mut row_spans: Vec<(u64, u64)> = Vec::new();
            let mut i = 0usize;
            while i + 1 < xs.len() {
                let a = xs[i];
                let b = xs[i + 1];
                if b > a {
                    // interior x are a ..= (b-1). Points exactly on vertical edge (x == b) will be handled
                    // by on-edge checks later, so keep spans as inclusive ranges of interior tiles.
                    row_spans.push((a, b.saturating_sub(1)));
                }
                i += 2;
            }
            // merge contiguous spans just in case
            if !row_spans.is_empty() {
                let mut merged = Vec::new();
                let mut cur = row_spans[0];
                for &(s, e) in row_spans.iter().skip(1) {
                    if s <= cur.1 + 1 {
                        cur.1 = cur.1.max(e);
                    } else {
                        merged.push(cur);
                        cur = (s, e);
                    }
                }
                merged.push(cur);
                spans.push(merged);
            } else {
                spans.push(Vec::new());
            }
        }

        Self {
            edges,
            min_x,
            max_x,
            min_y,
            max_y,
            spans,
            cache: RefCell::new(HashMap::new()),
        }
    }
    fn point_on_edge(&self, point: &Point) -> bool {
        for edge in self.edges.iter() {
            if edge.start.y == edge.end.y {
                // horizontal
                if point.y == edge.start.y
                    && point.x >= edge.start.x.min(edge.end.x)
                    && point.x <= edge.start.x.max(edge.end.x)
                {
                    return true;
                }
            } else {
                // vertical
                if point.x == edge.start.x
                    && point.y >= edge.start.y.min(edge.end.y)
                    && point.y <= edge.start.y.max(edge.end.y)
                {
                    return true;
                }
            }
        }
        false
    }
    fn is_inside(&self, point: &Point) -> bool {
        if let Some(cached) = self.cache.borrow().get(point) {
            return *cached;
        }

        if point.x < self.min_x
            || point.x > self.max_x
            || point.y < self.min_y
            || point.y > self.max_y
        {
            self.cache.borrow_mut().insert(*point, false);
            return false;
        }
        let row = (point.y - self.min_y) as usize;
        // Check spans first
        if self.spans[row]
            .iter()
            .any(|&(s, e)| point.x >= s && point.x <= e)
        {
            self.cache.borrow_mut().insert(*point, true);
            return true;
        }
        // If not in a span, it might still be exactly on an edge
        let out = self.point_on_edge(point);
        self.cache.borrow_mut().insert(*point, out);
        out
    }

    // Check corners first, then verify each row of the rectangle is fully covered by spans
    // (allowing points on edges via point_on_edge)
    fn is_rectangle_inside(&self, rect: &Rectangle) -> bool {
        let top_left = Point {
            x: rect.point_1.x.min(rect.point_2.x),
            y: rect.point_1.y.min(rect.point_2.y),
        };
        let bottom_right = Point {
            x: rect.point_1.x.max(rect.point_2.x),
            y: rect.point_1.y.max(rect.point_2.y),
        };
        let top_right = Point {
            x: bottom_right.x,
            y: top_left.y,
        };
        let bottom_left = Point {
            x: top_left.x,
            y: bottom_right.y,
        };

        if !(self.is_inside(&top_left)
            && self.is_inside(&top_right)
            && self.is_inside(&bottom_left)
            && self.is_inside(&bottom_right))
        {
            return false;
        }

        // For each row, check the required x-range is covered by the union of spans or by points on edges.
        for y in top_left.y..=bottom_right.y {
            if y < self.min_y || y > self.max_y {
                return false;
            }
            let row = (y - self.min_y) as usize;
            let req_start = top_left.x;
            let req_end = bottom_right.x;

            // Fast check using merged spans
            let mut cur = req_start;
            for &(s, e) in &self.spans[row] {
                if s > cur {
                    // gap before this span
                    break;
                }
                if e >= cur {
                    cur = e.saturating_add(1);
                    if cur > req_end {
                        break;
                    }
                }
            }
            if cur > req_end {
                continue; // this row fully covered by spans
            }
            // There is a remaining gap [cur..=req_end]; all those x must be on edges
            let mut all_on_edge = true;
            for x in cur..=req_end {
                if !self.point_on_edge(&Point { x, y }) {
                    all_on_edge = false;
                    break;
                }
            }
            if !all_on_edge {
                return false;
            }
        }
        true
    }
}

struct Rectangle {
    point_1: Point,
    point_2: Point,
}

impl Rectangle {
    fn area(&self) -> u64 {
        area_between(&self.point_1, &self.point_2)
    }
}

#[inline]
pub fn part2(input: &str) -> u64 {
    let red_points = parse(input);
    // Bounding boxes of our grid.
    let shape = Shape::new(&red_points);

    let mut largest_area = 0;
    for (i, j) in red_points.iter().tuple_combinations() {
        let rectangle = Rectangle {
            point_1: *i,
            point_2: *j,
        };
        let area = rectangle.area();
        if area > largest_area && shape.is_rectangle_inside(&rectangle) {
            largest_area = area;
        }
    }
    println!("Largest area found: {}", largest_area);
    largest_area
}

common::aoc_test!(50, 4763509452, 24, 1516897893);
