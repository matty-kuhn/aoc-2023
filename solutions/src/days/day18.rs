use std::str::FromStr;

use super::{get_lines, Day};

pub struct Day18 {
    input: String,
}

impl Day18 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Step> {
        get_lines(&self.input)
            .iter()
            .map(|line| line.parse().unwrap())
            .collect()
    }

    /// returns a bunch of 2x2 arrays for us to do the shoelace formula on
    /// returns as (x1, y1), (x2, y2)
    fn get_matrices(steps: Vec<Step>) -> Vec<((isize, isize), (isize, isize))> {
        let mut map = vec![];
        let mut matricies = vec![];
        let mut x = 0;
        let mut y = 0;
        let mut min_x = 0;
        let mut min_y = 0;
        for step in steps {
            let start_x = x;
            let start_y = y;
            match step.direction {
                Direction::Up => y -= step.length,
                Direction::Down => y += step.length,
                Direction::Right => x += step.length,
                Direction::Left => x -= step.length,
            }
            map.push(Trench {
                start_x,
                start_y,
                end_x: x,
                end_y: y,
            });
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
        }
        // normalize the map
        for trench in &mut map {
            trench.start_x -= min_x;
            trench.end_x -= min_x;
            trench.start_y -= min_y;
            trench.end_y -= min_y;
            let insert = (
                (trench.start_x, trench.start_y),
                (trench.end_x, trench.end_y),
            );
            if !matricies.contains(&insert) {
                matricies.push(insert);
            }
        }
        matricies
    }

    // coming in as (x1,y1,x2,y2)
    fn shoelace_formula(matricies: Vec<((isize, isize), (isize, isize))>) -> isize {
        let mut sum = 0;
        for matrix in matricies {
            let (x1, y1) = matrix.0;
            let (x2, y2) = matrix.1;
            sum += (y1 + y2) * (x1 - x2);
            sum += x1.abs_diff(x2) as isize;
            sum += y1.abs_diff(y2) as isize;
        }
        sum / 2 + 1
    }

    fn convert_steps(steps: Vec<Step>) -> Vec<Step> {
        let mut new_steps = vec![];
        for value in steps {
            let color = value.color;
            let direction = color.chars().last().unwrap();
            let direction = match direction {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                _ => panic!("unknown direction"),
            };
            let length = isize::from_str_radix(&color[1..color.len() - 1], 16).unwrap();
            new_steps.push(Step {
                direction,
                length,
                color,
            })
        }
        new_steps
    }
}

impl Day for Day18 {
    fn part1(&self) -> String {
        let start = std::time::Instant::now();
        let steps = self.parse_input();
        let mats = Self::get_matrices(steps);
        let inner = Self::shoelace_formula(mats);
        println!("Time: {:?}", start.elapsed());
        format!("{inner}")
    }

    fn part2(&self) -> String {
        let start = std::time::Instant::now();
        let steps = self.parse_input();
        let steps = Self::convert_steps(steps);
        let mats = Self::get_matrices(steps);
        let inner = Self::shoelace_formula(mats);
        println!("Time: {:?}", start.elapsed());
        format!("{inner}")
    }
}

#[derive(Clone, Debug)]
struct Trench {
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
}

#[derive(Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Step {
    direction: Direction,
    length: isize,
    color: String,
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // str looks like: D 3 (#ffffff)
        let mut split = s.split_whitespace();
        let direction = split.next().unwrap().parse().unwrap();
        let length: isize = split.next().unwrap().parse().unwrap();
        let color = split
            .next()
            .unwrap()
            .to_string()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .to_string();
        Ok(Self {
            direction,
            length,
            color,
        })
    }
}
