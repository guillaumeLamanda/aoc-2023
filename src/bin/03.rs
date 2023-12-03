use std::ops::Mul;

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>();
        Self { map }
    }
}


impl Map {
    fn get_number_and_position(&self) -> Vec<(u32, Position)> {
        let mut result: Vec<(u32, Position)> = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            let numbers_in_row = get_numbers_from_line(row.clone());
            for (x, number) in numbers_in_row {
                result.push((number, Position { x, y }));
            }
        }
        result
    }

    fn get_symbol_and_position(&self) -> Vec<(char, Position)> {
        let mut result: Vec<(char, Position)> = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if is_symbol(*c) {
                    result.push((*c, Position { x, y }));
                }
            }
        }
        result
    }
}

fn is_symbol(c: char) -> bool {
    if c == '.' {
        return false;
    }
    if c as u32 >= 48 && (c as u32) <= 57 {
        return false;
    }
    true
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}


fn get_numbers_from_line(line: Vec<char>) -> Vec<(usize, u32)> {
    line.iter()
        .enumerate()
        .filter_map(|(position, c)| {
            if !c.is_ascii_digit() {
                return None;
            }
            if position != 0 {
                let previous_char = line.get(position - 1).unwrap();
                if previous_char.is_ascii_digit() {
                    return None;
                }

            }
            let number: String = line.iter()
                .skip(position)
                .take_while(|c| c.is_ascii_digit())
                .collect();
            Some((position, number.parse::<u32>().unwrap()))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    let numbers = map.get_number_and_position();
    let symbols = map.get_symbol_and_position();
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
    let numbers = map.get_number_and_position();
    let symbols: Vec<(char, Position)> = map.get_symbol_and_position();
    let res = symbols
        .iter()
        .filter(|(c, _)| c == &'*')
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
        .sum::<u32>();

    Some(res)
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
    fn test_get_numbers_from_line() {
        assert_eq!(
            get_numbers_from_line("467..114..".chars().collect()),
            vec![(0, 467), (5, 114)]
        );
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
