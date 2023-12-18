use std::{
    collections::HashSet,
    ops::{Add, Div},
    str::FromStr,
};

use advent_of_code::{direction::Direction, map::Position};
use geo::{area::Area, polygon, Coord, CoordNum, GeodesicArea, LineString, Polygon};

pub fn part_one(input: &str) -> Option<f64> {
    let instructions = get_instructions(input);
    let borders = get_polynom_points(instructions);
    let perimeter = &borders.len();
    let polygon = Polygon::new(LineString::from(borders), vec![]);
    let area = polygon.unsigned_area();

    // 34771
    println!("area {} ,perimeter: {}", area, perimeter);

    Some(area + perimeter.div(2).add(1) as f64)
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

fn get_polynom_points(instructions: Vec<(Direction, usize, &str)>) -> Vec<Coord> {
    instructions
        .iter()
        .fold(vec![], |mut borders, (direction, count, _)| {
            for _ in 0..*count {
                let default = Coord::from((0., 0.));
                let last = borders.last().unwrap_or(&default);
                borders.push(direction.apply_on_coord(last));
            }
            borders
        })
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
        assert_eq!(result, Some(62.));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 18));
        assert_eq!(result, None);
    }
}
