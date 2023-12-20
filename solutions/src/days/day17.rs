use std::collections::HashMap;

use num::integer::sqrt;

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
        point: &Direction,
        map: &HashMap<(usize, usize), Pos>,
        min_path: u8,
        max_path: u8,
        goal: (usize, usize),
    ) -> Vec<(Direction, usize)> {
        // return all valid successors, removing ones that go too far in the direction
        let mut successors = vec![];
        if point.pos.x > 0 && point.delta.0 != 1 {
            // only valid if we can keep going in this direction, we also don't want to go directly backwards
            if (point.delta.0 == -1 && point.count <= max_path) || point.delta.0 == 0 {
                let curr_point = map.get(&(point.pos.x - 1, point.pos.y)).unwrap();
                let item = (
                    Direction {
                        pos: curr_point.clone(),
                        delta: (-1, 0),
                        count: if point.delta.0 == -1 {
                            point.count + 1
                        } else {
                            1
                        },
                    },
                    curr_point.heat_loss,
                );
                if item.0.pos.x == goal.0
                    && item.0.pos.y == goal.1
                    && item.0.count >= min_path
                    && item.0.count <= max_path
                {
                    // we've reached the goal, return the path
                    return vec![item];
                }
                successors.push(item);
            }
        }
        if point.pos.y > 0 && point.delta.1 != 1 {
            // only valid if we can keep going in this direction, we also don't want to go directly backwards
            if (point.delta.1 == -1 && point.count <= max_path) || point.delta.1 == 0 {
                let curr_point = map.get(&(point.pos.x, point.pos.y - 1)).unwrap();
                let item = (
                    Direction {
                        pos: curr_point.clone(),
                        delta: (0, -1),
                        count: if point.delta.1 == -1 {
                            point.count + 1
                        } else {
                            1
                        },
                    },
                    curr_point.heat_loss,
                );
                if item.0.pos.x == goal.0
                    && item.0.pos.y == goal.1
                    && item.0.count >= min_path
                    && item.0.count <= max_path
                {
                    // we've reached the goal, return the path
                    return vec![item];
                }
                successors.push(item);
            }
        }
        if point.pos.x < goal.0 && point.delta.0 != -1 {
            // only valid if we can keep going in this direction, we also don't want to go directly backwards
            if (point.delta.0 == 1 && point.count <= max_path) || point.delta.0 == 0 {
                let curr_point = map.get(&(point.pos.x + 1, point.pos.y)).unwrap();
                let item = (
                    Direction {
                        pos: curr_point.clone(),
                        delta: (1, 0),
                        count: if point.delta.0 == 1 {
                            point.count + 1
                        } else {
                            1
                        },
                    },
                    curr_point.heat_loss,
                );
                if item.0.pos.x == goal.0
                    && item.0.pos.y == goal.1
                    && item.0.count >= min_path
                    && item.0.count <= max_path
                {
                    // we've reached the goal, return the path
                    return vec![item];
                }
                successors.push(item);
            }
        }
        if point.pos.y < goal.1 && point.delta.1 != -1 {
            // only valid if we can keep going in this direction, we also don't want to go directly backwards
            if (point.delta.1 == 1 && point.count <= max_path) || point.delta.1 == 0 {
                let curr_point = map.get(&(point.pos.x, point.pos.y + 1)).unwrap();
                let item = (
                    Direction {
                        pos: curr_point.clone(),
                        delta: (0, 1),
                        count: if point.delta.1 == 1 {
                            point.count + 1
                        } else {
                            1
                        },
                    },
                    curr_point.heat_loss,
                );
                if item.0.pos.x == goal.0
                    && item.0.pos.y == goal.1
                    && item.0.count >= min_path
                    && item.0.count <= max_path
                {
                    // we've reached the goal, return the path
                    return vec![item];
                }
                successors.push(item);
            }
        }
        successors
            .iter()
            .filter(|s| s.0.count <= max_path)
            .filter(|s| {
                // if it is the same, need the right count, if it is different, don't need it
                (s.0.delta == point.delta && s.0.count >= min_path) || s.0.delta != point.delta
            })
            .filter(|s| !(s.0.pos.x == goal.0 && s.0.pos.y == goal.1 && s.0.count < min_path))
            .map(|x| x.to_owned())
            .collect()
    }

    fn get_next_successor_in_path(
        point: &Direction,
        map: &HashMap<(usize, usize), Pos>,
        min_path: u8,
        max_path: u8,
        goal: (usize, usize),
    ) -> Vec<(Direction, usize)> {
        let Some(pos) = map
            .get(&(
                (point.pos.x as i64 + point.delta.0 as i64) as usize,
                (point.pos.y as i64 + point.delta.1 as i64) as usize,
            ))
            .cloned()
        else {
            return vec![];
        };
        let heat_loss = pos.heat_loss;
        let next = Direction {
            pos,
            delta: point.delta,
            count: point.count + 1,
        };
        if next.pos.x == goal.0
            && next.pos.y == goal.1
            && (next.count < min_path || next.count > max_path)
        {
            // we dont have enough on the path to finish
            return vec![];
        }
        vec![(next, heat_loss)]
    }

    fn do_astar(
        map: HashMap<(usize, usize), Pos>,
        goal: (usize, usize),
        min_path: u8,
        max_path: u8,
    ) -> usize {
        let init_dir = Direction {
            pos: map.get(&(0, 0)).cloned().unwrap(),
            delta: (0, 0),
            count: 0,
        };
        let path = pathfinding::directed::astar::astar(
            &init_dir,
            |p| {
                if p.count < min_path && p.delta != (0, 0) {
                    // need to get next one in path
                    Self::get_next_successor_in_path(p, &map, min_path, max_path, goal)
                } else {
                    Self::get_successors(p, &map, min_path, max_path, goal)
                }
            },
            |p| goal.0.abs_diff(p.pos.x) + goal.1.abs_diff(p.pos.y),
            |p| p.pos == *map.get(&goal).unwrap(),
        );

        path.unwrap().1
    }
}

impl Day for Day17 {
    fn part1(&self) -> String {
        let start_time = std::time::Instant::now();
        let points = self.parse_input();
        let side_len = sqrt(points.len()) - 1;
        let heat = Self::do_astar(points, (side_len, side_len), 1, 3);
        println!("Part 1 took {:?}", start_time.elapsed());
        format!("{heat}")
    }

    fn part2(&self) -> String {
        let start_time = std::time::Instant::now();
        let points = self.parse_input();
        let side_len = sqrt(points.len()) - 1;
        let heat = Self::do_astar(points, (side_len, side_len), 4, 10);
        println!("Part 2 took {:?}", start_time.elapsed());
        format!("{heat}")
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    x: usize,
    y: usize,
    heat_loss: usize,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
struct Direction {
    pos: Pos,
    delta: (i8, i8),
    count: u8,
}
