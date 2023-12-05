use std::collections::HashMap;

use clap::Parser;
use days::{day_builder, Day};

const CURRENT_DAY: usize = 4;
pub mod days;

#[derive(Parser, Debug)]
struct Cli {
    /// The day to run
    day: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let day = args.day.unwrap_or("all".to_string());
    let mut days: HashMap<usize, Box<dyn Day>> = HashMap::new();
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            if day == "all" {
                for day_num in 1..=CURRENT_DAY {
                    days.insert(day_num, day_builder(day_num, &format!("day{}_test", day_num)));
                }
            } else {
                let day_num = day.parse::<usize>().unwrap();
                days.insert(day_num, day_builder(day_num, &format!("day{}_test", day_num)));
            }
        }
        else {
            if day == "all" {
                for day_num in 1..=CURRENT_DAY {
                    days.insert(day_num, day_builder(day_num, &format!("day{}", day_num)));
                }
            } else {
                let day_num = day.parse::<usize>().unwrap();
                days.insert(day_num, day_builder(day_num, &format!("day{}", day_num)));
            }
        }
    }
    for (idx, day) in days {
        println!("Day {}", idx + 1);
        println!("\tPart 1: {}", day.part1());
        println!("\tPart 2: {}", day.part2());
    }
}
