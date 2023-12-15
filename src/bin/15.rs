use std::{collections::HashMap, ops::Rem};

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .split(',')
            .map(|c| c.trim())
            .filter(|c| !c.is_empty())
            .map(hash)
            .sum(),
    )
}

fn hash(c: &str) -> usize {
    c.chars().fold(0, |accu, c| {
        let r = accu + c as usize;
        let r = r * 17;
        r.rem(256)
    })
}

#[test]
fn test_hash() {
    assert_eq!(hash("rn=1"), 30);
    assert_eq!(hash("ot=7"), 231);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut hashmap = HashMap::<usize, Vec<(String, usize)>>::new();
    let operations = input
        .split(',')
        .map(|c| c.trim())
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>();
    for operation in operations {
        match operation.contains('=') {
            true => {
                let (label, focal_length) = operation.split_once('=').unwrap();
                let lens_box = hash(label);
                let vec = hashmap.entry(lens_box).or_default();
                // TODO: change value if already exists
                if let Some(position) = vec.iter().position(|b| b.0 == label) {
                    let b = vec.get_mut(position).unwrap();
                    b.1 = focal_length.parse().unwrap();
                } else {
                    vec.push((label.to_string(), focal_length.parse().unwrap()));
                }
            }
            false => {
                let label = operation.replace('-', "");
                let lens_box = hash(&label);
                if let Some(lens_box) = hashmap.get_mut(&lens_box) {
                    lens_box.retain(|(l, _)| l != &label);
                }
            }
        }
    }
    let r = hashmap
        .iter()
        .flat_map(|(i, v)| {
            v.iter()
                .enumerate()
                .map(move |(y, v)| (i + 1) * (y + 1) * v.1)
        })
        .sum();
    Some(r)
}

advent_of_code::main!(15);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 15));
        assert_eq!(result, Some(145));
    }
}
