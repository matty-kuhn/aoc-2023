use crate::days::get_lines;

use super::Day;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

pub struct Day1 {
    input: String,
}

impl Day1 {
    pub fn new(input: String) -> Day1 {
        Day1 { input }
    }

    fn replace_nums(&self) -> String {
        let mut replaced = self.input.clone();
        for (idx, num) in NUMS.iter().enumerate() {
            // put first and last letters back to be reused by other words
            replaced = replaced.replace(
                num,
                &format!(
                    "{}{}{}",
                    num.chars().nth(0).unwrap(),
                    idx + 1,
                    num.chars().nth_back(0).unwrap()
                ),
            );
        }
        replaced
    }

    fn search_for_sum(lines: &Vec<&str>) -> u32 {
        let mut sum = 0;
        for line in lines {
            // search from front of line for the first digit
            let mut line_chars = line.chars();
            let first_num = loop {
                let char = line_chars.next();
                let Some(char) = char else { break None };
                if char.is_digit(10) {
                    break char.to_digit(10);
                }
            };
            let Some(first_num) = first_num else {
                panic!("No digits found in line")
            };
            // search from back of line for the first digit
            let mut line_chars = line.chars();
            let last_num = loop {
                let char = line_chars.next_back();
                let Some(char) = char else { break None };
                if char.is_digit(10) {
                    break char.to_digit(10);
                }
            };
            let Some(last_num) = last_num else {
                panic!("No digits found in line")
            };
            // the first digit multiply by 10, the last add to that, add to sum
            let num = first_num * 10 + last_num;
            sum += num;
        }
        sum
    }
}

impl Day for Day1 {
    fn part1(&self) -> String {
        let lines = get_lines(&self.input);
        format!("{}", Self::search_for_sum(&lines))
    }

    fn part2(&self) -> String {
        let replaced = self.replace_nums();
        let lines = get_lines(&replaced);
        format!("{}", Self::search_for_sum(&lines))
    }
}
