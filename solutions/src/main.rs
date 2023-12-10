use clap::Parser;
use days::{day_builder, Day};

use crate::days::CURRENT_DAY;

pub mod days;

#[derive(Parser, Debug)]
struct Cli {
    /// The day to run
    day: Option<i8>,
    /// the part to run
    part: Option<u8>,
}

fn main() {
    let args = Cli::parse();
    let day = args.day.unwrap_or(-1);
    let mut days: Vec<(i8, Box<dyn Day>)> = Vec::new();
    cfg_if::cfg_if! {
        if #[cfg(debug_assertions)] {
            if day == -1 {
                for day_num in 1..=CURRENT_DAY {
                    days.push((day_num, day_builder(day_num, &format!("day{}_test", day_num))));
                }
            } else {
                days.push((day, day_builder(day, &format!("day{}_test", day))));
            }
        }
        else {
            if day == -1 {
                for day_num in 1..=CURRENT_DAY {
                    days.push((day_num, day_builder(day_num, &format!("day{}", day_num))));
                }
            } else {
                days.push((day, day_builder(day, &format!("day{}", day))));
            }
        }
    }
    for (idx, day) in days {
        println!("Day {}", idx);
        if let Some(part) = args.part {
            match part {
                1 => println!("\tPart 1: {}", day.part1()),
                2 => println!("\tPart 2: {}", day.part2()),
                _ => panic!("Part {} not implemented yet", part),
            }
            continue;
        } else {
            println!("\tPart 1: {}", day.part1());
            println!("\tPart 2: {}", day.part2());
        }
    }
}
