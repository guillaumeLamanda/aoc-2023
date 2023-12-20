use std::{collections::HashMap, str::FromStr, usize};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Category {
    Extremely = 0,
    Musical = 1,
    Aerodynamic = 2,
    Shiny = 3,
}

impl From<char> for Category {
    fn from(c: char) -> Self {
        match c {
            'x' => Self::Extremely,
            'm' => Self::Musical,
            'a' => Self::Aerodynamic,
            's' => Self::Shiny,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Operand {
    GreaterThan,
    LowerThan,
}

#[derive(Debug)]
struct Rule {
    testing: Category,
    comparing_to: usize,
    operand: Operand,
    destination: String,
}

impl Rule {
    fn is_finisher(&self) -> bool {
        self.destination == "A"
    }

    fn verify(&self, datum: &HashMap<Category, usize>) -> Option<String> {
        match self.operand {
            Operand::GreaterThan => {
                if datum.get(&self.testing).unwrap() > &self.comparing_to {
                    Some(self.destination.clone())
                } else {
                    None
                }
            }
            Operand::LowerThan => {
                if datum.get(&self.testing).unwrap() < &self.comparing_to {
                    Some(self.destination.clone())
                } else {
                    None
                }
            }
        }
    }
}

#[test]
fn test_rule_verify() {
    let rule = Rule::from_str("a<2006:qkq").unwrap();
    let mut map = HashMap::<Category, usize>::new();
    map.insert(Category::Aerodynamic, 0);
    assert_eq!(rule.verify(&map), Some("qkq".to_string()));
    let rule = Rule::from_str("m>2090:A").unwrap();
    map.insert(Category::Musical, 3000);
    assert_eq!(rule.verify(&map), Some("A".to_string()));
    map.insert(Category::Musical, 2000);
    assert_eq!(rule.verify(&map), None);
}

impl FromStr for Rule {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, destination) = s.split_once(':').unwrap();
        let category = s.chars().nth(0).unwrap();
        let operand = match s.as_bytes()[1] {
            b'>' => Operand::GreaterThan,
            b'<' => Operand::LowerThan,
            _ => unreachable!(),
        };
        let comparing_to = instruction.get(2..).unwrap().parse().unwrap();
        Ok(Self {
            testing: Category::from(category),
            destination: destination.to_string(),
            operand,
            comparing_to,
        })
    }
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
    default: String,
}

impl Rules {
    fn is_finisher(&self) -> bool {
        self.default == "A" || self.rules.iter().any(|rule| rule.is_finisher())
    }

    fn apply(&self, datum: &HashMap<Category, usize>) -> String {
        self.rules
            .iter()
            .find_map(|rule| rule.verify(datum))
            .unwrap_or(self.default.clone())
    }

    fn is_finishing_at(&self, destination: &str) -> bool {
        self.default == destination || self.get_rule_finishing_at(destination).is_some()
    }

    fn get_rule_finishing_at(&self, destination: &str) -> Option<&Rule> {
        self.rules
            .iter()
            .find(|rule| rule.destination == destination)
    }
}

#[test]
fn test_rules_apply() {
    let rules = Rules::from_str("a<2006:qkq,m>2090:A,rfg").unwrap();
    let mut categories = HashMap::<Category, usize>::new();
    categories.insert(Category::Aerodynamic, 0);
    assert_eq!(rules.apply(&categories), "qkq");
    categories.insert(Category::Aerodynamic, 3000);
    categories.insert(Category::Musical, 3000);
    assert_eq!(rules.apply(&categories), "A");
    categories.insert(Category::Musical, 2000);
    assert_eq!(categories.get(&Category::Musical), Some(&2000));
    assert_eq!(rules.apply(&categories), "rfg");
}

impl FromStr for Rules {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = s.split(',').collect::<Vec<_>>();
        let default = rules.pop().unwrap().to_string();
        let rules = rules.iter().map(|r| Rule::from_str(r).unwrap()).collect();
        Ok(Self { rules, default })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, dataset) = input.split_once("\n\n").unwrap();

    let rules: HashMap<String, Rules> = rules
        .lines()
        .map(|rule_line| {
            let (id, rules) = rule_line.split_once('{').unwrap();
            let rules = rules.replace('}', "").to_string();
            (id.to_string(), Rules::from_str(&rules).unwrap())
        })
        .collect();

    let dataset = dataset
        .lines()
        .map(|data_line| {
            let data_line = data_line.replace(['{', '}'], "");
            data_line
                .split(',')
                .filter_map(|catogory_to_number| catogory_to_number.split_once('='))
                .map(|(category, number)| {
                    (
                        Category::from(category.chars().next().unwrap()),
                        number.parse::<usize>().unwrap(),
                    )
                })
                .collect::<HashMap<Category, usize>>()
        })
        .collect::<Vec<_>>();

    Some(
        dataset
            .iter()
            .filter_map(|datum| follow(datum, &rules))
            .sum(),
    )
}

fn follow(datum: &HashMap<Category, usize>, rules: &HashMap<String, Rules>) -> Option<usize> {
    let mut current = "in".to_string();
    while current != "A" {
        let rules = rules.get(&current).unwrap();
        let destination = rules.apply(datum);
        if destination == "R" {
            return None;
        }
        current = destination;
    }

    Some(datum.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rules, _) = input.split_once("\n\n").unwrap();

    let id_to_rules: HashMap<String, Rules> = rules
        .lines()
        .map(|rule_line| {
            let (id, rules) = rule_line.split_once('{').unwrap();
            let rules = rules.replace('}', "").to_string();
            (id.to_string(), Rules::from_str(&rules).unwrap())
        })
        .collect();

    let finishing_rules = id_to_rules
        .iter()
        .filter(|(_, rules)| rules.is_finisher())
        .collect::<Vec<_>>();
    println!("finishing_rules: {:?}", finishing_rules);

    for (id, rules) in finishing_rules {
        let mut possibilities_ranges = [[1, 4000]; 4];
        if rules.default == "A" {
            // reverse all conditions
            // example: pv{a>1716:R,A}
            for rule in &rules.rules {
                match rule.operand {
                    Operand::GreaterThan => {
                        possibilities_ranges[rule.testing as usize][0] = rule.comparing_to + 1;
                    }
                    Operand::LowerThan => {
                        possibilities_ranges[rule.testing as usize][1] = rule.comparing_to - 1;
                    }
                }
            }
        } else {
            for rule in &rules.rules {
                match rule.operand {
                    Operand::GreaterThan => {
                        possibilities_ranges[rule.testing as usize][1] = rule.comparing_to;
                    }
                    Operand::LowerThan => {
                        possibilities_ranges[rule.testing as usize][0] = rule.comparing_to;
                    }
                }
            }
        }

        // need to get the next one
        let mut next_id = id;
        while let Some((id, rules)) = id_to_rules
            .iter()
            .find(|(_, rules)| rules.is_finishing_at(next_id))
        {
            if let Some(rule) = rules.get_rule_finishing_at(next_id) {
                match rule.operand {
                    Operand::GreaterThan => {
                        possibilities_ranges[rule.testing as usize][0] = rule.comparing_to + 1;
                    }
                    Operand::LowerThan => {
                        possibilities_ranges[rule.testing as usize][1] = rule.comparing_to - 1;
                    }
                }
            } else {
                // revert, one more time
                for rule in &rules.rules {
                    match rule.operand {
                        Operand::GreaterThan => {
                            possibilities_ranges[rule.testing as usize][1] = rule.comparing_to;
                        }
                        Operand::LowerThan => {
                            possibilities_ranges[rule.testing as usize][0] = rule.comparing_to;
                        }
                    }
                }
            }
            next_id = id;
        }
        println!("possibilities_ranges: {:?}", possibilities_ranges);
    }

    None
}

advent_of_code::main!(19);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 19));
        assert_eq!(result, Some(167409079868000));
    }
}
