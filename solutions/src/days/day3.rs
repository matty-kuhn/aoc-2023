use crate::days::get_lines;
use array_tool::vec::Union;
use std::collections::HashMap;

use super::Day;

type Point = (usize, usize);
type PointsToCheck = Vec<Point>;
type NumsToIndicies = HashMap<(Point, usize), PointsToCheck>;

pub struct Day3 {
    input: String,
}

impl Day3 {
    pub fn new(input: String) -> Day3 {
        Day3 { input }
    }

    // add each point surrounding the given point to the points to check
    fn get_points_to_check(row_idx: usize, col_idx: usize) -> PointsToCheck {
        let mut points_to_check = PointsToCheck::new();
        let row_idx_to_check = row_idx.saturating_sub(1);
        let col_idx_to_check = col_idx.saturating_sub(1);
        for row_idx_to_check in row_idx_to_check..=row_idx + 1 {
            for col_idx_to_check in col_idx_to_check..=col_idx + 1 {
                if row_idx_to_check != row_idx || col_idx_to_check != col_idx {
                    points_to_check.push((row_idx_to_check, col_idx_to_check));
                }
            }
        }
        points_to_check
    }

    fn find_all_nums(lines: &[&str]) -> NumsToIndicies {
        let mut nums_to_indicies: NumsToIndicies = HashMap::new();
        let mut curr_num = String::new();
        let mut curr_check = PointsToCheck::new();
        // iterate over each line
        for (row_idx, line) in lines.iter().enumerate() {
            // iterate over each char in the line
            for (col_idx, char) in line.chars().enumerate() {
                // if the char is a digit, add it to the current number
                // also add the indicies to check for this number
                if char.is_ascii_digit() {
                    curr_num.push(char);
                    curr_check = curr_check.union(Self::get_points_to_check(row_idx, col_idx));
                } else {
                    // if the char is not a digit, we've reached the end of the number
                    // so add the number and indicies to check to the hashmap
                    if !curr_num.is_empty() {
                        let num = curr_num.parse::<usize>().unwrap();
                        nums_to_indicies.insert(((row_idx, col_idx), num), curr_check);
                    }
                    // reset the current number and indicies to check
                    curr_check = PointsToCheck::new();
                    curr_num = String::new();
                }
            }
        }
        nums_to_indicies
    }
}

impl Day for Day3 {
    fn part1(&self) -> String {
        let lines = get_lines(&self.input);
        let nums_to_indicies = Self::find_all_nums(&lines);
        // we now have a hashmap of each number and the indicies to check for that number
        // so we can check if the number is touching something other than a .
        // if it is, we will add it to the sum
        let mut sum = 0;
        for entry in nums_to_indicies.iter() {
            let ((_row_idx, _col_idx), num) = entry.0;
            let points_to_check = entry.1;
            let mut touching = false;
            for point in points_to_check {
                let (row_idx_to_check, col_idx_to_check) = point;
                //check that the point is in bounds
                if *row_idx_to_check >= lines.len()
                    || *col_idx_to_check >= lines[*row_idx_to_check].len()
                {
                    continue;
                }
                let char_to_check = lines[*row_idx_to_check]
                    .chars()
                    .nth(*col_idx_to_check)
                    .unwrap();
                if char_to_check != '.' && !char_to_check.is_ascii_digit() {
                    touching = true;
                    break;
                }
            }
            if touching {
                sum += num;
            }
        }
        format!("{sum}")
    }

    fn part2(&self) -> String {
        let lines = get_lines(&self.input);
        let nums_to_indicies = Self::find_all_nums(&lines);
        // a hashmap of all gears, with a count of how many nums they touch, and their ratio
        let mut gears: HashMap<Point, (usize, usize)> = HashMap::new();
        for entry in nums_to_indicies.iter() {
            let ((_row_idx, _col_idx), num) = entry.0;
            let points_to_check = entry.1;
            for point in points_to_check {
                let (row_idx_to_check, col_idx_to_check) = point;
                //check that the point is in bounds
                if *row_idx_to_check >= lines.len()
                    || *col_idx_to_check >= lines[*row_idx_to_check].len()
                {
                    continue;
                }
                let char_to_check = lines[*row_idx_to_check]
                    .chars()
                    .nth(*col_idx_to_check)
                    .unwrap();
                if char_to_check == '*' {
                    let gear = gears.entry(*point).or_insert((0, 0));
                    if gear.0 == 0 {
                        gear.1 = *num;
                    } else if gear.0 == 1 {
                        gear.1 *= *num;
                    } else {
                        // set the gear power to 0, since it touches more than 2 nums
                        gear.1 *= 0;
                    }
                    gear.0 += 1;
                }
            }
        }
        // sum the gear powers, but only if they touch 2 nums
        let sum = gears.iter().fold(0, |acc, entry| {
            let (_, (count, power)) = entry;
            if *count == 2 {
                acc + *power
            } else {
                acc
            }
        });
        format!("{sum}")
    }
}
