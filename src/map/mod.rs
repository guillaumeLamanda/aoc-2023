#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut split = value.split(',');
        let x = split.next().unwrap().parse::<usize>().unwrap();
        let y = split.next().unwrap().parse::<usize>().unwrap();
        Self { x, y }
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
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

    pub fn get_char_at_position(&self, p: Position) -> char {
        self.map[p.y][p.x]
    }

    pub fn get_adjacents(&self, position: Position) -> Vec<(char, Position)> {
        let mut adjacents = vec![];
        if position.y > 0 {
            let north = self.map[position.y - 1][position.x];
            adjacents.push((
                north,
                Position {
                    x: position.x,
                    y: position.y - 1,
                },
            ));
        }
        if position.x > 0 {
            let west = self.map[position.y][position.x - 1];
            adjacents.push((
                west,
                Position {
                    x: position.x - 1,
                    y: position.y,
                },
            ));
        }
        if position.x < self.map[0].len() - 1 {
            let east = self.map[position.y][position.x + 1];
            adjacents.push((
                east,
                Position {
                    x: position.x + 1,
                    y: position.y,
                },
            ));
        }
        if position.y < self.map.len() - 1 {
            let south = self.map[position.y + 1][position.x];
            adjacents.push((
                south,
                Position {
                    x: position.x,
                    y: position.y + 1,
                },
            ));
        }
        adjacents
    }
}

#[test]
fn test_get_adjacents() {
    let map = Map::from("123\n456\n789");
    let adjacents = map.get_adjacents(Position { x: 0, y: 0 });
    assert!(adjacents.contains(&('2', Position { x: 1, y: 0 })));
    assert!(adjacents.contains(&('4', Position { x: 0, y: 1 })));
    let d = map.get_adjacents(Position { x: 1, y: 1 });
    assert!(d.contains(&('2', Position { x: 1, y: 0 })));
    assert!(d.contains(&('4', Position { x: 0, y: 1 })));
    assert!(d.contains(&('6', Position { x: 2, y: 1 })));
    assert!(d.contains(&('8', Position { x: 1, y: 2 })));
    let d = map.get_adjacents(Position { x: 2, y: 2 });
    assert!(d.contains(&('6', Position { x: 2, y: 1 })));
    assert!(d.contains(&('8', Position { x: 1, y: 2 })));
    let d = map.get_adjacents(Position { x: 2, y: 0 });
    assert!(d.contains(&('2', Position { x: 1, y: 0 })));
    assert!(d.contains(&('6', Position { x: 2, y: 1 })));
    let d = map.get_adjacents(Position { x: 0, y: 2 });
    assert!(d.contains(&('4', Position { x: 0, y: 1 })));
    assert!(d.contains(&('8', Position { x: 1, y: 2 })));
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
            let number: String = line
                .iter()
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
