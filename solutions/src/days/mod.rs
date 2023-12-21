use std::fs;

use self::{
    day1::Day1, day10::Day10, day11::Day11, day12::Day12, day13::Day13, day14::Day14, day15::Day15,
    day16::Day16, day17::Day17, day18::Day18, day2::Day2, day3::Day3, day4::Day4, day5::Day5,
    day6::Day6, day7::Day7, day8::Day8, day9::Day9,
};

pub const CURRENT_DAY: i8 = 16;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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
        13 => Box::new(Day13::new(get_day_input(input_name))) as Box<dyn Day>,
        14 => Box::new(Day14::new(get_day_input(input_name))) as Box<dyn Day>,
        15 => Box::new(Day15::new(get_day_input(input_name))) as Box<dyn Day>,
        16 => Box::new(Day16::new(get_day_input(input_name))) as Box<dyn Day>,
        17 => Box::new(Day17::new(get_day_input(input_name))) as Box<dyn Day>,
        18 => Box::new(Day18::new(get_day_input(input_name))) as Box<dyn Day>,
        // 19 => Box::new(Day19::new(get_day_input(input_name))) as Box<dyn Day>,
        // 20 => Box::new(Day20::new(get_day_input(input_name))) as Box<dyn Day>,
        // 21 => Box::new(Day21::new(get_day_input(input_name))) as Box<dyn Day>,
        // 22 => Box::new(Day22::new(get_day_input(input_name))) as Box<dyn Day>,
        // 23 => Box::new(Day23::new(get_day_input(input_name))) as Box<dyn Day>,
        // 24 => Box::new(Day24::new(get_day_input(input_name))) as Box<dyn Day>,
        // 25 => Box::new(Day25::new(get_day_input(input_name))) as Box<dyn Day>,
        _ => panic!("Day {} not implemented yet", day),
    }
}
