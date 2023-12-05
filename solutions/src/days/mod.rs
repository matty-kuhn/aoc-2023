use std::fs;

use self::{day1::Day1, day2::Day2, day3::Day3, day4::Day4};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn get_day_input(day: &str) -> String {
    fs::read_to_string(format!("inputs/{}.txt", day))
        .expect("Something went wrong reading the file")
}

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

pub fn day_builder(day: usize, input_name: &str) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1::new(get_day_input(input_name))) as Box<dyn Day>,
        2 => Box::new(Day2::new(get_day_input(input_name))) as Box<dyn Day>,
        3 => Box::new(Day3::new(get_day_input(input_name))) as Box<dyn Day>,
        4 => Box::new(Day4::new(get_day_input(input_name))) as Box<dyn Day>,
        _ => panic!("Day {} not implemented yet", day),
    }
}
