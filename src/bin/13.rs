use advent_of_code::map::Map;

pub fn part_one(input: &str) -> Option<u32> {
    let x = input
        .split("\n\n")
        .filter_map(|map| {
            let map = Map::from(map);
            let rows_count = walk_through_map(&map.map);
            if rows_count.is_some() {
                return rows_count.map(|c| c * 100);
            }
            let cols = Map::transpose(map.map);
            walk_through_map(&cols)
        })
        .sum::<usize>();

    Some(x as u32)
}

fn walk_through_map(map: &[Vec<char>]) -> Option<usize> {
    let mut count = vec![];
    let mut i = 1;
    while i < map.len() {
        // we need to check if the n next columns are mirroring
        let mut x = map.to_vec().clone();
        let (part_1, part_2) = x.split_at_mut(i);
        part_1.reverse();

        let min = std::cmp::min(part_1.len(), part_2.len());
        if part_1[0..min] == part_2[0..min] {
            count.push(part_1.len());
        }
        i += 1;
    }
    count.iter().max().copied()
}

#[test]
fn test_walk_througth_second_schema() {
    let example = &advent_of_code::template::read_file("examples", 13);
    let (schema_1, schema_2) = example.split_once("\n\n").unwrap();

    let map = Map::from(schema_1);
    let r = walk_through_map(&map.map);
    assert_eq!(r, None);

    let cols = Map::transpose(map.map);
    let r = walk_through_map(&cols);
    assert_eq!(r, Some(5));

    let map = Map::from(schema_2);
    let r = walk_through_map(&map.map);
    assert_eq!(r, Some(4));

    let cols = Map::transpose(map.map);
    let r = walk_through_map(&cols);
    assert_eq!(r, None);
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(13);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 13));
        assert_eq!(result, None);
    }
}
