use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use super::Day;

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new(input: String) -> Day4 {
        Day4 { input }
    }

    fn get_card_map(&self) -> HashMap<usize, Card> {
        let lines = self.input.lines().collect::<Vec<&str>>();
        let mut cards = HashMap::new();
        for line in lines {
            let card = Card::from_str(line).unwrap();
            cards.insert(card.id, card);
        }
        cards
    }

    fn get_card_queue(&self) -> VecDeque<Card> {
        let lines = self.input.lines().collect::<Vec<&str>>();
        let mut cards = VecDeque::new();
        for line in lines {
            let card = Card::from_str(line).unwrap();
            cards.push_back(card);
        }
        cards
    }
}

impl Day for Day4 {
    fn part1(&self) -> String {
        let mut total_points = 0;
        let cards = self.get_card_map();
        for card in cards {
            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    println!("Card {} has {} winning numbers", card.1.id, card.1.get_winning_numbers().len());
                    println!("Card {} has {} points", card.1.id, card.1.get_point_value());
                }
            }
            total_points += card.1.get_point_value();
        }
        format!("{total_points}")
    }

    fn part2(&self) -> String {
        // create queue of all cards, with 1 at the front
        // pop off the top card, add one to the counter
        // if it has winning numbers, add the next x # of cards to the back, where x is the number of winning numbers
        // repeat until the entire queue has been processed
        // return the counter
        let card_map = self.get_card_map();
        let mut card_queue = self.get_card_queue();
        let mut counter = 0;
        while let Some(card) = card_queue.pop_front() {
            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    println!("Card {} has {} winning numbers", card.id, card.get_winning_numbers().len());
                }
            }
            counter += 1;
            let winning_numbers = card.get_winning_numbers();
            if !winning_numbers.is_empty() {
                for n in 1..winning_numbers.len() + 1 {
                    let next_card = card_map.get(&(card.id + n));
                    if let Some(next_card) = next_card {
                        cfg_if::cfg_if! {
                            if #[cfg(debug_assertions)] {
                                println!("Adding card {} to the queue", next_card.id);
                            }
                        }
                        card_queue.push_back(next_card.clone());
                    };
                }
            }
        }
        format!("{counter}")
    }
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    point_value: usize,
    winners: HashSet<usize>,
}

impl Card {
    // helper function to calculate the winning numbers on a card
    fn calculate_winning_numbers(
        winning_numbers: &HashSet<usize>,
        card_numbers: &HashSet<usize>,
    ) -> HashSet<usize> {
        winning_numbers
            .intersection(card_numbers)
            .map(|n| *n)
            .collect::<HashSet<usize>>()
    }

    // helper function to calculate the point value of a card
    fn calculate_point_value(winners: &HashSet<usize>) -> usize {
        if winners.is_empty() {
            return 0;
        }
        2usize.pow((winners.len() - 1) as u32)
    }

    fn get_winning_numbers(&self) -> &HashSet<usize> {
        &self.winners
    }

    /// point value is determined by 2^(x-1) where x is the number of winning numbers on the card
    fn get_point_value(&self) -> usize {
        self.point_value
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // string format:
        // Card <id>: <number> <number> <number> <number> <number> | <number> <number> <number> <number> <number>
        // the nuumbers before the | are the winning numbers, the rest are the numbers on the card
        let mut parts = s.split(":");
        let card_id = parts
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let numbers = parts
            .next()
            .unwrap()
            .trim()
            .split("|")
            .collect::<Vec<&str>>();
        let winning_numbers = numbers[0]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let card_numbers = numbers[1]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect::<HashSet<usize>>();
        let winners = Card::calculate_winning_numbers(&winning_numbers, &card_numbers);
        let point_value = Card::calculate_point_value(&winners);
        Ok(Card {
            id: card_id,
            winners,
            point_value,
        })
    }
}
