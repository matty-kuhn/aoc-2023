use std::collections::HashMap;

use num::integer::sqrt;

use super::{get_lines, Day};

type Coord = (i32, i32);
type Map = HashMap<Coord, Point>;

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new(input: String) -> Day10 {
        Day10 { input }
    }

    fn parse_input(&self) -> (Coord, Map) {
        let mut map = Map::new();
        let mut start = (0, 0);
        let lines = get_lines(&self.input);
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let coord = (x as i32, y as i32);
                let point = c.into();
                if point == Point::Start {
                    start = coord;
                }
                map.insert(coord, point);
            }
        }
        (start, map)
    }

    ///returns a list of points that are interior to the path
    fn find_interior_points(map: &Map, curve: &HashMap<Coord, i32>) -> Vec<Coord> {
        let mut inner_points = Vec::new();
        // only works for square maps
        let side_len = sqrt(map.len() as i32 + 1);
        // the direction of loop is currently up
        let mut curr_up = None;
        for y in 0..side_len {
            for x in 0..side_len {
                let coord = (x, y);
                if !map.contains_key(&coord) {
                    continue;
                }
                if curve.contains_key(&coord) {
                    let Some(point) = map.get(&coord) else {
                        panic!()
                    };
                    match point {
                        Point::NorthEast | Point::NorthWest | Point::Ground | Point::EastWest => {
                            // do nothing
                        }
                        Point::Start => {
                            // check if you can go south from Start
                            if let Some(point) = map.get(&(x, y + 1)) {
                                if Point::Start.check_next_point_valid(point, Direction::South) {
                                    // if first upwards, then curr_up is true, else just switch directions
                                    if curr_up.is_none() {
                                        curr_up = Some(true)
                                    } else {
                                        curr_up = Some(!curr_up.unwrap());
                                    }
                                }
                            }
                        }
                        _ => {
                            // if first upwards, then curr_up is true, else just switch directions
                            if curr_up.is_none() {
                                curr_up = Some(true)
                            } else {
                                curr_up = Some(!curr_up.unwrap());
                            }
                        }
                    }
                } else {
                    // we havne't hit the loop yet
                    let Some(curr_up) = curr_up else {
                        continue;
                    };
                    if curr_up {
                        inner_points.push(coord);
                    }
                }
            }
        }
        inner_points
    }

    /// returns the direction from the start point
    fn find_start_direction(start: &Coord, map: &Map) -> (Coord, Point) {
        let checks = vec![
            (start.0, start.1 - 1),
            (start.0, start.1 + 1),
            (start.0 - 1, start.1),
            (start.0 + 1, start.1),
        ];
        for check in checks {
            if let Some(point) = map.get(&check) {
                if Point::Start.check_next_point_valid(point, Direction::from((*start, check))) {
                    return (check, *point);
                }
            }
        }
        panic!("Couldn't find start direction");
    }

    /// returns the total number of steps taken, and a map of the number of steps taken to each point
    fn traverse_path(start: &Coord, map: &Map) -> (i32, HashMap<Coord, i32>) {
        let mut steps = 0;
        let (mut current_coord, mut current_point) = Self::find_start_direction(start, map);
        let mut visited = HashMap::new();
        visited.insert(*start, steps);
        loop {
            let checks = vec![
                (current_coord.0, current_coord.1 - 1),
                (current_coord.0, current_coord.1 + 1),
                (current_coord.0 - 1, current_coord.1),
                (current_coord.0 + 1, current_coord.1),
            ];
            let mut found = false;
            for check in checks {
                if let Some(point) = map.get(&check) {
                    if (!visited.contains_key(&check) || point.is_start())
                        && current_point
                            .check_next_point_valid(point, Direction::from((current_coord, check)))
                    {
                        steps += 1;
                        visited.insert(current_coord, steps);
                        current_coord = check;
                        current_point = *point;
                        found = true;
                        break;
                    }
                    if point.is_start() {
                        found = true;
                    }
                }
            }
            if !found {
                break (steps, visited);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<(Coord, Coord)> for Direction {
    fn from(coords: (Coord, Coord)) -> Self {
        let (start, end) = coords;
        if start.0 == end.0 {
            if start.1 > end.1 {
                Self::North
            } else {
                Self::South
            }
        } else if start.1 == end.1 {
            if start.0 > end.0 {
                Self::West
            } else {
                Self::East
            }
        } else {
            panic!("Can't convert {:?} to direction", coords);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Point {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Point {
    fn is_start(&self) -> bool {
        *self == Point::Start
    }

    /// relative direction is what direction to move from self to other,
    /// so if it is North, then it looks like this:
    /// Other
    /// Self
    fn check_next_point_valid(&self, other: &Point, relative_direction: Direction) -> bool {
        if other.is_start() {
            return true;
        }
        match self {
            Self::NorthSouth => match relative_direction {
                Direction::North => {
                    *other == Self::NorthSouth
                        || *other == Self::SouthWest
                        || *other == Self::SouthEast
                }
                Direction::South => {
                    *other == Self::NorthSouth
                        || *other == Self::NorthWest
                        || *other == Self::NorthEast
                }
                _ => false, // can't go east or west from north south
            },
            Self::EastWest => match relative_direction {
                Direction::East => {
                    *other == Self::EastWest
                        || *other == Self::NorthWest
                        || *other == Self::SouthWest
                }
                Direction::West => {
                    *other == Self::EastWest
                        || *other == Self::NorthEast
                        || *other == Self::SouthEast
                }
                _ => false, // can't go north or south from east west
            },
            Self::NorthEast => match relative_direction {
                Direction::North => {
                    *other == Self::NorthSouth
                        || *other == Self::SouthWest
                        || *other == Self::SouthEast
                }
                Direction::East => {
                    *other == Self::NorthWest
                        || *other == Self::EastWest
                        || *other == Self::SouthWest
                }
                _ => false, // can't go south or west from north east
            },
            Self::NorthWest => match relative_direction {
                Direction::North => {
                    *other == Self::NorthSouth
                        || *other == Self::SouthWest
                        || *other == Self::SouthEast
                }
                Direction::West => {
                    *other == Self::NorthEast
                        || *other == Self::EastWest
                        || *other == Self::SouthEast
                }
                _ => false, // can't go south or east from north west
            },
            Self::SouthWest => match relative_direction {
                Direction::South => {
                    *other == Self::NorthSouth
                        || *other == Self::NorthWest
                        || *other == Self::NorthEast
                }
                Direction::West => {
                    *other == Self::NorthEast
                        || *other == Self::EastWest
                        || *other == Self::SouthEast
                }
                _ => false, // can't go north or east from south west
            },
            Self::SouthEast => match relative_direction {
                Direction::South => {
                    *other == Self::NorthSouth
                        || *other == Self::NorthWest
                        || *other == Self::NorthEast
                }
                Direction::East => {
                    *other == Self::NorthWest
                        || *other == Self::EastWest
                        || *other == Self::SouthWest
                }
                _ => false, // can't go north or west from south east
            },
            Self::Ground => false,
            Self::Start => match relative_direction {
                Direction::North => {
                    *other == Self::NorthSouth
                        || *other == Self::SouthWest
                        || *other == Self::SouthEast
                }
                Direction::South => {
                    *other == Self::NorthSouth
                        || *other == Self::NorthWest
                        || *other == Self::NorthEast
                }
                Direction::East => {
                    *other == Self::NorthWest
                        || *other == Self::EastWest
                        || *other == Self::SouthWest
                }
                Direction::West => {
                    *other == Self::NorthEast
                        || *other == Self::EastWest
                        || *other == Self::SouthEast
                }
            },
        }
    }
}

impl From<char> for Point {
    fn from(c: char) -> Self {
        match c {
            '|' => Point::NorthSouth,
            '-' => Point::EastWest,
            'L' => Point::NorthEast,
            'J' => Point::NorthWest,
            '7' => Point::SouthWest,
            'F' => Point::SouthEast,
            '.' => Point::Ground,
            'S' => Point::Start,
            _ => panic!("Unknown point type"),
        }
    }
}

impl Day for Day10 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let map = self.parse_input();
        let steps = Self::traverse_path(&map.0, &map.1).0;
        println!("Part 1 took {:?}", start_time.elapsed());
        format!("{}", (steps + 1) / 2)
    }

    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        let (start_point, map) = self.parse_input();
        let curve = Self::traverse_path(&start_point, &map).1;
        let num_interior = Self::find_interior_points(&map, &curve).len();
        println!("Part 2 took {:?}", start_time.elapsed());
        format!("{num_interior}")
    }
}
