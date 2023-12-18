use std::str::FromStr;

use advent_of_code::{direction::Direction, map::Position};

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = get_instructions(input);

    let borders = get_polynom_points(instructions);

    None
}

fn get_instructions(input: &str) -> Vec<(Direction, usize, &str)> {
    let instructions: Vec<(Direction, usize, &str)> = input
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<&str>>();
            (
                Direction::from_str(parts.first().unwrap()).unwrap(),
                parts[1].parse().unwrap(),
                parts[2],
            )
        })
        .collect();
    instructions
}

fn get_polynom_points(instructions: Vec<(Direction, usize, &str)>) -> Vec<Position> {
    let borders: Vec<Position> =
        instructions
            .iter()
            .fold(vec![], |mut borders, (direction, count, _)| {
                for _ in 0..*count {
                    let default = Position::from((0, 0));
                    let last = borders.last().unwrap_or(&default);
                    borders.push(direction.apply(last));
                }
                borders
            });
    borders
}

fn get_square_limits(start: Position, borders: &[Position]) -> (Position, Position) {
    let mut get_end_of_row = |start: Position| -> Position {
        let mut end_row_position = borders
            .iter()
            .filter(|p| p.y == start.y)
            .map(|p| p.x)
            .collect::<Vec<usize>>();
        end_row_position.sort();
        let end_row_position = end_row_position
            .iter()
            .as_slice()
            .windows(2)
            .take_while(|x| x[0] == (x[1] - 1))
            .map(|x| x[1])
            .max();
        Position::from((end_row_position.unwrap(), start.y))
    };

    let mut get_end_of_column = |start: Position| -> Position {
        let mut end_column_position = borders
            .iter()
            .filter(|p| p.x == start.x)
            .map(|p| p.y)
            .collect::<Vec<usize>>();
        end_column_position.sort();
        let end_column_position = end_column_position
            .iter()
            .as_slice()
            .windows(2)
            .take_while(|y| y[0] == y[1] - 1)
            .map(|y| y[1])
            .max();
        Position::from((start.x, end_column_position.unwrap()))
    };

    let p2 = get_end_of_row(start);
    let p3 = get_end_of_column(start);
    let p4 = get_end_of_column(p2);

    let final_end_position = Position::from((p4.x, p4.y.min(p3.y)));

    let next_y = final_end_position.y + 1;
    let next_x_start = borders
        .iter()
        .filter(|p| p.y == next_y)
        .min()
        .map(|p| p.x)
        .unwrap();
    let next_start = Position::from((next_x_start, next_y));

    (final_end_position, next_start)
}

#[test]
fn test_get_squares_limits() {
    let result = advent_of_code::template::read_file("examples", 18);
    let instructions = get_instructions(&result);
    let polynomial_points = get_polynom_points(instructions);

    let result = get_square_limits(Position::from((0, 0)), &polynomial_points);
    assert_eq!(result, (Position::from((6, 2)), Position::from((2, 3))));

    let result = get_square_limits(Position::from((2, 3)), &polynomial_points);
    assert_eq!(result, (Position::from((6, 5)), Position::from((0, 5))));
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(18);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, None);
    }
}
