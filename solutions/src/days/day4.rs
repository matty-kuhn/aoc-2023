use std::{collections::{HashSet, HashMap}, str::FromStr};

use super::Day;

pub struct Day4 {
    input: String,
}

impl Day4 {
    pub fn new(input: String) -> Day4 {
        Day4 { input }
    }
}

impl Day for Day4 {
    fn part1(&self) -> String {
        let lines = self.input.lines().collect::<Vec<&str>>();
        let mut cards = HashMap::new();
        for line in lines {
            let card = Card::from_str(line).unwrap();
            cards.insert(card.id, card);
        }
        let mut total_points = 0;
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
        format!("todo")
    }
}

struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    card_numbers: HashSet<usize>,
}

impl Card {
    fn get_winning_numbers(&self) -> HashSet<usize> {
        self.winning_numbers
            .intersection(&self.card_numbers)
            .map(|n| *n)
            .collect::<HashSet<usize>>()
    }

    /// point value is determined by 2^(x-1) where x is the number of winning numbers on the card
    fn get_point_value(&self) -> usize {
        let winning_numbers = self.get_winning_numbers();
        if winning_numbers.is_empty() {
            return 0;
        }
        2usize.pow((winning_numbers.len() - 1) as u32)
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
        Ok(Card {
            id: card_id,
            winning_numbers,
            card_numbers,
        })
    }
}
