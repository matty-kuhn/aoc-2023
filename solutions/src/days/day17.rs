use std::collections::HashMap;

use super::{get_lines, Day};

pub struct Day17 {
    input: String,
}

impl Day17 {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    fn parse_input(&self) -> HashMap<(usize, usize), Pos> {
        let lines = get_lines(&self.input);
        let mut points = HashMap::new();
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                points.insert(
                    (x, y),
                    Pos {
                        x,
                        y,
                        heat_loss: lines[y].chars().nth(x).unwrap().to_digit(10).unwrap() as usize,
                    },
                );
            }
        }
        points
    }

    fn get_successors(
        point: &Pos,
        map: &HashMap<(usize, usize), Pos>,
        direction: &mut Direction,
    ) -> Vec<(Pos, usize)> {
        vec![]
    }

    fn heuristic(point: &Pos, map: &HashMap<(usize, usize), Pos>) -> usize {
        point.heat_loss
    }

    fn do_astar(map: HashMap<(usize, usize), Pos>, dims: (usize, usize)) -> usize {
        let mut curr_direction = Direction {
            delta: (0, 0),
            count: 0,
        };
        let path = pathfinding::directed::astar::astar(
            map.get(&(0, 0)).unwrap(),
            |p| Self::get_successors(p, &map, &mut curr_direction),
            |p| Self::heuristic(p, &map),
            |p| *p == *map.get(&dims).unwrap(),
        );
        0
    }
}

impl Day for Day17 {
    fn part1(&self) -> String {
        let points = self.parse_input();
        dbg!(points);
        format!("todo")
    }

    fn part2(&self) -> String {
        format!("todo")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
    heat_loss: usize,
}

#[derive(Debug)]
struct Direction {
    delta: (i8, i8),
    count: u8,
}
