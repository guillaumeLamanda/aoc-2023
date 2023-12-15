use std::collections::HashMap;

use advent_of_code::map::{transpose, Map};

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq, Hash)]
enum Item {
    Rounded,
    Empty,
    Square,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            'O' => Self::Rounded,
            '#' => Self::Square,
            _ => unreachable!("Cannot parse {} into Item", c),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let map: Map<Item> = Map::from(input);
    let binding = transpose(&map.map);
    let mut memo = HashMap::<Vec<Item>, Vec<Item>>::new();
    let r: Vec<Vec<&Item>> = binding
        .iter()
        .filter_map(|col| -> Option<_> {
            let binding = col.iter().collect::<Vec<_>>();
            let x = binding
                .split(|&&c| c == Item::Square)
                .map(|f| {
                    let mut col = f.to_vec().clone();
                    col.sort();
                    col
                })
                .reduce(|mut accu, mut f| {
                    accu.push(&Item::Square);
                    accu.append(&mut f);
                    accu
                });
            x
        })
        .collect();

    r.iter()
        .map(|column| {
            let num_cols = column.len();
            column
                .iter()
                .enumerate()
                .filter(|item| item.1 == &&Item::Rounded)
                .map(|(i, _)| i)
                .map(|i| num_cols - i)
                .sum::<usize>()
        })
        .sum::<usize>()
        .into()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(14);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 14));
        assert_eq!(result, None);
    }
}
