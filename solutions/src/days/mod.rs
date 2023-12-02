use std::fs;



pub mod day1;
pub mod day2;

pub trait Day {
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub fn get_day_input(day: &str) -> String {
    let input = fs::read_to_string(format!("inputs/{}.txt", day))
        .expect("Something went wrong reading the file");
    input
}

fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}
