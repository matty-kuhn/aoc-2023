use std::collections::HashMap;

use super::{get_lines, Day};

pub struct Day9 {
    input: String,
}

impl Day9 {
    pub fn new(input: String) -> Day9 {
        Day9 { input }
    }

    fn get_next_num(sequence: &Vec<i64>, back: bool) -> i64 {
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                println!("sequence: {:?}", sequence);
            }
        }
        let mut counts = HashMap::new();
        let mut differences = Vec::new();
        let mut prev_num = sequence[0];
        for (idx, num) in sequence.iter().enumerate() {
            // get difference
            if idx != 0 {
                let difference = num - prev_num;
                differences.push(difference);
            }
            // add to counts
            let count = counts.entry(num).or_insert(0);
            *count += 1;
            prev_num = *num;
        }
        if counts.len() == 1 {
            // it is either all same num or all 0s
            // so next number in either will be just the same number
            if back {
                return sequence[0];
            } else {
                return sequence[sequence.len() - 1];
            }
        }

        if back {
            sequence[sequence.len() - 1] + Self::get_next_num(&differences, back)
        } else {
            sequence[0] - Self::get_next_num(&differences, back)
        }
    }

    fn get_sequences(&self) -> Vec<Vec<i64>> {
        get_lines(&self.input)
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>()
    }
}

impl Day for Day9 {
    fn part1(&self) -> String {
        let sequences = self.get_sequences();
        let mut next_nums = Vec::new();
        for sequence in sequences {
            let next_num = Self::get_next_num(&sequence, true);
            next_nums.push(next_num);
        }
        format!("{}", next_nums.iter().sum::<i64>())
    }

    fn part2(&self) -> String {
        let sequences = self.get_sequences();
        let mut next_nums = Vec::new();
        for sequence in sequences {
            let next_num = Self::get_next_num(&sequence, false);
            next_nums.push(next_num);
        }
        format!("{}", next_nums.iter().sum::<i64>())
    }
}
