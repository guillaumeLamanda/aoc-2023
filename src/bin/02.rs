use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Game {
    id: u32,
    throws: Vec<Vec<(u32, String)>>,
}

const COLORS_POSSIBILITIES: [(u32, &str); 3] = [(12, "red"), (13, "green"), (14, "blue")];
impl TryFrom<&str> for Game {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s.is_empty() {
            return Err("Empty string");
        }
        let (game_str, throws) = s.split_once(':').unwrap();
        let id = game_str.split_whitespace().last().unwrap().parse().unwrap();

        let throws = throws
            .split(';')
            .map(|s| s.trim())
            .map(|s| {
                s.split(',')
                    .map(|s| s.trim())
                    .map(|s| s.split_once(' ').unwrap())
                    .map(|(a, b)| (a.parse::<u32>().unwrap(), b.to_string()))
                    .collect::<Vec<(u32, String)>>()
            })
            .collect::<Vec<Vec<(u32, String)>>>();

        Ok(Self { id, throws })
    }
}

impl Game {
    fn get_power(&self) -> u32 {
        let mut max_throws: HashMap<String, u32> = HashMap::new();
        self.throws.iter().for_each(|throw| {
            throw.iter().for_each(|(number, color)| {
                max_throws
                    .entry(color.to_string())
                    .and_modify(|e| if number > e {
                        *e = *number;
                    })
                    .or_insert(*number);
            });
        });

        let mut power = 1;
        for value in max_throws.values() {
            power *= value
        }
        power
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(|line| Game::try_from(line).ok())
        .filter(|game| {
            game.throws.iter().all(|throw| {
                throw.iter().all(|(a, b)| {
                    let color = COLORS_POSSIBILITIES
                        .iter()
                        .find(|(_, color)| *color == b)
                        .unwrap();
                    a <= &color.0
                })
            })
        })
        .map(|game| game.id)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| Game::try_from(line).ok())
            .map(|game| game.get_power())
            .sum(),
    )
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", 2, 2));
        assert_eq!(result, Some(2286));
    }
}
