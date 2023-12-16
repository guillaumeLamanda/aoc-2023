use std::collections::HashSet;

use advent_of_code::map::Position;
use num_traits::CheckedSub;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn apply(&self, position: &Position) -> Position {
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

#[derive(Debug, Clone, Copy)]
enum SplitterDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Mirror(Direction),
    Splitter(SplitterDirection),
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::Mirror(Direction::Right),
            '\\' => Self::Mirror(Direction::Left),
            '|' => Self::Splitter(SplitterDirection::Vertical),
            '-' => Self::Splitter(SplitterDirection::Horizontal),
            _ => panic!("Invalid tile"),
        }
    }
}

impl Tile {
    fn next(&self, direction: &Direction) -> (Direction, Option<Direction>) {
        match self {
            Tile::Empty => (*direction, None),
            Tile::Mirror(Direction::Right) => match direction {
                Direction::Up => (Direction::Right, None),
                Direction::Down => (Direction::Left, None),
                Direction::Left => (Direction::Down, None),
                Direction::Right => (Direction::Up, None),
            },
            Tile::Mirror(Direction::Left) => match direction {
                Direction::Up => (Direction::Left, None),
                Direction::Down => (Direction::Right, None),
                Direction::Left => (Direction::Up, None),
                Direction::Right => (Direction::Down, None),
            },
            Tile::Mirror(_) => panic!("Invalid direction"),
            Tile::Splitter(SplitterDirection::Vertical) => match direction {
                Direction::Left => (Direction::Down, Some(Direction::Up)),
                Direction::Right => (Direction::Up, Some(Direction::Down)),
                _ => (*direction, None),
            },
            Tile::Splitter(SplitterDirection::Horizontal) => match direction {
                Direction::Up => (Direction::Left, Some(Direction::Right)),
                Direction::Down => (Direction::Left, Some(Direction::Right)),
                _ => (*direction, None),
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = advent_of_code::map::Map::<Tile>::from(input);

    let mut visited_tiles = vec![];
    let mut stack = vec![(Position::from((0, 0)), Direction::Right)];

    loop {
        if stack.is_empty() {
            break;
        }
        let (position, direction) = stack.pop().unwrap();
        visited_tiles.push((position, direction));

        let tile = map.get(position);
        let (next_direction, next_direction_alternative) = tile.next(&direction);
        let next_position = next_direction.apply(&position);
        let already_visited = visited_tiles
            .iter()
            .any(|(p, d)| p == &next_position && d == &next_direction);
        if next_position != position && !map.is_out_of_bounds(next_position) && !already_visited {
            stack.push((next_position, next_direction));
        }

        if let Some(next_direction_alternative) = next_direction_alternative {
            let next_position = next_direction_alternative.apply(&position);
            let already_visited = visited_tiles
                .iter()
                .any(|(p, d)| p == &next_position && d == &next_direction);
            if next_position != position && !map.is_out_of_bounds(next_position) && !already_visited
            {
                stack.push((next_position, next_direction_alternative));
            }
        }
    }

    let visited_tiles: HashSet<Position> = visited_tiles.into_iter().map(|(p, _)| p).collect();

    Some(visited_tiles.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(16);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 16));
        assert_eq!(result, None);
    }

    #[test]
    fn test_direction_apply() {
        assert_eq!(
            Direction::Up.apply(&Position { x: 0, y: 0 }),
            Position { x: 0, y: 0 }
        );
    }

    #[test]
    fn test_next_tile() {
        // .x.
        // >/
        let direction = Direction::Right;
        let tile = Tile::Mirror(Direction::Right);
        assert_eq!(tile.next(&direction), (Direction::Up, None));
    }
}
