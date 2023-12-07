use std::iter::zip;

use super::{get_lines, Day};

pub struct Day6 {
    input: String,
}

impl Day6 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn get_races(&self) -> Vec<Race> {
        let lines = get_lines(&self.input);
        let mut races = Vec::new();
        let times = lines[0]
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        let distances = lines[1]
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<f64>().unwrap())
            .collect::<Vec<f64>>();
        for (time, distance) in zip(times, distances) {
            races.push(Race::new(time, distance));
        }
        races
    }

    fn get_big_race(&self) -> Race {
        let lines = get_lines(&self.input);
        let time = lines[0]
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<f64>()
            .unwrap();
        let distance = lines[1]
            .split(':')
            .nth(1)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<f64>()
            .unwrap();
        Race::new(time, distance)
    }

    /// for this, a will be -1, b will be the time, and c will be the distance
    fn quadratic_formula(a: f64, b: f64, c: f64) -> (f64, f64) {
        let discriminant = b.powi(2) - 4.0 * a * c;
        let x1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let x2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (x1, x2)
    }
}

impl Day for Day6 {
    fn part1(&self) -> String {
        let races = self.get_races();
        let mut total_possible: u64 = 1;
        for race in races {
            let (min, max) = Self::quadratic_formula(-1.0, race.time, -1.0 * race.distance);
            let min = (min + 1.0).floor() as u64;
            let max = (max - 1.0).ceil() as u64;
            total_possible *= max - min + 1;
            cfg_if::cfg_if! {
                if #[cfg(debug_assertions)] {
                    println!("min: {}, max: {}", min, max);
                    println!("total_possible: {}", total_possible);
                }
            }
        }
        format!("{total_possible}")
    }

    fn part2(&self) -> String {
        let race = self.get_big_race();
        let (min, max) = Self::quadratic_formula(-1.0, race.time, -1.0 * race.distance);
        let min = (min + 1.0).floor() as u64;
        let max = (max - 1.0).ceil() as u64;
        let total_possible = max - min + 1;
        format!("{total_possible}")
    }
}

#[derive(Debug)]
struct Race {
    time: f64,
    distance: f64,
}

impl Race {
    fn new(time: f64, distance: f64) -> Self {
        Self { time, distance }
    }
}
