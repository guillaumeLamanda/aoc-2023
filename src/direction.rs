use crate::map::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
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
}
