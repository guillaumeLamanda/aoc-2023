
#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Map {
    pub map: Vec<Vec<char>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>();
        Self { map }
    }
}


impl Map {
    pub fn get_numbers_and_position(&self) -> Vec<(u32, Position)> {
        let mut result: Vec<(u32, Position)> = Vec::new();

        for (y, row) in self.map.iter().enumerate() {
            let numbers_in_row = get_numbers_from_line(row.clone());
            for (x, number) in numbers_in_row {
                result.push((number, Position { x, y }));
            }
        }
        result
    }

    pub fn get_symbol_and_position(&self, is_symbol: fn(char) -> bool) -> Vec<(char, Position)> {
        let mut result: Vec<(char, Position)> = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if is_symbol(*c) {
                    result.push((*c, Position { x, y }));
                }
            }
        }
        result
    }
}

fn get_numbers_from_line(line: Vec<char>) -> Vec<(usize, u32)> {
    line.iter()
        .enumerate()
        .filter_map(|(position, c)| {
            if !c.is_ascii_digit() {
                return None;
            }
            if position != 0 {
                let previous_char = line.get(position - 1).unwrap();
                if previous_char.is_ascii_digit() {
                    return None;
                }

            }
            let number: String = line.iter()
                .skip(position)
                .take_while(|c| c.is_ascii_digit())
                .collect();
            Some((position, number.parse::<u32>().unwrap()))
        })
        .collect()
}

#[test]
fn test_get_numbers_from_line() {
    assert_eq!(
        get_numbers_from_line("467..114..".chars().collect()),
        vec![(0, 467), (5, 114)]
    );
}
