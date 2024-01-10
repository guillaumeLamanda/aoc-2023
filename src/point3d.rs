use std::num::ParseIntError;

use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
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
