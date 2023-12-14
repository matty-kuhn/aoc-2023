use memoize::memoize;
use std::{ops::MulAssign, str::FromStr};

use super::{get_lines, Day};

pub struct Day12 {
    input: String,
}

impl Day12 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Row> {
        get_lines(&self.input)
            .iter()
            .map(|line| line.parse::<Row>().unwrap())
            .collect()
    }
}

#[memoize]
fn count_solutions(map: String, counts: Vec<i64>, prev_in_group: Option<i64>) -> i64 {
    // shoutout https://github.com/fuglede/ for reminding me what dp is
    let prev_in_group = prev_in_group.unwrap_or(0);
    if map.is_empty() {
        // if we closed all groups, string is empty, and counts empty, we got a solution
        if counts.is_empty() && prev_in_group == 0 {
            return 1;
        }
        return 0;
    }
    let mut solutions = 0;
    let next_chars = if map.starts_with("?") {
        // want to check for solutions with both '.' and '#'
        vec!['.', '#']
    } else {
        // otherwise, just check for the next char
        vec![map.chars().next().unwrap()]
    };
    for next in next_chars {
        if next == '#' {
            // if we have a #, we want to chase that group down
            solutions += count_solutions(
                map[1..].to_string(),
                counts.clone(),
                Some(prev_in_group + 1),
            );
        } else {
            if prev_in_group > 0 {
                if !counts.is_empty() && counts[0] == prev_in_group {
                    // if we have a '.', we want to close the group if we have one open
                    solutions += count_solutions(map[1..].to_string(), counts[1..].to_vec(), None);
                }
            } else {
                // not in group, move to next
                solutions += count_solutions(map[1..].to_string(), counts.clone(), None);
            }
        }
    }

    solutions
}

impl Day for Day12 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let mut rows = self.parse_input();
        let total_solutions = rows
            .iter_mut()
            .map(|row| {
                row.map.push_str(".");
                count_solutions(row.map.clone(), row.counts.clone(), None)
            })
            .sum::<i64>();
        println!("part 1 Time: {:#?}", start_time.elapsed());
        format!("{}", total_solutions)
    }

    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        let mut rows = self.parse_input();
        let total_solutions = rows
            .iter_mut()
            .map(|row| {
                *row *= 5;
                row.map.push_str(".");
                count_solutions(row.map.clone(), row.counts.clone(), None)
            })
            .sum::<i64>();
        println!("part 2 Time: {:#?}", start_time.elapsed());
        format!("{}", total_solutions)
    }
}

#[derive(Debug, Clone)]
struct Row {
    counts: Vec<i64>,
    map: String,
}

impl MulAssign<i64> for Row {
    fn mul_assign(&mut self, rhs: i64) {
        let delim = "?";
        let copy = self.map.clone();
        for _ in 0..rhs - 1 {
            self.map.push_str(delim);
            self.map.push_str(&copy);
        }
        self.counts = self
            .counts
            .iter()
            .cycle()
            .take(self.counts.len() * rhs as usize)
            .cloned()
            .collect();
    }
}

impl FromStr for Row {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();

        Ok(Self {
            counts: split
                .last()
                .unwrap()
                .split(",")
                .map(|s| s.parse::<i64>().unwrap())
                .collect(),
            map: split[0].to_string(),
        })
    }
}
