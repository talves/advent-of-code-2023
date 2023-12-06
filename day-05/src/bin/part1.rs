fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct CategoryItem {
    source_start: i64,
    destination_start: i64,
    range_length: i64,
}

#[derive(Debug)]
struct Mapper {
    items: Vec<CategoryItem>,
}

impl Mapper {
    fn new() -> Mapper {
        Mapper { items: Vec::new() }
    }
    fn add(&mut self, destination_start: i64, source_start: i64, range_length: i64) {
        self.items.push(CategoryItem {
            destination_start,
            source_start,
            range_length,
        });
    }
    fn get_destination(&self, value: i64) -> i64 {
        // Return the value if it doesn't fall in the ranges of the items
        // Check for the source value in each item range and return the mapped destination
        for item in &self.items {
            // Check for the source value in each item range and return the mapped destination
            let item_source_end = item.source_start + item.range_length - 1;
            if (value >= item.source_start) && (value <= item_source_end) {
                let placement_idx = value - item.source_start;
                return item.destination_start + placement_idx;
            }
        }
        value
    }
}

#[derive(Debug)]
struct MapperGroup {
    soil: Mapper,
    fertilizer: Mapper,
    water: Mapper,
    light: Mapper,
    temperature: Mapper,
    humidity: Mapper,
    location: Mapper,
}

impl MapperGroup {
    fn new(
        soil: Mapper,
        fertilizer: Mapper,
        water: Mapper,
        light: Mapper,
        temperature: Mapper,
        humidity: Mapper,
        location: Mapper,
    ) -> MapperGroup {
        MapperGroup {
            soil,
            fertilizer,
            water,
            light,
            temperature,
            humidity,
            location,
        }
    }
    fn get_location(&self, value: i64) -> i64 {
        self.location.get_destination(
            self.humidity.get_destination(
                self.temperature.get_destination(
                    self.light.get_destination(
                        self.water.get_destination(
                            self.fertilizer
                                .get_destination(self.soil.get_destination(value)),
                        ),
                    ),
                ),
            ),
        )
    }
}

fn parse_input(input: &str) -> (Vec<i64>, MapperGroup) {
    let mut soil: Mapper = Mapper::new();
    let mut fertilizer: Mapper = Mapper::new();
    let mut water: Mapper = Mapper::new();
    let mut light: Mapper = Mapper::new();
    let mut temperature: Mapper = Mapper::new();
    let mut humidity: Mapper = Mapper::new();
    let mut location: Mapper = Mapper::new();
    let mut current_mapper: &str = "";
    let mut seeds = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        match parts[0] {
            "seeds" => {
                seeds = parts[1]
                    .trim()
                    .split(' ')
                    .map(|num| num.parse::<i64>().unwrap_or(0))
                    .collect();
            }
            "seed-to-soil map" => {
                current_mapper = "soil";
            }
            "soil-to-fertilizer map" => {
                current_mapper = "fertilizer";
            }
            "fertilizer-to-water map" => {
                current_mapper = "water";
            }
            "water-to-light map" => {
                current_mapper = "light";
            }
            "light-to-temperature map" => {
                current_mapper = "temperature";
            }
            "temperature-to-humidity map" => {
                current_mapper = "humidity";
            }
            "humidity-to-location map" => {
                current_mapper = "location";
            }
            _ => {
                if !(parts[0].len() == 0) {
                    let group: Vec<&str> = parts[0].trim().split(' ').collect();
                    let (destination_start, source_start, range_length) = (
                        group[0].parse::<i64>().unwrap_or(0),
                        group[1].parse::<i64>().unwrap_or(0),
                        group[2].parse::<i64>().unwrap_or(0),
                    );
                    match current_mapper {
                        "soil" => {
                            soil.add(destination_start, source_start, range_length);
                        }
                        "fertilizer" => {
                            fertilizer.add(destination_start, source_start, range_length);
                        }
                        "water" => {
                            water.add(destination_start, source_start, range_length);
                        }
                        "light" => {
                            light.add(destination_start, source_start, range_length);
                        }
                        "temperature" => {
                            temperature.add(destination_start, source_start, range_length);
                        }
                        "humidity" => {
                            humidity.add(destination_start, source_start, range_length);
                        }
                        "location" => {
                            location.add(destination_start, source_start, range_length);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    let mapper_group = MapperGroup::new(
        soil,
        fertilizer,
        water,
        light,
        temperature,
        humidity,
        location,
    );
    (seeds, mapper_group)
}

fn process(input: &str) -> i64 {
    let (seed, group) = parse_input(input);
    // dbg!(&seed);
    // dbg!(&group);
    let mut lowest = i64::MAX;
    for seed_num in seed {
        let location = group.get_location(seed_num);
        // dbg!(location);
        if location < lowest {
            lowest = location
        }
    }
    lowest
}

fn part1(input: &str) -> i64 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, 35);
    }

    #[test]
    fn parse_works() {
        let result = parse_input(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result.0, vec![79, 14, 55, 13]);
    }

    #[test]
    fn mapper_works() {
        let seeds = [79, 14, 55, 13];
        let mut soil = Mapper::new();
        soil.add(50, 98, 2);
        soil.add(52, 50, 48);
        assert_eq!(soil.get_destination(seeds[0]), 81);
        assert_eq!(soil.get_destination(seeds[1]), 14);
        assert_eq!(soil.get_destination(seeds[2]), 57);
        assert_eq!(soil.get_destination(seeds[3]), 13);
    }
}
