use std::cmp::Reverse;

struct PartOne<'a>(&'a str);
struct PartTwo<'a>(&'a str);

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPairs = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandType {
    fn from_cards_part_1(cards: Vec<u8>) -> HandType {
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
fn hand_type_from_cards_part_1() {
    assert_eq!(
        HandType::from_cards_part_1(vec![5, 12, 11, 13, 2]),
        HandType::HighCard
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![3, 2, 10, 3, 13]),
        HandType::OnePair
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![10, 5, 5, 11, 5]),
        HandType::ThreeOfAKind
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![10, 6, 5, 10, 5]),
        HandType::TwoPairs
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![10, 5, 5, 5, 5]),
        HandType::FourOfAKind
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![10, 5, 5, 10, 5]),
        HandType::FullHouse
    );
    assert_eq!(
        HandType::from_cards_part_1(vec![10, 10, 10, 10]),
        HandType::FiveOfAKind
    );
}

impl HandType {
    fn from_cards_part_2(cards: Vec<u8>) -> HandType {
        let number_of_jokers = cards.iter().filter(|&&card| card == 1).count();
        if number_of_jokers == 0 {
            return HandType::from_cards_part_1(cards);
        }

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
        let number_of_jokers = groups.iter().find(|(_, &card)| card == 1).unwrap().0;

        match (groups[0].0, number_of_jokers) {
            (5, _) => HandType::FiveOfAKind,
            (4, _) => HandType::FiveOfAKind,
            (3, _) if groups[1].0 == 2 => HandType::FiveOfAKind,
            (3, _) => HandType::FourOfAKind,
            (2, 2) if groups[1].0 == 2 => HandType::FourOfAKind,
            (2, 1) if groups[1].0 == 2 => HandType::FullHouse,
            (2, _) => HandType::ThreeOfAKind,
            _ => HandType::OnePair,
        }
    }
}

#[test]
fn test_from_cards_part_2() {
    assert_eq!(
        HandType::from_cards_part_2(vec![1, 2, 3, 4, 5]),
        HandType::OnePair
    );
    assert_eq!(
        HandType::from_cards_part_2(vec![10, 3, 10, 3, 1]),
        HandType::FullHouse
    );
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
    hand_type: HandType,
}

impl<'a> From<PartOne<'a>> for Hand {
    fn from(PartOne(s): PartOne<'a>) -> Self {
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
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(parse_card).collect();
        Self {
            hand_type: HandType::from_cards_part_1(cards.clone()),
            cards,
            bid: bid.parse().unwrap(),
        }
    }
}

#[test]
fn test_hand_from() {
    assert_eq!(
        Hand::from(PartOne("32T3K 765")),
        Hand {
            hand_type: HandType::OnePair,
            cards: vec![3, 2, 10, 3, 13],
            bid: 765,
        }
    )
}

impl<'a> From<PartTwo<'a>> for Hand {
    fn from(PartTwo(s): PartTwo<'a>) -> Self {
        fn parse_card(card: char) -> u8 {
            match card {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => card.to_digit(10).unwrap() as u8,
            }
        }
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards: Vec<_> = cards.chars().map(parse_card).collect();
        Self {
            hand_type: HandType::from_cards_part_2(cards.clone()),
            cards,
            bid: bid.parse().unwrap(),
        }
    }
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
    let simple = Hand::from(PartOne("32T3K 765"));
    let pair1 = Hand::from(PartOne("KK677 28"));
    assert!(pair1 > simple);
    let pair2 = Hand::from(PartOne("KTJJT 220"));
    assert!(pair1 > pair2);
    let triple1 = Hand::from(PartOne("T55J5 684"));
    assert!(triple1 > pair1);
    assert!(triple1 > pair2);
    let triple2 = Hand::from(PartOne("QQQJA 483"));
    assert!(triple2 > triple1);
    let full_house1 = Hand::from(PartOne("KKKQQ 483"));
    assert!(full_house1 > triple2);
    let full_house2 = Hand::from(PartOne("QQTTT 483"));
    assert!(full_house2 < full_house1);
    let four1 = Hand::from(PartOne("KKKKA 483"));
    assert!(four1 > full_house1);
    assert!(four1 > full_house2);
    let five1 = Hand::from(PartOne("KKKKK 483"));
    assert!(five1 > four1);
    let five2 = Hand::from(PartOne("AAAAA 483"));
    assert!(five2 > five1);

    assert!(Hand::from(PartOne("AAAAK 483")) > Hand::from(PartOne("KAAAA 483")));
    assert!(Hand::from(PartOne("AAAAK 483")) > Hand::from(PartOne("AKAAA 483")));
    assert!(Hand::from(PartOne("AAAAK 483")) > Hand::from(PartOne("AKAAA 483")));
}

#[test]
fn test_hand_ordering_on_examples() {
    let mut hands = [
        Hand::from(PartOne("32T3K 765")),
        Hand::from(PartOne("KK677 28")),
        Hand::from(PartOne("T55J5 684")),
        Hand::from(PartOne("KTJJT 220")),
        Hand::from(PartOne("QQQJA 483")),
    ];
    hands.sort();
    assert_eq!(
        hands,
        [
            Hand::from(PartOne("32T3K 765")),
            Hand::from(PartOne("KTJJT 220")),
            Hand::from(PartOne("KK677 28")),
            Hand::from(PartOne("T55J5 684")),
            Hand::from(PartOne("QQQJA 483")),
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
        .map(|line| Hand::from(PartOne(line)))
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
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| Hand::from(PartTwo(line)))
        .collect();

    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank as u32 + 1) * hand.bid)
        .sum::<u32>()
        .into()
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
        let result = part_two(&advent_of_code::template::read_file_part("examples", 7, 2));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_part_two_alternate() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", 7, 1));
        assert_eq!(result, Some(6839));
    }
}
