use std::collections::HashSet;

use super::{get_lines, Day};

type Galaxy = (i64, i64);

pub struct Day11 {
    input: String,
}

impl Day11 {
    pub fn new(input: String) -> Day11 {
        Day11 { input }
    }

    fn get_galaxies(&self) -> (Vec<Galaxy>, (Vec<i64>, Vec<i64>)) {
        let mut galaxies = Vec::new();
        let lines = get_lines(&self.input);
        // use this to count galaxies in each line, 0 means that line gets expanded
        let mut galaxy_row_counts = vec![0; lines[0].len()];
        // same for columns
        let mut galaxy_column_counts = vec![0; lines.len()];
        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => {
                        galaxy_row_counts[y] += 1;
                        galaxy_column_counts[x] += 1;
                        galaxies.push((x as i64, y as i64))
                    }
                    _ => (),
                }
            }
        }

        (galaxies, (galaxy_row_counts, galaxy_column_counts))
    }

    fn adjust_rows_and_columns(
        galaxies: Vec<Galaxy>,
        galaxy_row_counts: Vec<i64>,
        galaxy_column_counts: Vec<i64>,
        expansion_amount: i64,
    ) -> Vec<Galaxy> {
        let mut new_galaxies = Vec::new();
        for galaxy in galaxies {
            let (galaxy_x, galaxy_y) = galaxy;
            let mut add_x = 0;
            let mut add_y = 0;
            // look at every row up to the current
            for y in galaxy_row_counts.iter().take(galaxy_y as usize) {
                if *y < 1 {
                    add_y += expansion_amount;
                }
            }
            // look at every column up to the current
            for x in galaxy_column_counts.iter().take(galaxy_x as usize) {
                if *x < 1 {
                    add_x += expansion_amount;
                }
            }
            new_galaxies.push((galaxy_x + add_x, galaxy_y + add_y));
        }
        new_galaxies
    }

    fn distance_between_all_galaxies(galaxies: Vec<Galaxy>) -> i64 {
        let mut total_distance = 0;
        let mut visited_pairs = HashSet::new();
        for galaxy in &galaxies {
            for other_galaxy in &galaxies {
                if visited_pairs.contains(&(other_galaxy, galaxy)) {
                    continue;
                }
                total_distance +=
                    (galaxy.0 - other_galaxy.0).abs() + (galaxy.1 - other_galaxy.1).abs();
                visited_pairs.insert((galaxy, other_galaxy));
            }
        }
        total_distance
    }
}

impl Day for Day11 {
    fn part1(&self) -> String {
        let expansion_amount = 1;
        let (galaxies, (galaxy_row_counts, galaxy_column_counts)) = self.get_galaxies();
        let galaxies = Self::adjust_rows_and_columns(
            galaxies,
            galaxy_row_counts,
            galaxy_column_counts,
            expansion_amount,
        );
        format!("{}", Self::distance_between_all_galaxies(galaxies))
    }

    fn part2(&self) -> String {
        let expansion_amount = 999_999;
        let (galaxies, (galaxy_row_counts, galaxy_column_counts)) = self.get_galaxies();
        let galaxies = Self::adjust_rows_and_columns(
            galaxies,
            galaxy_row_counts,
            galaxy_column_counts,
            expansion_amount,
        );
        format!("{}", Self::distance_between_all_galaxies(galaxies))
    }
}
