use std::str::FromStr;

use advent_of_code::point3d::Point3D;
use geo::{Contains, Coord, Line, Rect};

const START: usize = 7;
#[cfg(not(debug_assertions))]
const START: usize = 7;

const END: usize = 27;
#[cfg(not(debug_assertions))]
const END: usize = 7;

#[derive(Debug)]
struct Speed3D {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&str> for Speed3D {
    fn from(value: &str) -> Self {
        let mut speeds = value.split(", ");
        Self {
            x: speeds.next().unwrap().parse().unwrap(),
            y: speeds.next().unwrap().parse().unwrap(),
            z: speeds.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Hailstone {
    point: Point3D,
    speed: Speed3D,
}

impl Hailstone {
    fn get_start_coord(&self) -> Coord {
        let c = Coord::from((self.point.x as f64, self.point.y as f64));
        let window = (
            Coord::from((START as f64, START as f64)),
            Coord::from((END as f64, END as f64)),
        );
        let rectangle = Rect::new(window.0, window.1);
        let is_inside = rectangle.contains(&c);
        if is_inside {
            return c;
        }
        let is_over_max_x = c.x > END as f64;
        let is_over_max_y = c.y > END as f64;
        let time_delta_before_crossing_x = (END as f64 - c.x) / self.speed.x as f64;

        c
    }
}

#[test]
fn test_get_start_point() {
    let hailstone = Hailstone::from("12, 31, 28 @ -1, -2, -1");
    assert_eq!(hailstone.get_start_coord(), Coord::from((10., 27.)));
}

impl From<&str> for Hailstone {
    fn from(value: &str) -> Self {
        let (point, speed) = value.split_once(" @ ").unwrap();
        let point = Point3D::from_str(point).unwrap();
        let speed = Speed3D::from(speed);
        Self { point, speed }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::from).collect();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(24);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 24));
        assert_eq!(result, None);
    }
}
