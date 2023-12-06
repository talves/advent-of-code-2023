fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u64,
    record_distance: u64,
}

impl Race {
    fn new(time: u64, record_distance: u64) -> Race {
        Race {
            time,
            record_distance,
        }
    }
    fn ways_to_beat_record(&self) -> Vec<u64> {
        let mut wins: Vec<u64> = Vec::new();
        for i in 1..self.time as usize {
            let hold_ms = i as u64;
            let millisecons_to_move = self.time - hold_ms;
            let distance: u64 = hold_ms * millisecons_to_move;
            if distance > self.record_distance {
                wins.push(hold_ms);
            }
        }
        wins
    }
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut races: Vec<Race> = vec![];
    let mut times: Vec<u64> = vec![];
    let mut record_distances: Vec<u64> = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        match parts[0] {
            "Time" => {
                match parts[1]
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                {
                    Ok(x) => times.push(x),
                    Err(e) => {
                        panic!("Could not parse time: {e}")
                    }
                };
            }
            "Distance" => {
                match parts[1]
                    .split_whitespace()
                    .collect::<String>()
                    .parse::<u64>()
                {
                    Ok(x) => record_distances.push(x),
                    Err(e) => {
                        panic!("Could not parse distance: {e}")
                    }
                };
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

fn process(input: &str) -> u64 {
    let mut value: u64 = 1;
    let races = parse_input(input);
    // dbg!(&races);
    for race in &races {
        let ways = race.ways_to_beat_record().len();
        // dbg!(ways);
        value *= ways as u64;
    }
    value
}

fn part2(input: &str) -> u64 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }
}
