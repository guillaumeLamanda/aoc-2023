use std::{collections::HashMap, u64};

use num::integer::lcm;

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let instructions: Vec<_> = lines.next().unwrap().chars().collect();
    lines.next();

    let map: HashMap<&str, (&str, &str)> = lines
        .filter_map(|line| {
            let (source, possibilities) = line.split_once(" = ")?;
            let possibilities = possibilities
                .strip_prefix('(')?
                .strip_suffix(')')?
                .split_once(", ")?;

            Some((source, possibilities))
        })
        .collect::<HashMap<_, _>>();

    let mut current = "AAA";
    let mut count = 0;
    while current != "ZZZ" {
        for instruction in &instructions {
            match instruction {
                'L' => current = map.get(current)?.0,
                'R' => current = map.get(current)?.1,
                _ => unreachable!(),
            };
            count += 1;
            if current == "ZZZ" {
                break;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let instructions: Vec<_> = lines.next().unwrap().chars().collect();
    lines.next();
    let nodes: HashMap<&str, (&str, &str)> = lines
        .filter_map(|line| {
            let (source, possibilities) = line.split_once(" = ")?;
            let possibilities = possibilities
                .strip_prefix('(')?
                .strip_suffix(')')?
                .split_once(", ")?;
            Some((source, possibilities))
        })
        .collect::<HashMap<_, _>>();

    let starting_nodes = nodes
        .keys()
        .filter(|s| s.ends_with('A'))
        .collect::<Vec<&&str>>();

    starting_nodes
        .iter()
        .map(|s| {
            let mut count: u64 = 0;
            let mut current_node = **s;
            loop {
                if current_node.ends_with('Z') {
                    break;
                }
                let instruction = instructions[count as usize % instructions.len()];
                match instruction {
                    'L' => current_node = nodes.get(current_node).unwrap().0,
                    'R' => current_node = nodes.get(current_node).unwrap().1,
                    _ => unreachable!(),
                };
                count += 1;
            }
            count
        })
        .fold(1, lcm)
        .into()
}

advent_of_code::main!(8);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 8));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 8, 2));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", 8));
        assert_eq!(result, Some(14449445933179));
    }
}
