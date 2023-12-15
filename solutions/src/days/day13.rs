use super::{get_lines, Day};

pub struct Day13 {
    input: String,
}

impl Day13 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Puzzle> {
        let mut puzzles = Vec::new();
        let mut current_puzzle = Vec::new();
        for line in get_lines(&self.input) {
            if line.is_empty() {
                puzzles.push(Puzzle::from(current_puzzle));
                current_puzzle = Vec::new();
            } else {
                current_puzzle.push(line.to_string());
            }
        }
        puzzles.push(Puzzle::from(current_puzzle));

        puzzles
    }

    fn palindrome_verify(nums: &Vec<u128>, left: usize, right: usize) -> Option<usize> {
        let mut palindrome_idx = None;
        let mut left_idx = left;
        let mut right_idx = right;
        while left_idx < right_idx {
            if nums[left_idx] != nums[right_idx] {
                break;
            }
            left_idx += 1;
            right_idx -= 1;
        }
        if left_idx >= right_idx {
            palindrome_idx =
                Some(((left_idx as f64 + right_idx as f64) / 2 as f64).floor() as usize);
        }
        palindrome_idx
    }

    // we want to figure out where the palindrome is in the vec itself
    // returns the index of the midpoint of the palindrome, inclusive
    fn find_palindrome(nums: Vec<u128>) -> Option<usize> {
        // check from left
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[nums.len() - 1] {
                if let Some(idx) = Self::palindrome_verify(&nums, i + 1, nums.len() - 2) {
                    return Some(idx);
                };
            }
        }
        // check from right
        for i in (1..nums.len()).rev() {
            if nums[i] == nums[0] {
                if let Some(idx) = Self::palindrome_verify(&nums, 1, i - 1) {
                    return Some(idx);
                };
            }
        }
        None
    }

    fn get_puzzle_soln(puzzle: Puzzle) -> (usize, usize) {
        let mut nums_left = 0;
        let mut nums_above = 0;
        // check rows for palindrome
        if let Some(num_above) = Self::find_palindrome(puzzle.rows) {
            nums_above += num_above + 1;
        };
        if nums_above == 0 {
            // check cols for palindrome
            if let Some(num_to_left) = Self::find_palindrome(puzzle.cols) {
                nums_left += num_to_left + 1;
            };
        }
        (nums_left, nums_above)
    }

    fn get_puzzle_soln_pt2(puzzle: Puzzle) -> (usize, usize) {
        let mut nums_left = 0;
        let mut nums_above = 0;
        // check rows for palindrome
        if let Some(num_above) = find_palindrome_mult(puzzle.rows, puzzle.rows_max_idx) {
            println!("found row palindrome {}", (num_above + 1) * 100);
            nums_above += num_above + 1;
        };
        if nums_above == 0 {
            // check cols for palindrome
            if let Some(num_to_left) = find_palindrome_mult(puzzle.cols, puzzle.cols_max_idx) {
                println!("found col palindrome {}", num_to_left + 1);
                nums_left += num_to_left + 1;
            };
        }
        if nums_left == 0 && nums_above == 0 {
            panic!("no palindrome found");
        }
        (nums_left, nums_above)
    }
}

/// returns all possible values of this number after bitshifts, representing changing the smudges
fn transform_num(num: u128, max_idx: usize) -> Vec<u128> {
    let mut nums = Vec::new();
    for i in 0..max_idx + 1 {
        nums.push(num ^ 2u128.pow(i as u32));
    }
    nums
}

fn find_palindrome_mult(nums: Vec<u128>, max_idx: usize) -> Option<usize> {
    // we check a moving line to see if two given items are one off from each other
    for idx in 0..nums.len() - 1 {
        if fuzzy_match(nums[idx], nums[idx + 1], max_idx) {
            if palindrome_verify_mult(nums.clone(), idx, max_idx) {
                return Some(idx);
            }
        }
    }
    // so we have the original center line, we now need to search for real mirror center lines with off by one other rows
    // this is basically palindrome_verify but inside out, and with the fuzzy match
    for idx in 0..nums.len() - 1 {
        if nums[idx] == nums[idx + 1] {
            if let Some(idx) = palindrome_verify_mult_fuzzy(nums.clone(), idx, max_idx) {
                return Some(idx);
            }
        }
    }
    None
}

fn palindrome_verify_mult(nums: Vec<u128>, idx: usize, max_idx: usize) -> bool {
    // start working our way out
    if idx == 0 {
        return fuzzy_match(nums[idx], nums[idx + 1], max_idx);
    }
    if idx == nums.len() - 1 {
        return fuzzy_match(nums[idx], nums[idx - 1], max_idx);
    }
    if idx == nums.len() - 2 {
        return fuzzy_match(nums[idx], nums[idx + 1], max_idx);
    }
    let mut left = idx - 1;
    let mut right = idx + 2;
    loop {
        if left == 0 || right >= nums.len() - 1 {
            // we are at the ends, so if they equal we gucci
            if right <= nums.len() - 1 {
                break nums[left] == nums[right];
            }
            break false;
        }
        if nums[left] == nums[right] {
            // keep going
            left -= 1;
            right += 1;
        } else {
            // we are at the end, so this isn't it
            break false;
        }
    }
}

fn palindrome_verify_mult_fuzzy(nums: Vec<u128>, idx: usize, max_idx: usize) -> Option<usize> {
    // start working our way out
    if idx == 0 || idx == nums.len() - 1 {
        // this isn't it because no smudges were changed
        return None;
    }
    let mut left = idx - 1;
    let mut right = idx + 2;
    let mut fuzzy_found = false;
    loop {
        if left == 0 || right >= nums.len() - 1 {
            if right <= nums.len() - 1 {
                // we are at the ends, so if they equal we gucci
                if fuzzy_match(nums[left], nums[right], max_idx) {
                    if fuzzy_found {
                        // we already found one, so this isn't it
                        break None;
                    }
                    fuzzy_found = true;
                }
            }
            if fuzzy_found {
                // we found one, so this is it
                return Some(idx);
            }
            // we are at the end, so this isn't it
            break None;
        }
        if fuzzy_match(nums[left], nums[right], max_idx) {
            // keep going
            if fuzzy_found {
                // we already found one, so this isn't it
                break None;
            }
            fuzzy_found = true;
            left -= 1;
            right += 1;
        } else if nums[left] == nums[right] {
            // keep going
            left -= 1;
            right += 1;
        } else {
            // no matches
            break None;
        }
    }
}

fn fuzzy_match(num1: u128, num2: u128, max_idx: usize) -> bool {
    // check if we can match this number to a number in the array
    let nums_to_check = transform_num(num1, max_idx);
    for num in nums_to_check {
        if num == num2 {
            return true;
        }
    }
    let nums_to_check = transform_num(num2, max_idx);
    for num in nums_to_check {
        if num == num1 {
            return true;
        }
    }
    false
}

impl Day for Day13 {
    fn part1(&self) -> String {
        let puzzles = self.parse_input();
        let mut nums_left = 0;
        let mut nums_above = 0;
        for puzzle in puzzles {
            let (left, above) = Self::get_puzzle_soln(puzzle);
            nums_left += left;
            nums_above += above;
        }
        format!("{}", nums_left + (nums_above * 100))
    }

    fn part2(&self) -> String {
        let puzzles = self.parse_input();
        let mut nums_left = 0;
        let mut nums_above = 0;
        // for this one, we are going to change each row/col value by one bit to see if we can get a new value
        for puzzle in puzzles {
            let (left, above) = Self::get_puzzle_soln_pt2(puzzle);
            nums_left += left;
            nums_above += above;
        }
        format!("{}", nums_left + (nums_above * 100))
    }
}

#[derive(Debug, Clone)]
struct Puzzle {
    cols: Vec<u128>,
    cols_max_idx: usize,
    rows: Vec<u128>,
    rows_max_idx: usize,
}

impl From<Vec<String>> for Puzzle {
    fn from(value: Vec<String>) -> Self {
        let mut cols = Vec::new();
        let mut rows = Vec::new();
        for y in 0..value.len() {
            let mut row_num = 0;
            for x in 0..value[y].len() {
                let c = value[y].chars().nth(x).unwrap();
                if c == '#' {
                    row_num += 2u128.pow(x as u32);
                }
            }
            rows.push(row_num);
        }
        for x in 0..value[0].len() {
            let mut col_num = 0;
            for y in 0..value.len() {
                let c = value[y].chars().nth(x).unwrap();
                if c == '#' {
                    col_num += 2u128.pow(y as u32);
                }
            }
            cols.push(col_num);
        }

        let cols_max_idx = rows.len() - 1;
        let rows_max_idx = cols.len() - 1;
        Self {
            cols,
            rows,
            cols_max_idx,
            rows_max_idx,
        }
    }
}
