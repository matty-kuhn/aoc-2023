use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

use super::{get_lines, Day};

pub struct Day8 {
    input: String,
}

impl Day8 {
    pub fn new(input: String) -> Day8 {
        Day8 { input }
    }

    // returns instructions, map
    fn parse_input(&self) -> (VecDeque<char>, HashMap<&str, (&str, &str)>) {
        let lines = get_lines(&self.input);
        let instructions = lines[0];
        let mut map = HashMap::new();
        // skip line 1 bc it is empty
        for line in lines[2..].iter() {
            let mut parts = line.split_whitespace();
            let key = parts.next().unwrap();
            // rest of parts is ["=", "(PART1,", "PART2)"]
            // drop =
            let _ = parts.next();
            // this has "(AAA,"
            let mut part1 = parts.next().unwrap();
            part1 = &part1[1..part1.len() - 1];
            // this has "BBB)"
            let mut part2 = parts.next().unwrap();
            part2 = &part2[..part2.len() - 1];
            map.insert(key, (part1, part2));
        }
        (instructions.chars().collect(), map)
    }

    fn get_num_steps(
        instructions: &mut VecDeque<char>,
        map: &HashMap<&str, (&str, &str)>,
        starting_key: &str,
    ) -> usize {
        let mut next_key = starting_key;
        let mut sum = 0;
        while let Some(direction) = instructions.pop_front() {
            match direction {
                'R' => {
                    let (_, right) = map.get(next_key).unwrap();
                    next_key = right;
                }
                'L' => {
                    let (left, _) = map.get(next_key).unwrap();
                    next_key = left;
                }
                _ => panic!("Invalid direction"),
            }
            sum += 1;
            if next_key == "ZZZ" {
                break;
            }
            instructions.push_back(direction);
        }
        sum
    }

    fn get_steps_to_end_z(
        instructions: &mut VecDeque<char>,
        map: &HashMap<&str, (&str, &str)>,
        starting_key: &str,
    ) -> usize {
        let mut next_key = starting_key;
        let mut sum = 0;
        while let Some(direction) = instructions.pop_front() {
            match direction {
                'R' => {
                    let (_, right) = map.get(next_key).unwrap();
                    next_key = right;
                }
                'L' => {
                    let (left, _) = map.get(next_key).unwrap();
                    next_key = left;
                }
                _ => panic!("Invalid direction"),
            }
            sum += 1;
            if next_key.ends_with('Z') {
                break;
            }
            instructions.push_back(direction);
        }
        sum
    }
}

impl Day for Day8 {
    fn part1(&self) -> String {
        let (mut instructions, map) = self.parse_input();
        format!("{}", Self::get_num_steps(&mut instructions, &map, "AAA"))
    }

    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        let (instructions, map) = self.parse_input();
        let mut a_enders = Vec::new();
        for key in map.keys() {
            if key.ends_with('A') {
                a_enders.push(key);
            }
        }
        let sums = a_enders
            .iter()
            .map(|key| Self::get_steps_to_end_z(&mut instructions.clone(), &map, key))
            .collect::<Vec<usize>>();
        let lcm = sums.iter().fold(1, |acc, x| lcm(acc, *x));
        let elapsed = start_time.elapsed();
        println!("Elapsed: {:?}", elapsed);
        format!("{lcm}")
    }
}
