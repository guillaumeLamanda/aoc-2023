use std::collections::{HashMap, HashSet};

pub fn part_one(input: &str) -> Option<u32> {
    let edges = input
        .lines()
        .flat_map(|line| {
            let (source, destinations) = line.split_once(": ").unwrap();
            destinations
                .split(", ")
                .map(|destination| (source, destination))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut graph = HashMap::<&str, HashSet<&str>>::new();
    for (source, destination) in edges {
        graph.entry(source).or_default().insert(destination);
        graph.entry(destination).or_default().insert(source);
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(25);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 25));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 25));
        assert_eq!(result, None);
    }
}
