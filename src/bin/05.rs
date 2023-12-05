use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
struct AlmanacMap {
    source: (u32, u32),
    destination: (u32, u32),
    range: u32,
}

fn build_maps(lines: Vec<&str>) -> Result<Vec<Vec<AlmanacMap>>, String> {
    let mut maps: Vec<Vec<AlmanacMap>> = vec![];
    let mut current_map: Vec<AlmanacMap> = vec![];
    for line in lines.clone().iter().skip(1) {
        if line.is_empty() {
            if !current_map.is_empty() {
                maps.push(current_map);
                current_map = vec![];
            }
            continue;
        }
        if !line.chars().next().unwrap().is_numeric() {
            continue;
        }
        let almanac_map = AlmanacMap::from_str(line)?;
        current_map.push(almanac_map);
    }
    if !current_map.is_empty() {
        maps.push(current_map);
    }
    assert!(maps.len() == 7);
    Ok(maps)
}

mod part_one {
    use std::str::FromStr;

    use crate::{build_maps, AlmanacMap};

    pub struct Almanac {
        pub seeds: Vec<u32>,
        pub maps: Vec<Vec<AlmanacMap>>,
    }

    impl FromStr for Almanac {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut lines = s.lines();
            let seeds = lines
                .nth(0)
                .unwrap()
                .split_whitespace()
                .flat_map(|s| s.parse())
                .collect::<Vec<u32>>();
            Ok(Almanac {
                seeds,
                maps: build_maps(lines.collect())?,
            })
        }
    }
}

mod part_two {
    use std::str::FromStr;

    use crate::{build_maps, AlmanacMap};

    pub struct Almanac {
        pub seeds: Vec<(u32, u32)>,
        pub maps: Vec<Vec<AlmanacMap>>,
    }

    impl FromStr for Almanac {
        type Err = String;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut lines = s.lines();
            let seeds = lines
                .nth(0)
                .unwrap()
                .split_whitespace()
                .flat_map(|s| s.parse())
                .collect::<Vec<u32>>()
                .as_slice()
                .chunks(2)
                .map(|chunk| (chunk[0], chunk[1]))
                .collect::<Vec<(u32, u32)>>();
            Ok(Almanac {
                seeds,
                maps: build_maps(lines.collect())?,
            })
        }
    }
}

impl FromStr for AlmanacMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split_whitespace()
            .flat_map(|s| s.parse::<u32>())
            .collect::<Vec<u32>>();
        let destination_range_start = parts.first().unwrap();
        let source_range_start = parts.get(1).unwrap();
        let range = parts.get(2).unwrap();
        Ok(AlmanacMap {
            source: (
                *source_range_start,
                source_range_start.wrapping_add(range.wrapping_sub(1)),
            ),
            destination: (
                *destination_range_start,
                destination_range_start.wrapping_add(range.wrapping_sub(1)),
            ),
            range: *range,
        })
    }
}

#[test]
fn test_almanac_map_from_str() {
    assert_eq!(
        AlmanacMap::from_str("50 98 2").unwrap(),
        AlmanacMap {
            source: (98, 99),
            destination: (50, 51),
            range: 2,
        }
    );
    // 52 50 48
    assert_eq!(
        AlmanacMap::from_str("52 50 48").unwrap(),
        AlmanacMap {
            source: (50, 97),
            destination: (52, 99),
            range: 48,
        }
    );
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    assert_eq!(
        AlmanacMap::from_str("88 18 7").unwrap(),
        AlmanacMap {
            source: (18, 24),
            destination: (88, 94),
            range: 7,
        }
    );
    assert_eq!(
        AlmanacMap::from_str("18 25 70").unwrap(),
        AlmanacMap {
            source: (25, 94),
            destination: (18, 87),
            range: 70,
        }
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    let almanac = part_one::Almanac::from_str(input).ok()?;
    almanac
        .seeds
        .iter()
        .map(get_min_of_locations(almanac.maps))
        .min()
}

fn get_min_of_locations(maps: Vec<Vec<AlmanacMap>>) -> impl Fn(&u32) -> u32 {
    move |&seed| maps.iter().fold(seed, |s, map| translate_seed(s, map))
}

fn translate_seed(from: u32, map: &[AlmanacMap]) -> u32 {
    map.iter()
        .find(|tr| tr.source.0 <= from && tr.source.1 >= from)
        .map(|tr| tr.destination.0 + (from - tr.source.0))
        .unwrap_or(from)
}

pub fn part_two(input: &str) -> Option<u32> {
    let almanac = part_two::Almanac::from_str(input).ok()?;
    almanac
        .seeds
        .iter()
        .flat_map(|(start, range)| {
            let array = *start..=start.wrapping_add(*range).wrapping_sub(1);
            array
                .map(|seed| {
                    almanac
                        .maps
                        .iter()
                        .fold(seed, |s, map| translate_seed(s, map))
                })
                .collect::<Vec<u32>>()
        })
        .min()
}

advent_of_code::main!(5);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_seed() {
        let map = [
            AlmanacMap {
                source: (98, 99),
                destination: (50, 51),
                range: 2,
            },
            AlmanacMap {
                source: (50, 97),
                destination: (52, 99),
                range: 48,
            },
        ];
        assert_eq!(translate_seed(33, &map), 33);
        assert_eq!(translate_seed(50, &map), 52);
        assert_eq!(translate_seed(53, &map), 55);
        assert_eq!(translate_seed(96, &map), 98);
        assert_eq!(translate_seed(97, &map), 99);
        assert_eq!(translate_seed(98, &map), 50);
        assert_eq!(translate_seed(99, &map), 51);

        // water-to-light map:
        // 88 18 7
        // 18 25 70
        let map = [
            AlmanacMap {
                source: (18, 24),
                destination: (88, 94),
                range: 7,
            },
            AlmanacMap {
                source: (25, 94),
                destination: (18, 87),
                range: 70,
            },
        ];
        let seed = 81;
        assert_eq!(translate_seed(seed, &map), 74);
    }

    #[test]
    fn test_part_one_building() {
        let almanac =
            part_one::Almanac::from_str(&advent_of_code::template::read_file("examples", 5))
                .expect("failed to parse almanac");

        let x = almanac
            .seeds
            .iter()
            .map(|&seed| {
                almanac.maps.iter().fold(vec![seed], |s, map| {
                    let translation = translate_seed(*s.last().unwrap(), map);
                    let mut result: Vec<u32> = s;
                    result.push(translation);
                    result
                })
            })
            .collect::<Vec<Vec<u32>>>();

        let expected = vec![
            vec![79, 81, 81, 81, 74, 78, 78, 82],
            vec![14, 14, 53, 49, 42, 42, 43, 43],
            vec![55, 57, 57, 53, 46, 82, 82, 86],
            vec![13, 13, 52, 41, 34, 34, 35, 35],
        ];
        assert_eq!(x, expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 5));
        assert_eq!(result, Some(46));
    }
}
