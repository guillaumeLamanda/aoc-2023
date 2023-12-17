use crate::map::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right,
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
}
