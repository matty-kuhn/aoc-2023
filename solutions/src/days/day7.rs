use std::{collections::HashMap, str::FromStr};

use super::{get_lines, Day};

pub struct Day7 {
    input: String,
}

impl Day7 {
    pub fn new(input: String) -> Day7 {
        Day7 { input }
    }

    fn get_parsed_input(&self) -> Vec<Hand> {
        get_lines(&self.input)
            .iter()
            .map(|line| line.parse::<Hand>().unwrap())
            .collect::<Vec<Hand>>()
    }

    fn get_parsed_input2(&self) -> Vec<Hand2> {
        get_lines(&self.input)
            .iter()
            .map(|line| line.parse::<Hand2>().unwrap())
            .collect::<Vec<Hand2>>()
    }
}

impl Day for Day7 {
    fn part1(&self) -> String {
        let mut hands = self.get_parsed_input();
        hands.sort();
        let sum = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.get_bid() * (idx + 1)));
        format!("{sum}")
    }

    fn part2(&self) -> String {
        let mut hands = self.get_parsed_input2();
        hands.sort();
        let sum = hands
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, hand)| acc + (hand.get_bid() * (idx + 1)));
        format!("{sum}")
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Hand {
    Five { hand: Vec<Card>, bid: usize },
    Four { hand: Vec<Card>, bid: usize },
    Full { hand: Vec<Card>, bid: usize },
    Three { hand: Vec<Card>, bid: usize },
    Two { hand: Vec<Card>, bid: usize },
    One { hand: Vec<Card>, bid: usize },
    High { hand: Vec<Card>, bid: usize },
}

impl Hand {
    fn get_bid(&self) -> usize {
        match self {
            Hand::Five { bid, .. } => *bid,
            Hand::Four { bid, .. } => *bid,
            Hand::Full { bid, .. } => *bid,
            Hand::Three { bid, .. } => *bid,
            Hand::Two { bid, .. } => *bid,
            Hand::One { bid, .. } => *bid,
            Hand::High { bid, .. } => *bid,
        }
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // need to determine the type of hand by number of same cards in the list
        // five of a kind: all 4 the same
        // four of a kind: 4 the same
        // full house: 3 the same, 2 the same
        // three of a kind: 3 the same
        // two pair: 2 the same, 2 the same
        // one pair: 2 the same
        // high card: none the same
        let mut split = s.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Card>().unwrap())
            .collect::<Vec<Card>>();
        let bid = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid bid value");
        let mut counts = HashMap::new();
        for card in &cards {
            let count = counts.entry(card).or_insert(0);
            *count += 1;
        }
        match counts.keys().len() {
            1 => Ok(Hand::Five { hand: cards, bid }),
            2 => {
                let mut counts = counts.values().collect::<Vec<&usize>>();
                counts.sort();
                if *counts[0] == 1 {
                    Ok(Hand::Four { hand: cards, bid })
                } else {
                    Ok(Hand::Full { hand: cards, bid })
                }
            }
            3 => {
                let mut counts = counts.values().collect::<Vec<&usize>>();
                counts.sort();
                if *counts[0] == 1 {
                    if *counts[1] == 1 {
                        Ok(Hand::Three { hand: cards, bid })
                    } else {
                        Ok(Hand::Two { hand: cards, bid })
                    }
                } else {
                    unreachable!("Invalid hand")
                }
            }
            4 => Ok(Hand::One { hand: cards, bid }),
            5 => Ok(Hand::High { hand: cards, bid }),
            _ => panic!("Invalid number of cards"),
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
        match (self, other) {
            (Hand::Five { hand: h1, .. }, Hand::Five { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand::Five { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::Five { .. }) => std::cmp::Ordering::Less,
            (Hand::Four { hand: h1, .. }, Hand::Four { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand::Four { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::Four { .. }) => std::cmp::Ordering::Less,
            (Hand::Full { hand: h1, .. }, Hand::Full { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand::Full { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::Full { .. }) => std::cmp::Ordering::Less,
            (Hand::Three { hand: h1, .. }, Hand::Three { hand: h2, .. }) => h1.cmp(h2),
            (Hand::Three { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::Three { .. }) => std::cmp::Ordering::Less,
            (Hand::Two { hand: h1, .. }, Hand::Two { hand: h2, .. }) => h1.cmp(h2),
            (Hand::Two { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::Two { .. }) => std::cmp::Ordering::Less,
            (Hand::One { hand: h1, .. }, Hand::One { hand: h2, .. }) => h1.cmp(h2),
            (Hand::One { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand::One { .. }) => std::cmp::Ordering::Less,
            (Hand::High { hand: h1, .. }, Hand::High { hand: h2, .. }) => h1.cmp(h2),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => panic!("Invalid card"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Card::Two, Card::Two) => std::cmp::Ordering::Equal,
            (Card::Two, _) => std::cmp::Ordering::Less,
            (Card::Three, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Three, Card::Three) => std::cmp::Ordering::Equal,
            (Card::Three, _) => std::cmp::Ordering::Less,
            (Card::Four, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Four, Card::Three) => std::cmp::Ordering::Greater,
            (Card::Four, Card::Four) => std::cmp::Ordering::Equal,
            (Card::Four, _) => std::cmp::Ordering::Less,
            (Card::Five, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Five, Card::Three) => std::cmp::Ordering::Greater,
            (Card::Five, Card::Four) => std::cmp::Ordering::Greater,
            (Card::Five, Card::Five) => std::cmp::Ordering::Equal,
            (Card::Five, _) => std::cmp::Ordering::Less,
            (Card::Six, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Six, Card::Three) => std::cmp::Ordering::Greater,
            (Card::Six, Card::Four) => std::cmp::Ordering::Greater,
            (Card::Six, Card::Five) => std::cmp::Ordering::Greater,
            (Card::Six, Card::Six) => std::cmp::Ordering::Equal,
            (Card::Six, _) => std::cmp::Ordering::Less,
            (Card::Seven, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Seven, Card::Three) => std::cmp::Ordering::Greater,
            (Card::Seven, Card::Four) => std::cmp::Ordering::Greater,
            (Card::Seven, Card::Five) => std::cmp::Ordering::Greater,
            (Card::Seven, Card::Six) => std::cmp::Ordering::Greater,
            (Card::Seven, Card::Seven) => std::cmp::Ordering::Equal,
            (Card::Seven, _) => std::cmp::Ordering::Less,
            (Card::Eight, Card::Two) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Three) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Four) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Five) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Six) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Seven) => std::cmp::Ordering::Greater,
            (Card::Eight, Card::Eight) => std::cmp::Ordering::Equal,
            (Card::Eight, _) => std::cmp::Ordering::Less,
            (Card::Nine, Card::Ace) => std::cmp::Ordering::Less,
            (Card::Nine, Card::King) => std::cmp::Ordering::Less,
            (Card::Nine, Card::Queen) => std::cmp::Ordering::Less,
            (Card::Nine, Card::Jack) => std::cmp::Ordering::Less,
            (Card::Nine, Card::Ten) => std::cmp::Ordering::Less,
            (Card::Nine, Card::Nine) => std::cmp::Ordering::Equal,
            (Card::Nine, _) => std::cmp::Ordering::Greater,
            (Card::Ten, Card::Ace) => std::cmp::Ordering::Less,
            (Card::Ten, Card::King) => std::cmp::Ordering::Less,
            (Card::Ten, Card::Queen) => std::cmp::Ordering::Less,
            (Card::Ten, Card::Jack) => std::cmp::Ordering::Less,
            (Card::Ten, Card::Ten) => std::cmp::Ordering::Equal,
            (Card::Ten, _) => std::cmp::Ordering::Greater,
            (Card::Jack, Card::Ace) => std::cmp::Ordering::Less,
            (Card::Jack, Card::King) => std::cmp::Ordering::Less,
            (Card::Jack, Card::Queen) => std::cmp::Ordering::Less,
            (Card::Jack, Card::Jack) => std::cmp::Ordering::Equal,
            (Card::Jack, _) => std::cmp::Ordering::Greater,
            (Card::Queen, Card::Ace) => std::cmp::Ordering::Less,
            (Card::Queen, Card::King) => std::cmp::Ordering::Less,
            (Card::Queen, Card::Queen) => std::cmp::Ordering::Equal,
            (Card::Queen, _) => std::cmp::Ordering::Greater,
            (Card::King, Card::Ace) => std::cmp::Ordering::Less,
            (Card::King, Card::King) => std::cmp::Ordering::Equal,
            (Card::King, _) => std::cmp::Ordering::Greater,
            (Card::Ace, Card::Ace) => std::cmp::Ordering::Equal,
            (Card::Ace, _) => std::cmp::Ordering::Greater,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Hand2 {
    Five { hand: Vec<Card2>, bid: usize },
    Four { hand: Vec<Card2>, bid: usize },
    Full { hand: Vec<Card2>, bid: usize },
    Three { hand: Vec<Card2>, bid: usize },
    Two { hand: Vec<Card2>, bid: usize },
    One { hand: Vec<Card2>, bid: usize },
    High { hand: Vec<Card2>, bid: usize },
}

impl Hand2 {
    fn get_bid(&self) -> usize {
        match self {
            Hand2::Five { bid, .. } => *bid,
            Hand2::Four { bid, .. } => *bid,
            Hand2::Full { bid, .. } => *bid,
            Hand2::Three { bid, .. } => *bid,
            Hand2::Two { bid, .. } => *bid,
            Hand2::One { bid, .. } => *bid,
            Hand2::High { bid, .. } => *bid,
        }
    }
}

impl FromStr for Hand2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // need to determine the type of hand by number of same cards in the list
        // five of a kind: all 4 the same
        // four of a kind: 4 the same
        // full house: 3 the same, 2 the same
        // three of a kind: 3 the same
        // two pair: 2 the same, 2 the same
        // one pair: 2 the same
        // high card: none the same
        let mut split = s.split_whitespace();
        let cards = split
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse::<Card2>().unwrap())
            .collect::<Vec<Card2>>();
        let bid = split
            .next()
            .unwrap()
            .parse::<usize>()
            .expect("Invalid bid value");
        let mut counts_map = HashMap::new();
        for card in &cards {
            let count = counts_map.entry(card).or_insert(0);
            *count += 1;
        }
        match counts_map.keys().len() {
            1 => Ok(Hand2::Five { hand: cards, bid }),
            2 => {
                let mut counts = counts_map.values().collect::<Vec<&usize>>();
                counts.sort();
                if *counts[0] == 1 {
                    // if counts contains J at all it is 5 of a kind
                    if counts_map.contains_key(&Card2::Joker) {
                        Ok(Hand2::Five { hand: cards, bid })
                    } else {
                        Ok(Hand2::Four { hand: cards, bid })
                    }
                } else {
                    // there are 2 of one card, 3 of another, so if any of them are Jokers then its a 5 of a kind
                    if counts_map.contains_key(&Card2::Joker) {
                        Ok(Hand2::Five { hand: cards, bid })
                    } else {
                        Ok(Hand2::Full { hand: cards, bid })
                    }
                }
            }
            3 => {
                let mut counts = counts_map.values().collect::<Vec<&usize>>();
                counts.sort();
                if *counts[0] == 1 {
                    // there could be 3, 1, 1 or 2, 2, 1
                    if *counts[1] == 1 {
                        // there are 3, 1, 1
                        // if there is a joker anywhere it becomes 4 of a kind
                        if counts_map.contains_key(&Card2::Joker) {
                            Ok(Hand2::Four { hand: cards, bid })
                        } else {
                            Ok(Hand2::Three { hand: cards, bid })
                        }
                    } else {
                        // there are 2, 2, 1
                        // if there is one joker, it is full house
                        // if there are 2 jokers, it is 4 of a kind
                        // no jokers is 2 pair
                        if counts_map.contains_key(&Card2::Joker)
                            && counts_map.get(&Card2::Joker).unwrap() == &1
                        {
                            Ok(Hand2::Full { hand: cards, bid })
                        } else if counts_map.contains_key(&Card2::Joker)
                            && counts_map.get(&Card2::Joker).unwrap() == &2
                        {
                            Ok(Hand2::Four { hand: cards, bid })
                        } else {
                            Ok(Hand2::Two { hand: cards, bid })
                        }
                    }
                } else {
                    // there will always be 1 of something
                    unreachable!("Invalid hand")
                }
            }
            4 => {
                // there is 2, 1, 1, 1
                // if there is a joker, it becomes 3 of a kind
                if counts_map.contains_key(&Card2::Joker) {
                    Ok(Hand2::Three { hand: cards, bid })
                } else {
                    Ok(Hand2::One { hand: cards, bid })
                }
            }
            5 => {
                // there are no pairs, so if there is a joker it becomes 1 pair
                if counts_map.contains_key(&Card2::Joker) {
                    Ok(Hand2::One { hand: cards, bid })
                } else {
                    Ok(Hand2::High { hand: cards, bid })
                }
            }
            _ => panic!("Invalid number of cards"),
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Hand2::Five { hand: h1, .. }, Hand2::Five { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand2::Five { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::Five { .. }) => std::cmp::Ordering::Less,
            (Hand2::Four { hand: h1, .. }, Hand2::Four { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand2::Four { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::Four { .. }) => std::cmp::Ordering::Less,
            (Hand2::Full { hand: h1, .. }, Hand2::Full { hand: h2, .. }) => {
                for (card1, card2) in h1.iter().zip(h2.iter()) {
                    if card1 != card2 {
                        return card1.cmp(card2);
                    }
                }
                std::cmp::Ordering::Equal
            }
            (Hand2::Full { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::Full { .. }) => std::cmp::Ordering::Less,
            (Hand2::Three { hand: h1, .. }, Hand2::Three { hand: h2, .. }) => h1.cmp(h2),
            (Hand2::Three { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::Three { .. }) => std::cmp::Ordering::Less,
            (Hand2::Two { hand: h1, .. }, Hand2::Two { hand: h2, .. }) => h1.cmp(h2),
            (Hand2::Two { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::Two { .. }) => std::cmp::Ordering::Less,
            (Hand2::One { hand: h1, .. }, Hand2::One { hand: h2, .. }) => h1.cmp(h2),
            (Hand2::One { .. }, _) => std::cmp::Ordering::Greater,
            (_, Hand2::One { .. }) => std::cmp::Ordering::Less,
            (Hand2::High { hand: h1, .. }, Hand2::High { hand: h2, .. }) => h1.cmp(h2),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for Card2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::Ten),
            "J" => Ok(Self::Joker),
            "Q" => Ok(Self::Queen),
            "K" => Ok(Self::King),
            "A" => Ok(Self::Ace),
            _ => panic!("Invalid card"),
        }
    }
}

impl PartialOrd for Card2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Card2::Joker, Card2::Joker) => std::cmp::Ordering::Equal,
            (Card2::Joker, _) => std::cmp::Ordering::Less,
            (_, Card2::Joker) => std::cmp::Ordering::Greater,
            (Card2::Two, Card2::Two) => std::cmp::Ordering::Equal,
            (Card2::Two, _) => std::cmp::Ordering::Less,
            (Card2::Three, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Three, Card2::Three) => std::cmp::Ordering::Equal,
            (Card2::Three, _) => std::cmp::Ordering::Less,
            (Card2::Four, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Four, Card2::Three) => std::cmp::Ordering::Greater,
            (Card2::Four, Card2::Four) => std::cmp::Ordering::Equal,
            (Card2::Four, _) => std::cmp::Ordering::Less,
            (Card2::Five, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Five, Card2::Three) => std::cmp::Ordering::Greater,
            (Card2::Five, Card2::Four) => std::cmp::Ordering::Greater,
            (Card2::Five, Card2::Five) => std::cmp::Ordering::Equal,
            (Card2::Five, _) => std::cmp::Ordering::Less,
            (Card2::Six, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Six, Card2::Three) => std::cmp::Ordering::Greater,
            (Card2::Six, Card2::Four) => std::cmp::Ordering::Greater,
            (Card2::Six, Card2::Five) => std::cmp::Ordering::Greater,
            (Card2::Six, Card2::Six) => std::cmp::Ordering::Equal,
            (Card2::Six, _) => std::cmp::Ordering::Less,
            (Card2::Seven, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Seven, Card2::Three) => std::cmp::Ordering::Greater,
            (Card2::Seven, Card2::Four) => std::cmp::Ordering::Greater,
            (Card2::Seven, Card2::Five) => std::cmp::Ordering::Greater,
            (Card2::Seven, Card2::Six) => std::cmp::Ordering::Greater,
            (Card2::Seven, Card2::Seven) => std::cmp::Ordering::Equal,
            (Card2::Seven, _) => std::cmp::Ordering::Less,
            (Card2::Eight, Card2::Two) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Three) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Four) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Five) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Six) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Seven) => std::cmp::Ordering::Greater,
            (Card2::Eight, Card2::Eight) => std::cmp::Ordering::Equal,
            (Card2::Eight, _) => std::cmp::Ordering::Less,
            (Card2::Nine, Card2::Ace) => std::cmp::Ordering::Less,
            (Card2::Nine, Card2::King) => std::cmp::Ordering::Less,
            (Card2::Nine, Card2::Queen) => std::cmp::Ordering::Less,
            (Card2::Nine, Card2::Ten) => std::cmp::Ordering::Less,
            (Card2::Nine, Card2::Nine) => std::cmp::Ordering::Equal,
            (Card2::Nine, _) => std::cmp::Ordering::Greater,
            (Card2::Ten, Card2::Ace) => std::cmp::Ordering::Less,
            (Card2::Ten, Card2::King) => std::cmp::Ordering::Less,
            (Card2::Ten, Card2::Queen) => std::cmp::Ordering::Less,
            (Card2::Ten, Card2::Ten) => std::cmp::Ordering::Equal,
            (Card2::Ten, _) => std::cmp::Ordering::Greater,
            (Card2::Queen, Card2::Ace) => std::cmp::Ordering::Less,
            (Card2::Queen, Card2::King) => std::cmp::Ordering::Less,
            (Card2::Queen, Card2::Queen) => std::cmp::Ordering::Equal,
            (Card2::Queen, _) => std::cmp::Ordering::Greater,
            (Card2::King, Card2::Ace) => std::cmp::Ordering::Less,
            (Card2::King, Card2::King) => std::cmp::Ordering::Equal,
            (Card2::King, _) => std::cmp::Ordering::Greater,
            (Card2::Ace, Card2::Ace) => std::cmp::Ordering::Equal,
            (Card2::Ace, _) => std::cmp::Ordering::Greater,
        }
    }
}
