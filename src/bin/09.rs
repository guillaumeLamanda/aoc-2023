pub fn part_one(input: &str) -> Option<i64> {
    let suits = parse_input(input);
    suits
        .iter()
        .map(|suit| project_next_value(suit))
        .sum::<i64>()
        .into()
}

fn project_next_value(suit: &[i64]) -> i64 {
    let mut suit = suit.to_vec();
    let mut last_values = vec![*suit.last().unwrap()];
    loop {
        if is_arythmetic(&suit) {
            break;
        }
        suit = derive(&suit);
        last_values.push(*suit.last().unwrap());
    }
    last_values.iter().sum::<i64>()
}

#[test]
fn test_project_next_value() {
    fn parse_line(line: &str) -> Vec<i64> {
        line.split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>()
    }
    let line = "20 29 36 41 44 45 44 41 36 29 20 9 -4 -19 -36 -55 -76 -99 -124 -151 -180";
    let parsed_line = parse_line(line);
    assert_eq!(project_next_value(&parsed_line), -211);
}

fn project_previous_value(suit: &[i64]) -> i64 {
    let mut suit = suit.to_vec();
    let mut first_values = vec![*suit.first().unwrap()];
    loop {
        if is_arythmetic(&suit) {
            break;
        }
        suit = derive(&suit);
        first_values.push(*suit.first().unwrap());
    }
    first_values.iter().rev().fold(0, |res, &v| v - res)
}

#[test]
fn test_project_previous_value() {
    fn parse_line(line: &str) -> Vec<i64> {
        line.split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>()
    }
    let line = "10 13 16 21 30 45";
    let parsed_line = parse_line(line);
    assert_eq!(project_previous_value(&parsed_line), 5);
}

fn is_arythmetic(suit: &[i64]) -> bool {
    let first = suit.first().unwrap();
    suit.iter().all(|v| v.eq(first))
}

fn derive(suit: &[i64]) -> Vec<i64> {
    suit.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn part_two(input: &str) -> Option<i64> {
    let suits = parse_input(input);

    suits
        .iter()
        .map(|suit| project_previous_value(suit))
        .sum::<i64>()
        .into()
}

advent_of_code::main!(9);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 9));
        assert_eq!(result, Some(2));
    }
}
