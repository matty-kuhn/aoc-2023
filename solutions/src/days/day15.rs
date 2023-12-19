use std::collections::HashMap;

use super::Day;

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> String {
        self.input.trim().to_string()
    }

    fn score_group(group: &str) -> usize {
        let mut curr_val = 0;
        for c in group.chars() {
            curr_val += (c as u8) as usize;
            curr_val *= 17;
            curr_val %= 256;
        }
        curr_val
    }

    fn score_boxes(box_map: &HashMap<usize, HashMap<&str, (usize, usize)>>) -> usize {
        // lens score = (box # + 1) * index * len
        let mut total_score = 0;
        for (box_num, lenses) in box_map {
            for (group, (idx, len)) in lenses {
                let score = (box_num + 1) * (idx + 1) * len;
                // println!(
                //     "{} {} {} {} {}",
                //     group,
                //     (box_num + 1),
                //     (idx + 1),
                //     len,
                //     score
                // );
                total_score += score;
            }
        }
        total_score
    }
}

impl Day for Day15 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let sum = self
            .parse_input()
            .split(",")
            .fold(0, |acc, x| acc + Self::score_group(x));
        println!("part 1 time: {:?}", start_time.elapsed());
        format!("{sum}")
    }

    fn part2(&self) -> String {
        // map of box #: HashMap<group,(idx, len)>
        let start_time = std::time::Instant::now();
        let mut box_map = HashMap::new();
        let binding = self.parse_input();
        for group in binding.split(',') {
            if group.contains("-") {
                // take that lens out
                let group = group.trim_end_matches("-");
                let entry: &mut HashMap<&str, (usize, usize)> = box_map
                    .entry(Self::score_group(group))
                    .or_insert(HashMap::new());
                if let Some(inner_entry) = entry.remove(group) {
                    // there was an entry, so decrease all indicies that were greater than this one
                    for (_, (idx, _)) in entry.iter_mut() {
                        if *idx > inner_entry.0 {
                            *idx -= 1;
                        }
                    }
                }

            } else {
                // add to box map
                let mut split = group.split('=');
                let group = split.next().unwrap();
                let len = split.next().unwrap().parse::<usize>().unwrap();
                let entry: &mut HashMap<&str, (usize, usize)> = box_map
                    .entry(Self::score_group(group))
                    .or_insert(HashMap::new());
                let entry_len = entry.len();
                let inner_entry = entry.entry(group).or_insert((entry_len, len));
                inner_entry.1 = len;
            }
        }
        let sum = Self::score_boxes(&box_map);
        println!("part 2 time: {:?}", start_time.elapsed());
        format!("{sum}")
    }
}
