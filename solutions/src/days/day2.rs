use crate::days::get_lines;

use super::Day;

pub struct Day2 {
    input: String,
}

impl Day2 {
    pub fn new(input: String) -> Day2 {
        Day2 { input }
    }
}

impl Day for Day2 {
    fn part1(&self) -> String {
        let lines = get_lines(&self.input);
        let limits = GameLimits::new(12, 13, 14);
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let games = lines
                    .iter()
                    .map(|line| Game::parse_from_line(line, limits))
                    .collect::<Vec<Game>>();
                let mut sum = 0;
                for game in games {
                    println!(
                        "Game {} red: {}, blue: {}, green: {} {} valid",
                        game.id,
                        game.max_num_red,
                        game.max_num_blue,
                        game.max_num_green,
                        if game.is_valid() { "is" } else { "is not" }
                    );
                    if game.is_valid() {
                        sum += game.id;
                    }
                }
            }
            else {
                let valid_games = lines
                    .iter()
                    .map(|line| Game::parse_from_line(line, limits))
                    .filter(|game| game.is_valid())
                    .collect::<Vec<Game>>();
                let sum = valid_games.iter().fold(0, |acc, game| acc + game.id);
            }
        }
        format!("{sum}")
    }

    fn part2(&self) -> String {
        let lines = get_lines(&self.input);
        let limits = GameLimits::new(12, 13, 14);
        cfg_if::cfg_if! {
            if #[cfg(debug_assertions)] {
                let games = lines
                    .iter()
                    .map(|line| Game::parse_from_line(line, limits))
                    .collect::<Vec<Game>>();
                let mut sum = 0;
                for game in games {
                    println!(
                        "Game {} red: {}, blue: {}, green: {} power: {}",
                        game.id,
                        game.max_num_red,
                        game.max_num_blue,
                        game.max_num_green,
                        game.get_power()
                    );
                    sum += game.get_power();
                }
            }
            else {
                let sum = lines
                    .iter()
                    .map(|line| Game::parse_from_line(line, limits))
                    .fold(0, |acc, game| acc + game.get_power());
            }
        }
        format!("{sum}")
    }
}

#[derive(Clone, Copy)]
struct GameLimits {
    max_red: u32,
    max_blue: u32,
    max_green: u32,
}

impl GameLimits {
    fn new(max_red: u32, max_green: u32, max_blue: u32) -> GameLimits {
        GameLimits {
            max_red,
            max_blue,
            max_green,
        }
    }
}

struct Game {
    id: u32,
    max_num_red: u32,
    max_num_blue: u32,
    max_num_green: u32,
    limitations: GameLimits,
}

impl Game {
    fn parse_from_line(line: &str, limitations: GameLimits) -> Game {
        // sample line:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let mut parts = line.split(": ");
        let id = parts
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let parts = parts.next().unwrap().split("; ");
        let mut max_num_red = 0;
        let mut max_num_blue = 0;
        let mut max_num_green = 0;
        // for each round in the game
        for part in parts {
            let inner_parts = part.split(", ");
            // for each color in the round
            for curr in inner_parts {
                let mut curr = curr.split(" ");
                let num = curr.next().unwrap().parse::<u32>().unwrap();
                let color = curr.next().unwrap();
                match color {
                    "red" => {
                        if num > max_num_red {
                            max_num_red = num;
                        }
                    }
                    "blue" => {
                        if num > max_num_blue {
                            max_num_blue = num;
                        }
                    }
                    "green" => {
                        if num > max_num_green {
                            max_num_green = num;
                        }
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
        }
        Game {
            id,
            limitations,
            max_num_red,
            max_num_blue,
            max_num_green,
        }
    }

    fn is_valid(&self) -> bool {
        self.max_num_red <= self.limitations.max_red
            && self.max_num_blue <= self.limitations.max_blue
            && self.max_num_green <= self.limitations.max_green
    }

    fn get_power(&self) -> u32 {
        self.max_num_red * self.max_num_blue * self.max_num_green
    }
}
