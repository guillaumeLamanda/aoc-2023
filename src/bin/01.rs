pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .filter(|char| char.is_ascii_digit())
                    .collect::<String>()
            })
            .map(extract_number)
            .sum(),
    )
}

fn extract_number(line: String) -> u32 {
    let first = line.chars().next().unwrap();
    let last = line.chars().last().unwrap();
    let number = format!("{}{}", first, last);
    number.parse::<u32>().unwrap()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(get_all_numbers_from_line)
            .map(extract_number)
            .sum(),
    )
}

fn get_all_numbers_from_line(line: &str) -> String {
    let possible_numbers = vec![
        ('1', "one"),
        ('2', "two"),
        ('3', "three"),
        ('4', "four"),
        ('5', "five"),
        ('6', "six"),
        ('7', "seven"),
        ('8', "eight"),
        ('9', "nine"),
        ('0', "zero"),
    ];
    line.chars()
        .enumerate()
        .filter_map(|(position, k)| {
            if k.is_ascii_digit() {
                return Some(k);
            }
            possible_numbers
                .iter()
                .find(|number_as_string| {
                    line[position..].starts_with(number_as_string.1)
                })
                .map(|(number, _)| *number)
        })
        .collect::<String>()
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_numbers_from_line() {
        assert_eq!(get_all_numbers_from_line("1ai5ie2"), "152");
        assert_eq!(get_all_numbers_from_line("1two5ie2"), "1252");
        assert_eq!(get_all_numbers_from_line("1oneight5ie2"), "11852");
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 1, 2));
        assert_eq!(result, Some(281));
    }
}
