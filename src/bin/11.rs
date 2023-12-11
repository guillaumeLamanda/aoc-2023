use std::collections::HashMap;

use advent_of_code::map::{Map, Position};

#[derive(Debug, Clone, Copy)]
struct PositionPair(Position, Position);

impl std::hash::Hash for PositionPair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut positions = [self.0, self.1];
        positions.sort();
        positions[0].hash(state);
        positions[1].hash(state);
    }
}

impl PartialEq for PositionPair {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl Eq for PositionPair {}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use advent_of_code::map::Position;

    use crate::PositionPair;

    #[test]
    fn test_position_pair_hashing() {
        let position1 = Position::from((1, 2));
        let position2 = Position::from((3, 4));
        assert_eq!(
            PositionPair(position1, position2),
            PositionPair(position2, position1)
        );
        let mut set = HashSet::new();
        set.insert(PositionPair(position1, position2));
        set.insert(PositionPair(position2, position1));
        assert_eq!(set.len(), 1);
    }
}

fn expand_galaxy_map(map: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut expanded = vec![];
    for line in map {
        expanded.push(line.to_vec());
        if line.iter().all(|&c| c == '.') {
            expanded.push(line.to_vec());
        }
    }

    let mut number_of_cols_added = 0;
    for i in 0..map[0].len() {
        let column = expanded
            .iter()
            .map(|row| row[i + number_of_cols_added])
            .collect::<Vec<_>>();
        if column.iter().all(|&c| c == '.') {
            expanded.iter_mut().for_each(|row| {
                row.insert(i + 1 + number_of_cols_added, '.');
            });
            number_of_cols_added += 1;
        }
    }

    expanded
}

#[test]
fn test_expand_galaxy() {
    let result: &str = &advent_of_code::template::read_file("examples", 11);
    let mut galaxy = Map::from(result);
    galaxy.map = expand_galaxy_map(&galaxy.map);
    assert_eq!(galaxy.map.len(), 12);
    assert_eq!(galaxy.map[0].len(), 13);
    assert_eq!(galaxy.get_char_at_position(Position::from((5, 11))), '#')
}

fn get_expand(map: &[Vec<char>]) -> (Vec<usize>, Vec<usize>) {
    let mut rows_expansion = vec![];
    for (i, line) in map.iter().enumerate() {
        if line.iter().all(|&c| c == '.') {
            rows_expansion.push(i);
        }
    }
    let mut columns_expansion = vec![];
    for i in 0..map[0].len() {
        let column = map.iter().map(|row| row[i]).collect::<Vec<_>>();
        if column.iter().all(|&c| c == '.') {
            columns_expansion.push(i);
        }
    }
    (rows_expansion, columns_expansion)
}

#[test]
fn test_get_expand() {
    let result: &str = &advent_of_code::template::read_file("examples", 11);
    let galaxy = Map::from(result);
    let (rows_expansions, columns_expansions) = get_expand(&galaxy.map);
    assert_eq!(rows_expansions.len(), 2);
    assert_eq!(rows_expansions, vec![3, 7]);
    assert_eq!(columns_expansions.len(), 3);
    assert_eq!(columns_expansions, vec![2, 5, 8]);
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut galaxy = Map::from(input);
    galaxy.map = expand_galaxy_map(&galaxy.map);
    let planets = galaxy.get_symbol_and_position(|c| c == '#');
    let mut distances: HashMap<PositionPair, u64> = HashMap::new();
    for (_, planet_position) in &planets {
        for (_, other_position) in &planets {
            if planet_position == other_position {
                continue;
            }
            let position_pair = PositionPair(*planet_position, *other_position);
            let distance = planet_position.distance(other_position);
            let value = distances.entry(position_pair).or_insert(distance);
            if distance < *value {
                *value = distance;
            }
        }
    }
    Some(distances.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let galaxy = Map::from(input);
    let expands = get_expand(&galaxy.map);
    let planets = galaxy.get_symbol_and_position(|c| c == '#');
    let planets: Vec<Position> = planets
        .iter()
        .map(|(_, position)| compute_new_position_with_extensions(position, &expands, 1000000))
        .collect::<Vec<_>>();

    let mut distance = 0;
    for (i, planet) in planets.iter().enumerate() {
        for y in &planets[i + 1..] {
            distance += planet.distance(y);
        }
    }

    Some(distance)
}

fn compute_new_position_with_extensions(
    position: &Position,
    expands: &(Vec<usize>, Vec<usize>),
    size: usize,
) -> Position {
    let x_extension = expands.1.iter().take_while(|&&i| i < position.x).count();
    let y_extension = expands.0.iter().take_while(|&&i| i < position.y).count();
    Position {
        x: position.x + (x_extension * (size - 1)),
        y: position.y + (y_extension * (size - 1)),
    }
}

#[test]
fn test_compute_new_position_with_extensions() {
    let rows_expansion = vec![3];
    let columns_expansions = vec![2];
    let position = Position::from((3, 0));
    assert_eq!(
        compute_new_position_with_extensions(&position, &(rows_expansion, columns_expansions), 10),
        Position::from((12, 0))
    );
}

advent_of_code::main!(11);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn distances() {
        let result: &str = &advent_of_code::template::read_file("examples", 11);
        let mut galaxy = Map::from(result);
        galaxy.map = expand_galaxy_map(&galaxy.map);
        let planets = galaxy.get_symbol_and_position(|c| c == '#');
        let planets_with_id = planets
            .iter()
            .enumerate()
            .map(|(id, (_, position))| (id + 1, position))
            .collect::<Vec<_>>();
        let first = planets_with_id.iter().find(|(id, _)| id == &1).unwrap().1;
        let second = planets_with_id.iter().find(|(id, _)| id == &7).unwrap().1;
        assert_eq!(
            first.distance(second),
            15,
            "{:?} - {:?} != 15",
            first,
            second
        );
        let first = planets_with_id.iter().find(|(id, _)| id == &3).unwrap().1;
        let second = planets_with_id.iter().find(|(id, _)| id == &6).unwrap().1;
        assert_eq!(
            first.distance(second),
            17,
            "{:?} - {:?} != 17",
            first,
            second
        );
    }

    #[test]
    fn test_part_two_10() {
        // each expansion is one million size
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_10_2() {
        // each expansion is one million size
        let result = part_two(&advent_of_code::template::read_file("examples", 11));
        assert_eq!(result, Some(8410));
    }
}
