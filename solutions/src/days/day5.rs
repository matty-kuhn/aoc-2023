use std::str::FromStr;

use crate::days::get_lines;

use super::Day;

pub struct Day5 {
    input: String,
}

impl Day5 {
    pub fn new(input: String) -> Day5 {
        Day5 { input }
    }

    fn parse_input(&self) -> (Vec<isize>, Vec<ResourceMap>) {
        let lines = get_lines(&self.input);
        let mut seeds = Vec::new();
        let mut resource_maps = Vec::new();
        let mut current_resource_map = Vec::new();
        // get seeds from first line
        let seed_line = lines[0];
        let seed_line = seed_line.replace("seeds: ", "");
        let seed_line = seed_line.split_whitespace().collect::<Vec<&str>>();
        for seed in seed_line {
            seeds.push(seed.parse::<isize>().unwrap());
        }

        // this one will be first
        let mut curr_map_type = MapTypes::from_str(lines[2]).unwrap();

        for line in &lines[1..] {
            // skip empty lines
            if line.trim().is_empty() {
                continue;
            }
            // if the line is a map type, then we need to save the current resource map and start a new one
            if let Ok(map_type) = MapTypes::from_str(line) {
                if !current_resource_map.is_empty() {
                    resource_maps.push(ResourceMap::new(curr_map_type, current_resource_map));
                    current_resource_map = Vec::new();
                    curr_map_type = map_type;
                }
            } else {
                // otherwise, we need to add the line to the current resource map
                current_resource_map.push(RangeMap::from_str(line).unwrap());
            }
        }
        // add the last resource map
        resource_maps.push(ResourceMap::new(curr_map_type, current_resource_map));

        (seeds, resource_maps)
    }

    fn transform_seeds_as_pairs(seeds: &Vec<isize>) -> Vec<(isize, isize)> {
        let mut output = Vec::new();
        // for each seed, add that seed and the next
        for i in 0..seeds.len() - 1 {
            if i % 2 == 1 {
                continue;
            }
            output.push((seeds[i], seeds[i + 1]));
        }
        output
    }
}

impl Day for Day5 {
    fn part1(&self) -> String {
        let (seeds, resource_maps) = self.parse_input();
        let mut output = seeds;
        for resource_map in resource_maps {
            output = resource_map.apply(&output);
        }
        format!("{:?}", output.iter().min().unwrap())
    }

    fn part2(&self) -> String {
        let (seeds, resource_maps) = self.parse_input();
        let seed_ranges = Day5::transform_seeds_as_pairs(&seeds);
        let mut min = isize::MAX;
        for (start, len) in seed_ranges {
            let mut input = (start..start + len).collect::<Vec<isize>>();
            for resource_map in &resource_maps {
                input = resource_map.apply(&input);
            }
            min = min.min(*input.iter().min().unwrap());
        }
        format!("{min}")
    }
}

#[derive(Debug, Clone)]
struct RangeMap {
    dest_start: isize,
    source_start: isize,
    range_len: isize,
}

impl FromStr for RangeMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // line format:
        // <dest_start> <source_start> <range_len>
        let parts = s.split(' ').collect::<Vec<&str>>();
        let dest_start = parts[0].parse::<isize>().unwrap();
        let source_start = parts[1].parse::<isize>().unwrap();
        let range_len = parts[2].parse::<isize>().unwrap();
        Ok(RangeMap::new(dest_start, source_start, range_len))
    }
}

impl RangeMap {
    fn new(dest_start: isize, source_start: isize, range_len: isize) -> RangeMap {
        RangeMap {
            dest_start,
            source_start,
            range_len,
        }
    }

    /// returns a tuple of changed, unchanged
    fn apply(&self, input: &[isize]) -> (Vec<isize>, Vec<isize>) {
        let mut changed = Vec::new();
        let mut unchanged = Vec::new();
        for num in input.iter() {
            let offset = *num - self.source_start;
            // if offset >= 0 and offset < range_len
            // num = dest_start + offset
            // else num = num
            if offset >= 0 && offset < self.range_len {
                changed.push(self.dest_start + offset);
            } else {
                unchanged.push(*num);
            };
        }
        (changed, unchanged)
    }
}

#[derive(Debug, Clone)]
struct ResourceToResource {
    range_maps: Vec<RangeMap>,
}

impl ResourceToResource {
    fn new(range_maps: Vec<RangeMap>) -> ResourceToResource {
        ResourceToResource { range_maps }
    }

    fn apply(&self, input: &[isize]) -> Vec<isize> {
        let mut input = input.to_vec();
        let mut output = Vec::new();
        for range_map in self.range_maps.iter() {
            // apply the range map to the input
            let (changed, unchanged) = range_map.apply(&input);
            // save the changed values, and keep the unchanged values in input
            input = unchanged;
            output.extend(changed);
        }
        output.extend(input);
        output
    }
}

enum MapTypes {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl FromStr for MapTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed-to-soil map:" => Ok(MapTypes::SeedToSoil),
            "soil-to-fertilizer map:" => Ok(MapTypes::SoilToFertilizer),
            "fertilizer-to-water map:" => Ok(MapTypes::FertilizerToWater),
            "water-to-light map:" => Ok(MapTypes::WaterToLight),
            "light-to-temperature map:" => Ok(MapTypes::LightToTemperature),
            "temperature-to-humidity map:" => Ok(MapTypes::TemperatureToHumidity),
            "humidity-to-location map:" => Ok(MapTypes::HumidityToLocation),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum ResourceMap {
    SeedToSoil(ResourceToResource),
    SoilToFertilizer(ResourceToResource),
    FertilizerToWater(ResourceToResource),
    WaterToLight(ResourceToResource),
    LightToTemperature(ResourceToResource),
    TemperatureToHumidity(ResourceToResource),
    HumidityToLocation(ResourceToResource),
}

impl ResourceMap {
    fn new(map_type: MapTypes, maps: Vec<RangeMap>) -> Self {
        match map_type {
            MapTypes::SeedToSoil => ResourceMap::SeedToSoil(ResourceToResource::new(maps)),
            MapTypes::SoilToFertilizer => {
                ResourceMap::SoilToFertilizer(ResourceToResource::new(maps))
            }
            MapTypes::FertilizerToWater => {
                ResourceMap::FertilizerToWater(ResourceToResource::new(maps))
            }
            MapTypes::WaterToLight => ResourceMap::WaterToLight(ResourceToResource::new(maps)),
            MapTypes::LightToTemperature => {
                ResourceMap::LightToTemperature(ResourceToResource::new(maps))
            }
            MapTypes::TemperatureToHumidity => {
                ResourceMap::TemperatureToHumidity(ResourceToResource::new(maps))
            }
            MapTypes::HumidityToLocation => {
                ResourceMap::HumidityToLocation(ResourceToResource::new(maps))
            }
        }
    }

    fn apply(&self, input: &[isize]) -> Vec<isize> {
        match self {
            ResourceMap::SeedToSoil(resource_to_resource) => resource_to_resource.apply(input),
            ResourceMap::SoilToFertilizer(resource_to_resource) => {
                resource_to_resource.apply(input)
            }
            ResourceMap::FertilizerToWater(resource_to_resource) => {
                resource_to_resource.apply(input)
            }
            ResourceMap::WaterToLight(resource_to_resource) => resource_to_resource.apply(input),
            ResourceMap::LightToTemperature(resource_to_resource) => {
                resource_to_resource.apply(input)
            }
            ResourceMap::TemperatureToHumidity(resource_to_resource) => {
                resource_to_resource.apply(input)
            }
            ResourceMap::HumidityToLocation(resource_to_resource) => {
                resource_to_resource.apply(input)
            }
        }
    }
}
