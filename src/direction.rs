use std::str::FromStr;

use geo::Coord;

use crate::map::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s {
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            "R" => Self::Right,
            _ => return Err(()),
        };
        Ok(direction)
    }
}

impl Direction {
    pub fn apply(&self, position: &Position) -> Position {
        match self {
            Self::Up => {
                Position::from((position.x, position.y.checked_sub(1).unwrap_or(position.y)))
            }
            Self::Down => Position::from((position.x, position.y + 1)),
            Self::Left => {
                Position::from((position.x.checked_sub(1).unwrap_or(position.x), position.y))
            }
            Self::Right => Position::from((position.x + 1, position.y)),
        }
    }

    pub fn apply_on_coord(&self, coord: &Coord) -> Coord {
        match self {
            Self::Up => Coord::from((coord.x, coord.y - 1.)),
            Self::Down => Coord::from((coord.x, coord.y + 1.)),
            Self::Left => Coord::from((coord.x - 1., coord.y)),
            Self::Right => Coord::from((coord.x + 1., coord.y)),
        }
    }

    pub fn go_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn go_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn to_tuple(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }
}
