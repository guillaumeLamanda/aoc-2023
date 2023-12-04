pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(':').unwrap().1)
        .map(|game| {
            let (scores, winning_numbers) = game.split_once('|').unwrap();
            let scores: Vec<u32> = scores
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let winning_numbers: Vec<u32> = winning_numbers
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();

            let mut score = 0;
            for game_score in scores {
                if !winning_numbers.contains(&game_score) {
                    continue;
                }
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
            score
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards_score_counts = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once(':').unwrap().1)
        .map(|game| {
            let (scores, winning_numbers) = game.split_once('|').unwrap();
            let scores: Vec<u32> = scores
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let winning_numbers: Vec<u32> = winning_numbers
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
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
