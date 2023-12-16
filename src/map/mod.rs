use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn distance(&self, other: &Position) -> u64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y))
            .try_into()
            .unwrap()
    }
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
pub struct Map<T> {
    pub map: Vec<Vec<T>>,
}

impl<T> From<&str> for Map<T>
where
    T: From<char>,
{
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().map(|c| T::from(c)).collect())
            .collect::<Vec<_>>();
        Self { map }
    }
}

impl<T> FromStr for Map<T>
where
    T: From<char>,
{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<T> Map<T>
where
    T: Copy,
{
    pub fn get_symbol_and_position(&self, is_symbol: fn(T) -> bool) -> Vec<(T, Position)> {
        let mut result: Vec<(T, Position)> = Vec::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if is_symbol(*c) {
                    result.push((*c, Position { x, y }));
                }
            }
        }
        result
    }

    pub fn get(&self, p: Position) -> T {
        self.map[p.y][p.x]
    }

    pub fn is_out_of_bounds(&self, p: Position) -> bool {
        p.x >= self.map[0].len() || p.y >= self.map.len()
    }

    pub fn get_adjacents(&self, position: Position) -> Vec<(T, Position)> {
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

    pub fn transpose(&mut self) -> &Vec<Vec<T>> {
        self.map = transpose(&self.map);
        &self.map
    }
}

// from https://stackoverflow.com/a/64499219/10558013
pub fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

impl Map<char> {
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
