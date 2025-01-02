mod math;
use math::{crt, stats};
trait New
where
    Self: Sized,
{
    fn new(x: i32, y: i32) -> Self;
    fn from_str(s: &str) -> Self {
        let m = s
            .split_once('=')
            .map(|(_, right)| {
                right
                    .split(',')
                    .map(|num| num.trim().parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .unwrap();
        Self::new(m[0], m[1])
    }
}
#[derive(Debug)]
struct Velocity(i32, i32);

impl New for Velocity {
    fn new(i: i32, j: i32) -> Self {
        Self(i, j)
    }
}

enum Quadrant {
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Ord, Eq)]
struct Position(i32, i32);

impl New for Position {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }
}
impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self {
        Self::new(x, y)
    }
}

impl Position {
    fn get_quadrants(&self, tile_dims: &Position) -> Quadrant {
        let half_x = (tile_dims.0 - 1) / 2;
        let half_y = (tile_dims.1 - 1) / 2;
        if self.0 > half_x && self.1 > half_y {
            Quadrant::TopRight
        } else if self.0 < half_x && self.1 > half_y {
            Quadrant::TopLeft
        } else if self.0 < half_x && self.1 < half_y {
            Quadrant::BottomLeft
        } else if self.0 > half_x && self.1 < half_y {
            Quadrant::BottomRight
        } else {
            Quadrant::None
        }
    }
}
#[derive(Debug)]
struct Robot {
    position: Position,
    velocity: Velocity,
}

impl Robot {
    fn new(position: Position, velocity: Velocity) -> Self {
        Robot { position, velocity }
    }

    fn from_line(line: &str) -> Self {
        let (pos, v) = line.trim().split_once(" ").unwrap();
        let position = Position::from_str(pos);
        let velocity = Velocity::from_str(v);
        Robot::new(position, velocity)
    }

    fn from_input(input: &str) -> Vec<Self> {
        input.lines().map(Self::from_line).collect()
    }

    fn next(&mut self, tile_size: &Position) {
        self.position.0 = (self.position.0 + self.velocity.0).rem_euclid(tile_size.0);
        self.position.1 = (self.position.1 + self.velocity.1).rem_euclid(tile_size.1);
    }
}

#[derive(Debug)]
struct Room {
    robots: Vec<Robot>,
    tile_size: Position,
    iterations: i32,
}

impl Room {
    fn new(robots: Vec<Robot>, tile_size: impl Into<Position>) -> Self {
        Self {
            robots,
            tile_size: tile_size.into(),
            iterations: 0,
        }
    }

    fn move_robots(&mut self) {
        for robot in self.robots.iter_mut() {
            robot.next(&self.tile_size);
        }
        self.iterations += 1;
    }

    fn move_robots_n_times(&mut self, n: i32) {
        for _ in 0..n {
            self.move_robots();
        }
    }

    fn get_output(&mut self) -> i64 {
        self.move_robots_n_times(100);
        self.get_quadrants()
    }

    fn get_quadrants(&self) -> i64 {
        self.robots
            .iter()
            .fold([0; 4], |mut counts, robot| {
                match robot.position.get_quadrants(&self.tile_size) {
                    Quadrant::TopRight => counts[0] += 1,
                    Quadrant::TopLeft => counts[1] += 1,
                    Quadrant::BottomLeft => counts[2] += 1,
                    Quadrant::BottomRight => counts[3] += 1,
                    Quadrant::None => (),
                }
                counts
            })
            .iter()
            .product::<i64>()
    }

    /// Part 2: Finds the iteration at which the robots' positions have minimal variance
    fn get_output_part2(&mut self) -> Option<i32> {
        if self.robots.len() != 500 {
            return None;
        }

        let rows = self.tile_size.0;
        let columns = self.tile_size.1;
        let num_iterations = usize::max(rows as usize, columns as usize);

        let mut x_variance = Vec::with_capacity(num_iterations);
        let mut y_variance = Vec::with_capacity(num_iterations);
        let mut x_positions = vec![0; self.robots.len()];
        let mut y_positions = vec![0; self.robots.len()];

        for _ in 0..num_iterations {
            for (i, robot) in self.robots.iter_mut().enumerate() {
                robot.next(&self.tile_size);
                x_positions[i] = robot.position.0;
                y_positions[i] = robot.position.1;
            }
            x_variance.push(stats::variance(&x_positions));
            y_variance.push(stats::variance(&y_positions));
        }

        let min_x_variance = x_variance
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx as i32 + 1)?;

        let min_y_variance = y_variance
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(idx, _)| idx as i32 + 1)?;

        crt::solve(min_x_variance, rows, min_y_variance, columns)
    }
}

#[inline]
pub fn part1(input: &str) -> i64 {
    let robots = Robot::from_input(input);
    let tile_size = match robots.len() {
        12 => (11, 7),
        500 => (101, 103),
        _ => unreachable!("Unsupported grid"),
    };
    let mut room = Room::new(robots, tile_size);
    room.get_output()
}

#[inline]
pub fn part2(input: &str) -> i32 {
    let robots = Robot::from_input(input);
    if robots.len() != 500 {
        return 0;
    }

    let mut room = Room::new(robots, (101, 103));
    room.get_output_part2().unwrap_or(0)
}

common::aoc_test!(12, 222208000, 0, 7623);
