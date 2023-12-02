use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Game {
    id: u32,
}

const COLORS_POSSIBILITIES: [(u32, &str);3] = [(12, "red"), (13, "green"), (14, "blue")];
impl TryFrom<&str> for Game {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err("Empty string");
        }
        let (game_str, throws) = s.split_once(':').unwrap();
        let id = game_str.split_whitespace().last().unwrap().parse().unwrap();

        let throws = throws.split(';')
            .map(|s| s.trim())
            .map(|s|
                s.split(',')
                .map(|s| s.trim())
                .map(|s| s.split_once(' ').unwrap())
                .map(|(a, b)| (a.parse::<u32>().unwrap(), b))
                .collect::<Vec<(u32, &str) >>()
            )
            .collect::<Vec<Vec<(u32, &str) >>>();

        for throw in throws {
            for (a, b) in throw {
                let color = COLORS_POSSIBILITIES.iter().find(|(_, color)| color == &b).unwrap();
                if a.gt(&color.0) {
                    return Err(color.1);
                }
            }
        }

        Ok(Self {
            id
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input.lines()
        .filter_map(
        |line| Game::try_from(line).ok()
    )
        .map(|game| game.id)
        .sum::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(2);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2));
        assert_eq!(result, None);
    }
}
