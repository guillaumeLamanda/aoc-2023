use std::ops::Mul;

pub fn part_one(input: &str) -> Option<u32> {
    build_scores_and_winning_numbers(input)
        .iter()
        .map(|(scores, winning_numbers)| {
            scores
                .iter()
                .filter(|s| winning_numbers.contains(s))
                .collect::<Vec<&u32>>()
        })
        .map(|scores| {
            scores
                .iter()
                .fold(0, |acc, _b| if acc == 0 { 1 } else { acc.mul(2) })
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_score_counts = build_scores_and_winning_numbers(input)
        .iter()
        .map(|(scores, winning_numbers)| {
            scores
                .iter()
                .filter(|score| winning_numbers.contains(score))
                .count()
        })
        .map(|n| (n, 1))
        .collect::<Vec<(usize, usize)>>();

    for i in 0..cards_score_counts.len() {
        if cards_score_counts[i].0 == 0 {
            continue;
        }
        let copies_won = cards_score_counts[i];
        (i + 1..=i + copies_won.0).for_each(|index| {
            cards_score_counts[index].1 += copies_won.1;
        });
    }

    cards_score_counts
        .iter()
        .map(|&n| n.1 as u32)
        .sum::<u32>()
        .into()
}

fn build_scores_and_winning_numbers(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(':').unwrap().1)
        .filter_map(|game| game.split_once('|'))
        .map(|(scores, winning_numbers)| {
            (
                scores
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<u32>>(),
                winning_numbers
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect::<Vec<u32>>(),
            )
        })
        .collect()
}

advent_of_code::main!(4);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 4));
        assert_eq!(result, Some(30));
    }
}
