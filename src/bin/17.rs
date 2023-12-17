use std::collections::{BTreeSet, HashSet};

use advent_of_code::{
    direction::Direction,
    map::{Map, Position},
};

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::<u32>::from_numbers(input);

    let start_position = Position::from((0, 0));
    let end_position = Position::from((map.map.len() - 1, map.map[0].len() - 1));
    println!("{} {}", start_position, end_position);

    Some(find_path(start_position, end_position, map))
}

fn find_path(start_position: Position, end_position: Position, map: Map<u32>) -> u32 {
    let mut candidates = BTreeSet::<(u32, Position, Direction)>::default();
    candidates.insert((0, start_position, Direction::Right));
    let mut seen = HashSet::<(Position, Direction)>::new();

    while let Some((heat_loss, position, direction)) = candidates.pop_first() {
        if position == end_position {
            return heat_loss;
        }

        // dikstra's algorithm
        if !seen.insert((position, direction)) {
            continue;
        }
        println!("{}: {} {:?}", position, heat_loss, direction);

        let mut next_directions = vec![direction.go_left(), direction.go_right()];
        if position == start_position {
            next_directions.push(direction);
        }

        for direction in next_directions {
            let mut hl = heat_loss;
            let mut new_position = position;
            for _ in 0..3 {
                new_position = direction.apply(&new_position);
                if map.is_out_of_bounds(new_position) {
                    break;
                }
                hl += map.get(new_position);
                candidates.insert((hl, new_position, direction));
            }
        }
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(17);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 17));
        assert_eq!(result, None);
    }
}
