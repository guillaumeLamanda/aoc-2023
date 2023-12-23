use std::{collections::HashMap, num::ParseIntError, str::FromStr, string::ParseError};

use rangetools::Rangetools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: usize,
    y: usize,
    z: usize,
}

impl From<(usize, usize, usize)> for Point3D {
    fn from((x, y, z): (usize, usize, usize)) -> Self {
        Point3D { x, y, z }
    }
}

impl FromStr for Point3D {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        let x = coords.next().unwrap().parse()?;
        let y = coords.next().unwrap().parse()?;
        let z = coords.next().unwrap().parse()?;
        Ok(Point3D { x, y, z })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Brick {
    edges: (Point3D, Point3D),
}

impl FromStr for Brick {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let edges = s.split_once('~').unwrap();
        let e1 = edges.0.parse()?;
        let e2 = edges.1.parse()?;
        Ok(Brick { edges: (e1, e2) })
    }
}

impl Brick {
    fn volume(&self) -> usize {
        let e1 = &self.edges.0;
        let e2 = &self.edges.1;
        (e2.x - e1.x) * (e2.y - e1.y) * (e2.z - e1.z)
    }

    fn have_contact_with(&self, other: &Brick) -> bool {
        self.get_intersection(other).is_some()
    }

    fn get_intersection(&self, other: &Brick) -> Option<((usize, usize), (usize, usize))> {
        let x_intersection =
            (self.edges.0.x..=self.edges.1.x).intersection(other.edges.0.x..=other.edges.1.x);
        let y_intersection =
            (self.edges.0.y..=self.edges.1.y).intersection(other.edges.0.y..=other.edges.1.y);
        let get_bound_value = |bound: &rangetools::Bound<usize>| match bound {
            rangetools::Bound::Included(x) => *x,
            _ => unreachable!(),
        };

        if x_intersection.is_empty() || y_intersection.is_empty() {
            return None;
        }

        let x_start = get_bound_value(&x_intersection.start.to_bound());
        let x_end = get_bound_value(&x_intersection.end.to_bound());
        let y_start = get_bound_value(&y_intersection.start.to_bound());
        let y_end = get_bound_value(&y_intersection.end.to_bound());

        Some(((x_start, x_end), (y_start, y_end)))
    }

    fn get_max_z(&self, bricks: &[Brick]) -> usize {
        bricks
            .iter()
            .rev()
            .find(|brick| brick.get_intersection(self).is_some())
            .map(|brick| brick.edges.1.z)
            .unwrap_or(0)
    }
}

#[test]
fn test_get_intersection() {
    let no_intersection = (0..=1).intersection(3..5);
    assert!(no_intersection.is_empty());

    let a = Brick::from_str("1,0,1~1,2,1").unwrap();
    let b = Brick::from_str("0,0,2~2,0,2").unwrap();
    let c = Brick::from_str("0,2,2~2,2,2").unwrap();

    assert_eq!(a.get_intersection(&b), Some(((1, 1), (0, 0))));
    assert_eq!(b.get_intersection(&c), None);
    assert_eq!(a.get_intersection(&c), Some(((1, 1), (2, 2))));
}

#[test]
fn test_brick_contact() {
    let a = Brick::from_str("1,0,1~1,2,1").unwrap();
    let b = Brick::from_str("0,0,2~2,0,2").unwrap();
    let c = Brick::from_str("0,2,2~2,2,2").unwrap();

    assert!(a.have_contact_with(&b));
    assert!(a.have_contact_with(&c));
}

#[test]
fn test_brick_volume() {
    let brick = Brick {
        edges: (Point3D { x: 0, y: 0, z: 0 }, Point3D { x: 1, y: 1, z: 1 }),
    };
    assert_eq!(brick.volume(), 1);
    let a = Brick::from_str("1,0,1~1,2,1").unwrap();
    assert_eq!(a.volume(), 2);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stack(Vec<Brick>);

impl FromStr for Stack {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bricks = s
            .lines()
            .map(|line| line.parse::<Brick>().unwrap())
            .collect::<Vec<Brick>>();
        bricks.sort_by_key(|b| b.edges.1.z);
        Ok(Stack(bricks))
    }
}

impl Stack {
    fn let_them_fall(&mut self) {
        let mut new_bricks: Vec<Brick> = Vec::new();

        for brick in self.0.iter_mut() {
            let max_z = brick.get_max_z(&new_bricks);
            let z = max_z + 1;
            let z_diff_in_edges = brick.edges.1.z - brick.edges.0.z;
            brick.edges.0.z = z;
            brick.edges.1.z = z + z_diff_in_edges;
            new_bricks.push(*brick);
        }
        self.0 = new_bricks;
    }
}

#[test]
fn test_let_them_fall() {
    let mut stack = Stack(vec![
        Brick::from_str("1,0,1~1,2,1").unwrap(),
        Brick::from_str("0,0,2~2,0,2").unwrap(),
        Brick::from_str("0,2,3~2,2,3").unwrap(),
    ]);
    stack.let_them_fall();
    assert_eq!(stack.0[0], Brick::from_str("1,0,1~1,2,1").unwrap());
    assert_eq!(stack.0[1], Brick::from_str("0,0,2~2,0,2").unwrap());
    assert_eq!(stack.0[2], Brick::from_str("0,2,2~2,2,2").unwrap());
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut stack = input.parse::<Stack>().unwrap();
    stack.let_them_fall();

    let mut supports: HashMap<&Brick, Vec<&Brick>> = HashMap::new();
    let mut supported_by: HashMap<&Brick, Vec<&Brick>> = HashMap::new();

    for (i, brick) in stack.0.iter().enumerate() {
        for other in stack
            .0
            .iter()
            .skip(i + 1)
            .filter(|o| o.edges.0.z == brick.edges.1.z + 1)
        {
            if brick.have_contact_with(other) {
                supports.entry(brick).or_default().push(other);
                supported_by.entry(other).or_default().push(brick);
            }
        }
    }

    stack
        .0
        .iter()
        .filter(|brick| {
            supports
                .get(brick)
                .map(|supported| {
                    supported
                        .iter()
                        .all(|b| supported_by.get(b).unwrap_or(&vec![]).len() > 1)
                })
                .unwrap_or(true)
        })
        .inspect(|b| {
            println!("{:?}", b);
        })
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(22);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_alt() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", 22, 1));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 22));
        assert_eq!(result, None);
    }
}
