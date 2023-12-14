use std::fs;

use self::{
    day1::Day1, day10::Day10, day11::Day11, day2::Day2, day3::Day3, day4::Day4, day5::Day5,
    day6::Day6, day7::Day7, day8::Day8, day9::Day9, day12::Day12,
};

pub const CURRENT_DAY: i8 = 12;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day12;

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

pub fn day_builder(day: i8, input_name: &str) -> Box<dyn Day> {
    match day {
        1 => Box::new(Day1::new(get_day_input(input_name))) as Box<dyn Day>,
        2 => Box::new(Day2::new(get_day_input(input_name))) as Box<dyn Day>,
        3 => Box::new(Day3::new(get_day_input(input_name))) as Box<dyn Day>,
        4 => Box::new(Day4::new(get_day_input(input_name))) as Box<dyn Day>,
        5 => Box::new(Day5::new(get_day_input(input_name))) as Box<dyn Day>,
        6 => Box::new(Day6::new(get_day_input(input_name))) as Box<dyn Day>,
        7 => Box::new(Day7::new(get_day_input(input_name))) as Box<dyn Day>,
        8 => Box::new(Day8::new(get_day_input(input_name))) as Box<dyn Day>,
        9 => Box::new(Day9::new(get_day_input(input_name))) as Box<dyn Day>,
        10 => Box::new(Day10::new(get_day_input(input_name))) as Box<dyn Day>,
        11 => Box::new(Day11::new(get_day_input(input_name))) as Box<dyn Day>,
        12 => Box::new(Day12::new(get_day_input(input_name))) as Box<dyn Day>,
        _ => panic!("Day {} not implemented yet", day),
    }
}
