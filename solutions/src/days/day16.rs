use std::collections::{HashSet, VecDeque};

use super::{get_lines, Day};

pub struct Day16 {
    input: String,
}

impl Day16 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> Vec<Vec<char>> {
        get_lines(&self.input)
            .iter()
            .map(|l| l.chars().collect())
            .collect()
    }

    fn visualize_energized(&self, energized_tiles: &HashSet<(usize, usize)>) {
        let map = self.parse_input();
        for (y, row) in map.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if energized_tiles.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_next_square(
        x: usize,
        y: usize,
        dir: Direction,
        max_x: usize,
        max_y: usize,
    ) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Direction::Down => {
                if y == max_y {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Direction::Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Direction::Right => {
                if x == max_x {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
        }
    }

    fn count_energized_tiles(
        &self,
        start_dir: Direction,
        start_square: (usize, usize),
        map: &Vec<Vec<char>>,
    ) -> usize {
        // start count at 1 for the starting tile
        let mut curr_dir = start_dir;
        let (mut x, mut y) = start_square;
        let mut energized_tiles = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((curr_dir.clone(), (x, y)));
        energized_tiles.insert((x, y));
        while let Some((dir, square)) = queue.pop_front() {
            if visited.contains(&(dir, square)) {
                continue;
            }
            (x, y) = square;
            curr_dir = dir;
            visited.insert((curr_dir, (x, y)));
            match map[y][x] {
                '.' => {
                    // keep going dummy
                    if let Some(next_square) =
                        Self::get_next_square(x, y, curr_dir, map[0].len() - 1, map.len() - 1)
                    {
                        queue.push_back((curr_dir, next_square));
                    }
                }
                '-' => {
                    if matches!(curr_dir, Direction::Up) || matches!(curr_dir, Direction::Down) {
                        // push both onto the queue
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Left,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Left, next_square));
                        }
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Right,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Right, next_square));
                        }
                    } else {
                        if let Some(next_square) =
                            Self::get_next_square(x, y, curr_dir, map[0].len() - 1, map.len() - 1)
                        {
                            queue.push_back((curr_dir, next_square));
                        }
                    }
                }
                '|' => {
                    if matches!(curr_dir, Direction::Left) || matches!(curr_dir, Direction::Right) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Up,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Up, next_square));
                        }
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Down,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Down, next_square));
                        }
                        // push both onto the queue
                    } else {
                        if let Some(next_square) =
                            Self::get_next_square(x, y, curr_dir, map[0].len() - 1, map.len() - 1)
                        {
                            queue.push_back((curr_dir, next_square));
                        }
                    }
                }
                '/' => {
                    if matches!(curr_dir, Direction::Up) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Right,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Right, next_square));
                        }
                    } else if matches!(curr_dir, Direction::Down) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Left,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Left, next_square));
                        }
                    } else if matches!(curr_dir, Direction::Left) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Down,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Down, next_square));
                        }
                    } else {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Up,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Up, next_square));
                        }
                    }
                }
                '\\' => {
                    if matches!(curr_dir, Direction::Up) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Left,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Left, next_square));
                        }
                    } else if matches!(curr_dir, Direction::Down) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Right,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Right, next_square));
                        }
                    } else if matches!(curr_dir, Direction::Left) {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Up,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Up, next_square));
                        }
                    } else {
                        if let Some(next_square) = Self::get_next_square(
                            x,
                            y,
                            Direction::Down,
                            map[0].len() - 1,
                            map.len() - 1,
                        ) {
                            queue.push_back((Direction::Down, next_square));
                        }
                    }
                }
                _ => panic!("Invalid character in map"),
            }
            if !energized_tiles.contains(&(x, y)) {
                energized_tiles.insert((x, y));
            }
        }
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
            self.visualize_energized(&energized_tiles);
            }
        }
        energized_tiles.len()
    }
}

impl Day for Day16 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let energized_tiles =
            self.count_energized_tiles(Direction::Right, (0, 0), &self.parse_input());
        println!("part1 time: {:?}", start_time.elapsed());
        format!("{energized_tiles}")
    }

    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        // try every position around the perimiter, starting in, and find the max
        let map = self.parse_input();
        let max_x = map[0].len() - 1;
        let max_y = map.len() - 1;
        // score and starting position
        let mut max_energized = (0, (0, 0));
        for x in 0..=max_x {
            for y in 0..=max_y {
                if x == 0 {
                    let energized = self.count_energized_tiles(Direction::Right, (x, y), &map);
                    if energized > max_energized.0 {
                        max_energized = (energized, (x, y));
                    }
                }
                if y == 0 {
                    let energized = self.count_energized_tiles(Direction::Down, (x, y), &map);
                    if energized > max_energized.0 {
                        max_energized = (energized, (x, y));
                    }
                }
                if x == max_x {
                    let energized = self.count_energized_tiles(Direction::Left, (x, y), &map);
                    if energized > max_energized.0 {
                        max_energized = (energized, (x, y));
                    }
                }
                if y == max_y {
                    let energized = self.count_energized_tiles(Direction::Up, (x, y), &map);
                    if energized > max_energized.0 {
                        max_energized = (energized, (x, y));
                    }
                }
            }
        }
        println!("part2 time: {:?}", start_time.elapsed());
        format!("{:?}", max_energized.0)
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next_square(&self, square: (usize, usize)) -> (usize, usize) {
        let (x, y) = square;
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        }
    }
}
