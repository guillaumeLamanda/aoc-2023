pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()?
        .strip_prefix("Time:")?
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();
    let distances = lines
        .next()?
        .strip_prefix("Distance:")?
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>();

    let records = times.iter().zip(distances).collect::<Vec<(_, _)>>();
    let possibilities = records
        .iter()
        .map(|(time, distance)| {
            let middle = time.div_ceil(2);
            let part1 = (1..middle)
                .rev()
                .take_while(|&t| (t * (**time - t)) > *distance)
                .collect::<Vec<u32>>();

            let part2 = (middle..**time)
                .take_while(|&t| (t * (**time - t)) > *distance)
                .collect::<Vec<u32>>();
            part1.iter().chain(part2.iter()).count() as u32
        })
        .collect::<Vec<u32>>();

    possibilities.iter().product::<u32>().into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()?
        .strip_prefix("Time:")?
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()?
        .strip_prefix("Distance:")?
        .replace(' ', "")
        .trim()
        .parse::<u64>()
        .unwrap();

    let middle = time.div_ceil(2);
    let part1 = (1..middle)
        .rev()
        .take_while(|&t| (t * (time - t)) > distance)
        .collect::<Vec<_>>();

    let part2 = (middle..time)
        .take_while(|&t| (t * (time - t)) > distance)
        .collect::<Vec<_>>();

    Some(part1.iter().chain(part2.iter()).count() as u32)
}

advent_of_code::main!(6);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 6));
        assert_eq!(result, Some(71503));
    }
}
