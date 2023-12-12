use std::{collections::HashMap, usize};

#[derive(Debug)]
struct SpringsRow {
    arrangement: Vec<usize>,
    line: String,
    parts: Vec<String>,
}

impl From<&str> for SpringsRow {
    fn from(line: &str) -> Self {
        let (line, arrangement) = line.split_once(' ').unwrap();
        let arrangement: Vec<usize> = arrangement.split(',').map(|n| n.parse().unwrap()).collect();

        Self {
            arrangement,
            line: line.to_string(),
            parts: line
                .split('.')
                .filter(|l| !l.is_empty())
                .map(String::from)
                .collect::<Vec<_>>(),
        }
    }
}

impl SpringsRow {
    fn trim_existing(&mut self) {
        match (self.parts.first(), self.arrangement.first()) {
            (Some(fs), Some(fc)) if fc == &fs.len() && fs.chars().all(|c| c == '#') => {
                self.arrangement.remove(0);
                self.parts.remove(0);
                self.trim_existing();
            }
            (_, _) => {}
        }

        match (self.parts.last(), self.arrangement.last()) {
            (Some(fs), Some(fc)) if fc == &fs.len() && fs.chars().all(|c| c == '#') => {
                self.arrangement.pop();
                self.parts.pop();
                self.trim_existing();
            }
            (_, _) => {}
        }
    }

    fn count_possibilities(&self, memo: Option<&mut HashMap<(usize, usize), usize>>) -> usize {
        let mut default_memo = HashMap::new();
        let chars: Vec<u8> = self.line.chars().map(|c| c as u8).collect();
        count_springs_possibilities(&self.arrangement, &chars, memo.unwrap_or(&mut default_memo))
    }
}

/// * `arrangment` is the list of numbers represeting continguous damaged springs.
/// * `parts` is the ruined map with many springs in unknown condition.
fn count_springs_possibilities(
    arrangment: &[usize],
    parts: &[u8],
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(&count) = memo.get(&(arrangment.len(), parts.len())) {
        count
    } else {
        let mut count = 0;
        let space = arrangment.iter().sum::<usize>();
        let limit = parts.len() - space;
        let span = arrangment[0];
        let ulen = parts.len();

        for i in 0..=limit {
            if i > 0 && parts[i - 1] == b'#' {
                break;
            }

            if parts[i..i + span].iter().all(|&b| b != b'.') {
                if arrangment.len() == 1 {
                    if parts[i + span..].iter().all(|&b| b != b'#') {
                        count += 1;
                    }
                } else if (i + span == ulen || parts[i + span] != b'#') && ulen > i + space {
                    count +=
                        count_springs_possibilities(&arrangment[1..], &parts[i + span + 1..], memo);
                }
            }
        }
        memo.insert((arrangment.len(), parts.len()), count);

        count
    }
}

#[cfg(test)]
mod springs_rows_test {
    #[test]
    fn trim_existing() {
        let mut row = super::SpringsRow::from("???.### 1,1,3");
        row.trim_existing();
        assert_eq!(row.parts, vec!["???"]);
        assert_eq!(row.arrangement, vec![1, 1]);
        let mut row = super::SpringsRow::from("????.#...#... 4,1,1");
        row.trim_existing();
        assert_eq!(row.parts, vec!["????"]);
        assert_eq!(row.arrangement, vec![4]);
        let mut row = super::SpringsRow::from("?###???????? 3,2,1");
        row.trim_existing();
        assert_eq!(row.parts, vec!["?###????????"]);
        assert_eq!(row.arrangement, vec![3, 2, 1]);
    }

    #[test]
    fn count_possibilities() {
        let row = super::SpringsRow::from("???.### 1,1,3");
        assert_eq!(row.count_possibilities(None), 1,);
        // let row = super::SpringsRow::from("?###???????? 3,2,1");
        // assert_eq!(row.count_possibilities(), 10);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut memo = HashMap::<(usize, usize), usize>::new();
    let mut count = 0;
    for line in input.lines() {
        let spring = SpringsRow::from(line);
        count += spring.count_possibilities(Some(&mut memo));
    }
    Some(count as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(12);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 12));
        assert_eq!(result, None);
    }
}
