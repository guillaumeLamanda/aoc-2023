use std::collections::HashSet;

use advent_of_code::{direction::Direction, map::Position};
use pathfinding::{
    directed::dfs::{dfs, dfs_reach},
    matrix::Matrix,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Rock,
    GardenPlot,
    StartingPosition,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::GardenPlot,
            '#' => Tile::Rock,
            'S' => Tile::StartingPosition,
            _ => unreachable!(),
        }
    }
}

const LENGTH: usize = 6;

pub fn part_one(input: &str) -> Option<usize> {
    let matrix = Matrix::from_rows(
        input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<Tile>>())
            .collect::<Vec<_>>(),
    )
    .unwrap();

    let starting_position = matrix
        .items()
        .find(|(_, &tile)| tile.eq(&Tile::StartingPosition))
        .map(|a| a.0)
        .unwrap();

    dfs_reach(
        (Position::from(starting_position), 0),
        |(position, length)| {
            if length == &LENGTH {
                return vec![];
            }
            let mut next_positions = vec![];
            for direction in [
                Direction::Up,
                Direction::Left,
                Direction::Down,
                Direction::Right,
            ] {
                let next_position = direction.apply(position);
                if matrix
                    .get(next_position.to_tuple())
                    .map(|&tile| !tile.eq(&Tile::Rock))
                    .unwrap_or(false)
                {
                    next_positions.push((next_position, length + 1));
                }
            }
            next_positions
        },
    )
    .filter(|(_, length)| length == &LENGTH)
    .count()
    .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(21);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 21));
        assert_eq!(result, None);
    }
}
