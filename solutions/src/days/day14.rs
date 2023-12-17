use std::collections::VecDeque;

use memoize::memoize;

use super::Day;

pub struct Day14 {
    input: String,
}

impl Day14 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn get_columns(&self) -> Vec<Vec<char>> {
        let mut columns = vec![vec![]; self.input.lines().next().unwrap().len()];
        for line in self.input.lines() {
            for (i, c) in line.chars().enumerate() {
                columns[i].push(c);
            }
        }
        columns
    }

    fn get_spot_load(idx: usize, len: usize) -> usize {
        len - idx
    }

    fn get_col_weight(col: Vec<char>) -> usize {
        // we assume that we want the weight directed at whatever the front is, for part 1
        // the front will be north
        // this will be a queue of open indicies, to keep track of where boulders can roll
        let mut weight = 0;
        let mut open_spots = VecDeque::new();
        let col_len = col.len();
        for (idx, rock) in col.iter().enumerate() {
            match rock {
                '.' => {
                    open_spots.push_back(idx);
                }
                'O' => {
                    // if there is an empty spot, the boulder rolls to the first one, so we get
                    // the weight of that spot
                    if let Some(open_idx) = open_spots.pop_front() {
                        weight += Self::get_spot_load(open_idx, col_len);
                        // this boulder rolled, so its spot is now open
                        open_spots.push_back(idx);
                    } else {
                        // no empty spots, so calculate in place
                        weight += Self::get_spot_load(idx, col_len);
                    }
                }
                '#' => {
                    // this boulder can't move, and also clears all open spots in front of it
                    open_spots.clear();
                }
                _ => panic!("invalid"),
            }
        }

        weight
    }

    fn move_rocks(col: Vec<char>) -> Vec<char> {
        // this can work for cols and also rows
        let mut new_col = Vec::new();
        let mut open_spots = VecDeque::new();
        for (idx, rock) in col.iter().enumerate() {
            match rock {
                '.' => {
                    open_spots.push_back(idx);
                    new_col.push('.');
                }
                'O' => {
                    // if there is an empty spot, the boulder rolls to the first one, so we get
                    // the weight of that spot
                    if let Some(open_idx) = open_spots.pop_front() {
                        new_col[open_idx] = 'O';
                        // this boulder rolled, so its spot is now open
                        open_spots.push_back(idx);
                        new_col.push('.');
                    } else {
                        // no empty spots, so put it here
                        new_col.push('O');
                    }
                }
                '#' => {
                    // this boulder can't move, and also clears all open spots in front of it
                    open_spots.clear();
                    new_col.push('#');
                }
                _ => panic!("invalid"),
            }
        }
        new_col
    }
}

#[memoize]
fn get_col_weight_in_place(col: Vec<char>) -> usize {
    let mut weight = 0;
    let col_len = col.len();
    for (idx, rock) in col.iter().enumerate() {
        if rock == &'O' {
            weight += Day14::get_spot_load(idx, col_len);
        }
    }
    weight
}

#[memoize]
fn do_spin_cycle(cols: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_cols = Vec::new();
    // we start with cols north-south
    for col in cols {
        new_cols.push(Day14::move_rocks(col));
    }
    // now we tanspose the cols to be west-east
    let mut we_cols = vec![vec![]; new_cols[0].len()];
    for col in new_cols {
        for (i, c) in col.iter().enumerate() {
            we_cols[i].push(*c);
        }
    }
    // move again
    let mut new_cols = Vec::new();
    for col in we_cols {
        new_cols.push(Day14::move_rocks(col));
    }
    // transpose back to north south, but with south at the beginning now
    let mut sn_cols = vec![vec![]; new_cols[0].len()];
    // going backwards to do south to north
    for col in new_cols.iter().rev() {
        for (i, c) in col.iter().enumerate() {
            sn_cols[i].push(*c);
        }
    }
    // move south
    let mut new_cols = Vec::new();
    for col in sn_cols {
        new_cols.push(Day14::move_rocks(col));
    }
    // transpose to east-west
    let mut ew_cols = vec![vec![]; new_cols[0].len()];
    // going backwards to do east to west
    for col in new_cols.iter().rev() {
        for (i, c) in col.iter().enumerate() {
            ew_cols[i].push(*c);
        }
    }
    // last roll
    let mut new_cols = Vec::new();
    for col in ew_cols {
        new_cols.push(Day14::move_rocks(col));
    }
    // transpose back to north-south, so both are reverse
    let mut ns_cols = vec![vec![]; new_cols[0].len()];
    for col in new_cols.iter().rev() {
        for (i, c) in col.iter().rev().enumerate() {
            ns_cols[i].push(*c);
        }
    }
    ns_cols
}

impl Day for Day14 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        // once we have columns, calculate the load in each one then sum them
        let weight: usize = self
            .get_columns()
            .iter()
            .map(|col| Self::get_col_weight(col.clone()))
            .sum();
        println!("part 1 took {:?}", start_time.elapsed());
        format!("{weight}")
    }
    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        // cols facing north/south
        let mut ns_cols = self.get_columns();
        // use this to detect cycles
        let mut hist_vec: Vec<Vec<Vec<char>>> = Vec::new();
        let iters = 1_000_000_000;
        for _ in 0..iters {
            ns_cols = do_spin_cycle(ns_cols);
            if let Some(first_idx) = hist_vec.iter().position(|x| *x == ns_cols) {
                let cycle_len = hist_vec.len() - first_idx;
                let cycle_iter = ((iters - first_idx) % cycle_len + first_idx) - 1;
                ns_cols = hist_vec[cycle_iter].clone();
                break;
            } else {
                hist_vec.push(ns_cols.clone());
            }
        }
        let weight = ns_cols
            .iter()
            .map(|col| get_col_weight_in_place(col.clone()))
            .sum::<usize>();
        println!("part 2 took {:?}", start_time.elapsed());
        format!("{weight}")
    }
}
