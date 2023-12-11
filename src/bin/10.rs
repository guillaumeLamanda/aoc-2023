use advent_of_code::map::{Map, Position};

const VERTICAL: char = '|';
const HORIZONTAL: char = '-';
const NORTH_TO_EST: char = 'L';
const NORTH_TO_WEST: char = 'J';
const SOUTH_TO_WEST: char = '7';
const SOUTH_TO_EST: char = 'F';
const STARTING_POSITION: char = 'S';
const GROUND: char = '.';

fn get_next_position(direction: char, position: Position, previous_position: Position) -> Position {
    match direction {
        VERTICAL if position.y + 1 == previous_position.y => Position {
            x: position.x,
            y: position.y - 1,
        },
        VERTICAL => Position {
            x: position.x,
            y: position.y + 1,
        },
        HORIZONTAL if position.x + 1 == previous_position.x => Position {
            x: position.x - 1,
            y: position.y,
        },
        HORIZONTAL => Position {
            x: position.x + 1,
            y: position.y,
        },
        NORTH_TO_EST if position.x + 1 == previous_position.x => Position {
            x: position.x,
            y: position.y - 1,
        },
        NORTH_TO_EST => Position {
            x: position.x + 1,
            y: position.y,
        },
        NORTH_TO_WEST if position.x - 1 == previous_position.x => Position {
            x: position.x,
            y: position.y - 1,
        },
        NORTH_TO_WEST => Position {
            x: position.x - 1,
            y: position.y,
        },
        SOUTH_TO_WEST if position.x - 1 == previous_position.x => Position {
            x: position.x,
            y: position.y + 1,
        },
        SOUTH_TO_WEST => Position {
            x: position.x - 1,
            y: position.y,
        },
        SOUTH_TO_EST if position.x + 1 == previous_position.x => Position {
            x: position.x,
            y: position.y + 1,
        },
        SOUTH_TO_EST => Position {
            x: position.x + 1,
            y: position.y,
        },
        _ => panic!("Invalid direction"),
    }
}

#[test]
fn test_get_next_position() {
    assert_eq!(
        get_next_position(VERTICAL, Position { x: 0, y: 1 }, Position { x: 0, y: 0 }),
        Position { x: 0, y: 2 }
    );
    assert_eq!(
        get_next_position(VERTICAL, Position { x: 0, y: 1 }, Position { x: 0, y: 2 }),
        Position { x: 0, y: 0 }
    );
    assert_eq!(
        get_next_position(HORIZONTAL, Position { x: 1, y: 0 }, Position { x: 2, y: 0 }),
        Position { x: 0, y: 0 }
    );
    assert_eq!(
        get_next_position(HORIZONTAL, Position { x: 1, y: 0 }, Position { x: 2, y: 0 }),
        Position { x: 0, y: 0 }
    );
    assert_eq!(
        get_next_position(
            NORTH_TO_EST,
            Position { x: 0, y: 1 },
            Position { x: 0, y: 0 }
        ),
        Position { x: 1, y: 1 }
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::from(input);
    Some((get_polygon(&map).len() as u32 - 1) / 2)
}

fn get_polygon(map: &Map) -> Vec<Position> {
    let starting_position = map
        .get_symbol_and_position(|c| c == STARTING_POSITION)
        .first()
        .unwrap()
        .1;
    let mut polygon: Vec<Position> = vec![starting_position];

    let mut previous_position = starting_position;
    let mut position = get_next_position_from_start(starting_position, map).unwrap();
    polygon.push(position);

    while position != starting_position {
        let c = map.get_char_at_position(position);
        let next_position = get_next_position(c, position, previous_position);
        previous_position = position;
        position = next_position;
        polygon.push(position);
    }
    polygon
}

fn get_next_position_from_start(start: Position, map: &Map) -> Option<Position> {
    let adjacents = map.get_adjacents(start);
    adjacents
        .iter()
        .filter(|(c, _)| *c != GROUND)
        .map(|(_, position)| *position)
        .last()
}

fn ray_casting(polygon_line_points: &[&Position], line: &str) -> usize {
    let mut is_inside = false;
    let mut previous_char = None;
    let mut count = 0;
    for (x, char) in line.chars().enumerate() {
        if !polygon_line_points.iter().any(|p| p.x == x) {
            if is_inside {
                count += 1;
                previous_char = None;
            }
            continue;
        }
        if char == '-' {
            continue;
        }
        if char == '|' {
            is_inside = !is_inside;
        }
        if previous_char == Some('L') && char == '7' {
            is_inside = !is_inside;
        }
        if previous_char == Some('F') && char == 'J' {
            is_inside = !is_inside;
        }
        previous_char = Some(char);
    }

    count
}

#[test]
fn test_is_inside_alternate() {
    let result: &str = &advent_of_code::template::read_file_part("examples", 10, 4);
    let map = Map::from(result);
    let polygon = get_polygon(&map);
    let get_line_info = |line: usize| -> (Vec<&Position>, String) {
        (
            polygon.iter().filter(|p| p.y == line).collect::<Vec<_>>(),
            map.map[line].iter().collect::<String>(),
        )
    };
    let (polygon_points_for_line, line) = get_line_info(3);
    assert_eq!(
        ray_casting(&polygon_points_for_line, &line),
        1,
        "Error on {:?}",
        line
    );
}

#[test]
fn test_is_inside() {
    let result: &str = &advent_of_code::template::read_file_part("examples", 10, 3);
    let map = Map::from(result);
    let polygon = get_polygon(&map);
    let get_line_info = |line: usize| -> (Vec<&Position>, String) {
        (
            polygon.iter().filter(|p| p.y == line).collect::<Vec<_>>(),
            map.map[line].iter().collect::<String>(),
        )
    };
    let (polygon_points_for_line, line) = get_line_info(3);
    assert_eq!(
        ray_casting(&polygon_points_for_line, &line),
        1,
        "Error on {:?}",
        line
    );
    let (polygon_points_for_line, line) = get_line_info(4);
    assert_eq!(
        ray_casting(&polygon_points_for_line, &line),
        3,
        "Error on {:?}",
        line
    );
    let (polygon_points_for_line, line) = get_line_info(5);
    assert_eq!(
        ray_casting(&polygon_points_for_line, &line),
        2,
        "Error on {:?}",
        line
    );
    let (polygon_points_for_line, line) = get_line_info(6);
    assert_eq!(
        ray_casting(&polygon_points_for_line, &line),
        2,
        "Error on {:?}",
        line
    );
    let position = Position::from((20, 3));
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = Map::from(input);
    let polygon = get_polygon(&map);

    let mut count = 0;
    for (y, line) in input.lines().enumerate() {
        let revelent_polygon_points = polygon.iter().filter(|p| p.y == y).collect::<Vec<_>>();
        count += ray_casting(&revelent_polygon_points, line);
    }

    Some(count as u64)
}

advent_of_code::main!(10);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 10));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_alternative() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", 10, 1));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 10, 2));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_alternate() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 10, 3));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_alternate_2() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 10, 4));
        assert_eq!(result, Some(10));
    }
}
