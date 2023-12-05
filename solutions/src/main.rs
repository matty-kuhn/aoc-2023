use days::{day1::Day1, day2::Day2, day3::Day3, day4::Day4, Day};

pub mod days;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            let days = vec![
                Box::new(Day1::new(days::get_day_input("day1_test"))) as Box<dyn Day>,
                Box::new(Day2::new(days::get_day_input("day2_test"))) as Box<dyn Day>,
                Box::new(Day3::new(days::get_day_input("day3_test"))) as Box<dyn Day>,
                Box::new(Day4::new(days::get_day_input("day4_test"))) as Box<dyn Day>,
            ];
        }
        else {
            let days = vec![
                Box::new(Day1::new(days::get_day_input("day1"))) as Box<dyn Day>,
                Box::new(Day2::new(days::get_day_input("day2"))) as Box<dyn Day>,
                Box::new(Day3::new(days::get_day_input("day3"))) as Box<dyn Day>,
                Box::new(Day4::new(days::get_day_input("day4"))) as Box<dyn Day>,
            ];
        }
    }
    for (idx, day) in days.iter().enumerate() {
        println!("Day {}", idx + 1);
        println!("\tPart 1: {}", day.part1());
        println!("\tPart 2: {}", day.part2());
    }
}
