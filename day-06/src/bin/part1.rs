fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    record_distance: u32,
}

impl Race {
    fn new(time: u32, record_distance: u32) -> Race {
        Race {
            time,
            record_distance,
        }
    }
    fn ways_to_beat_record(&self) -> impl Iterator<Item = u32> + '_ {
        // let mut wins: Vec<u32> = Vec::new();
        let wins = (1..self.time).filter_map(|hold_ms| {
            // let distance = hold_ms * (self.time - hold_ms);
            (hold_ms * (self.time - hold_ms) > self.record_distance).then_some(hold_ms)
        });
        // for i in 1..self.time as usize {
        //     let hold_ms = i as u32;
        //     let millisecons_to_move = self.time - hold_ms;
        //     let distance: u32 = hold_ms * millisecons_to_move;
        //     if distance > self.record_distance {
        //         wins.push(hold_ms);
        //     }
        // }
        wins
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut times: Vec<u32> = vec![];
    let mut record_distances: Vec<u32> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        match parts[0] {
            "Time" => {
                times = parts[1]
                    .trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|num| num.trim().parse::<u32>().unwrap_or(0))
                    .collect();
            }
            "Distance" => {
                record_distances = parts[1]
                    .trim()
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|num| num.trim().parse::<u32>().unwrap_or(0))
                    .collect();
            }
            _ => {}
        };
    }
    // dbg!(&times);
    // dbg!(&record_distances);
    if times.len() == record_distances.len() {
        for i in 0..times.len() {
            races.push(Race::new(times[i], record_distances[i]))
        }
    }
    races
}

fn process(input: &str) -> u32 {
    let mut value: u32 = 1;
    let races = parse_input(input);
    // dbg!(&races);
    for race in &races {
        let ways = race.ways_to_beat_record().count();
        // dbg!(ways);
        value *= ways as u32;
    }
    value
}

fn part1(input: &str) -> u32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
