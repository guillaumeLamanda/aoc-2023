use advent_of_code::map::{Map, Position};
use std::ops::Mul;

pub fn part_one(input: &str) -> Option<u32> {
    fn is_symbol(c: char) -> bool {
        if c == '.' {
            return false;
        }
        if c as u32 >= 48 && (c as u32) <= 57 {
            return false;
        }
        true
    }

    let map = Map::from(input);
    let numbers = map.get_numbers_and_position();
    let symbols = map.get_symbol_and_position(is_symbol);
    let mut result = 0;
    for (number, position) in numbers {
        for symbol in symbols.iter() {
            if is_adjacent(number, position, symbol.1) {
                result += number;
            }
        }
    }
    Some(result)
}

fn is_adjacent(number: u32, number_position: Position, symbol_position: Position) -> bool {
    let number_end_position = Position {
        x: number_position.x + number.to_string().len() - 1,
        y: number_position.y,
    };

    let adjacent_x_start = number_position.x.saturating_sub(1);
    let adjacent_x_end = number_end_position.x + 1;
    let adjacent_y_start = number_position.y.saturating_sub(1);
    let adjacent_y_end = number_end_position.y + 1;

    if symbol_position.x < adjacent_x_start || symbol_position.x > adjacent_x_end {
        return false;
    }
    if symbol_position.y < adjacent_y_start || symbol_position.y > adjacent_y_end {
        return false;
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::from(input);
    let numbers = map.get_numbers_and_position();
    map.get_symbol_and_position(|c| c == '*')
        .iter()
        .map(|(_, position)| {
            numbers
                .iter()
                .filter(|(number, number_position)| {
                    is_adjacent(*number, *number_position, *position)
                })
                .map(|(number, _)| number)
                .collect::<Vec<&u32>>()
        })
        .filter(|numbers| numbers.len() == 2)
        .map(move |v| v.iter().fold(1, |a, &b| a.mul(b)))
        .sum::<u32>()
        .into()
}

advent_of_code::main!(3);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_adjacent() {
        assert!(is_adjacent(
            1,
            Position { x: 0, y: 0 },
            Position { x: 0, y: 1 },
        ));
        assert!(is_adjacent(
            12,
            Position { x: 0, y: 0 },
            Position { x: 2, y: 0 },
        ));
        assert!(is_adjacent(
            12,
            Position { x: 0, y: 1 },
            Position { x: 0, y: 0 },
        ));
        assert!(is_adjacent(
            12,
            Position { x: 2, y: 2 },
            Position { x: 2, y: 1 },
        ));
        assert!(is_adjacent(
            12,
            Position { x: 2, y: 2 },
            Position { x: 2, y: 3 },
        ));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 3));
        assert_eq!(result, Some(467835));
    }
}
