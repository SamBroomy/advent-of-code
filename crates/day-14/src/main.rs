use anyhow::Result;
use common::get_input;
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

    fn from_input(input: &str, tile_size: Position) -> Self {
        let robots = Robot::from_input(input);
        Self::new(robots, tile_size)
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
}

type Input<'a> = (&'a str, Position);

fn part_1((input, tile_size): Input) -> i64 {
    let mut room = Room::from_input(input, tile_size);
    room.get_output()
}

fn main() -> Result<()> {
    let input = get_input(14)?;
    let tile_size: Position = (101, 103).into();

    let start = std::time::Instant::now();
    let p1 = part_1((&input, tile_size));
    println!("Part 1: {} (took {:?})", p1, start.elapsed());

    // let start = std::time::Instant::now();
    // let p2 = part_2((&input, tile_size));
    // println!("Part 2: {} (took {:?})", p2, start.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: (&str, Position) = (
        "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
        Position(11, 7),
    );

    const INPUT_GRID_SIZE: Position = Position(101, 103);

    #[test]
    fn part_1_example() {
        let total = part_1(EXAMPLE_INPUT);
        assert_eq!(total, 12);
    }

    #[test]
    fn test_part_1() {
        let input = get_input(14).unwrap();
        let total = part_1((&input, INPUT_GRID_SIZE));
        assert_eq!(total, 222208000);
    }

    // #[test]
    // fn part_2_example() {
    //     let total = part_2(EXAMPLE_INPUT);
    //     assert_eq!(total, 0);
    // }

    // #[test]
    // fn test_part_2() {
    //     let input = get_input(14).unwrap();
    //     let total = part_2((&input, INPUT_GRID_SIZE));
    //     assert_eq!(total, 0);
    // }
}
