use std::{cmp::Reverse, str::FromStr};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: Vec<u8>) -> HandType {
        let mut cards = cards.clone();
        cards.sort();
        let map = cards
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, card| {
                *acc.entry(card).or_insert(0) += 1;
                acc
            });
        let mut groups: Vec<_> = map.into_iter().map(|(card, count)| (count, card)).collect();
        groups.sort_unstable_by_key(|&x| Reverse(x));
        match groups.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 if groups[0].0 == 3 => HandType::ThreeOfAKind,
            3 => HandType::TwoPairs,
            2 if groups[0].0 == 4 => HandType::FourOfAKind,
            2 if groups[0].0 == 3 => HandType::FullHouse,
            1 => HandType::FiveOfAKind,
            _ => panic!("Invalid hand"),
        }
    }
}

#[test]
fn hand_type_from_cards() {
    assert_eq!(
        HandType::from_cards(vec![5, 12, 11, 13, 2]),
        HandType::HighCard
    );
    assert_eq!(
        HandType::from_cards(vec![3, 2, 10, 3, 13]),
        HandType::OnePair
    );
    assert_eq!(
        HandType::from_cards(vec![10, 5, 5, 11, 5]),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        HandType::from_cards(vec![10, 6, 5, 10, 5]),
        HandType::TwoPairs
    );
    assert_eq!(
        HandType::from_cards(vec![10, 5, 5, 5, 5]),
        HandType::FourOfAKind
    );
    assert_eq!(
        HandType::from_cards(vec![10, 5, 5, 10, 5]),
        HandType::FullHouse
    );
    assert_eq!(
        HandType::from_cards(vec![10, 10, 10, 10]),
        HandType::FiveOfAKind
    )
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
    hand_type: HandType,
}

fn parse_card(card: char) -> u8 {
    match card {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => card.to_digit(10).unwrap() as u8,
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(parse_card).collect();
        Ok(Self {
            hand_type: HandType::from_cards(cards.clone()),
            cards,
            bid: bid.parse().unwrap(),
        })
    }
}

#[test]
fn test_hand_from_str() {
    assert_eq!(
        Hand::from_str("32T3K 765"),
        Ok(Hand {
            hand_type: HandType::OnePair,
            cards: vec![3, 2, 10, 3, 13],
            bid: 765,
        })
    )
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

#[test]
fn test_hand_ordering() {
    let simple = Hand::from_str("32T3K 765").unwrap();
    let pair1 = Hand::from_str("KK677 28").unwrap();
    assert!(pair1 > simple);
    let pair2 = Hand::from_str("KTJJT 220").unwrap();
    assert!(pair1 > pair2);
    let triple1 = Hand::from_str("T55J5 684").unwrap();
    assert!(triple1 > pair1);
    assert!(triple1 > pair2);
    let triple2 = Hand::from_str("QQQJA 483").unwrap();
    assert!(triple2 > triple1);
    let full_house1 = Hand::from_str("KKKQQ 483").unwrap();
    assert!(full_house1 > triple2);
    let full_house2 = Hand::from_str("QQTTT 483").unwrap();
    assert!(full_house2 < full_house1);
    let four1 = Hand::from_str("KKKKA 483").unwrap();
    assert!(four1 > full_house1);
    assert!(four1 > full_house2);
    let five1 = Hand::from_str("KKKKK 483").unwrap();
    assert!(five1 > four1);
    let five2 = Hand::from_str("AAAAA 483").unwrap();
    assert!(five2 > five1);

    assert!(Hand::from_str("AAAAK 483").unwrap() > Hand::from_str("KAAAA 483").unwrap());
    assert!(Hand::from_str("AAAAK 483").unwrap() > Hand::from_str("AKAAA 483").unwrap());
    assert!(Hand::from_str("AAAAK 483").unwrap() > Hand::from_str("AKAAA 483").unwrap());
}

#[test]
fn test_hand_ordering_on_examples() {
    let mut hands = [
        Hand::from_str("32T3K 765").unwrap(),
        Hand::from_str("KK677 28").unwrap(),
        Hand::from_str("T55J5 684").unwrap(),
        Hand::from_str("KTJJT 220").unwrap(),
        Hand::from_str("QQQJA 483").unwrap(),
    ];
    hands.sort();
    assert_eq!(
        hands,
        [
            Hand::from_str("32T3K 765").unwrap(),
            Hand::from_str("KTJJT 220").unwrap(),
            Hand::from_str("KK677 28").unwrap(),
            Hand::from_str("T55J5 684").unwrap(),
            Hand::from_str("QQQJA 483").unwrap(),
        ]
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    /* sorted
    32T3K 765
    KTJJT 220
    KK677 28
    T55J5 684
    QQQJA 483
        */
    let mut hands: Vec<_> = input
        .lines()
        .flat_map(|line| Hand::from_str(line).ok())
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

advent_of_code::main!(7);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_one_alternate() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", 7, 1));
        assert_eq!(result, Some(6592));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 7));
        assert_eq!(result, Some(5905));
    }
}
