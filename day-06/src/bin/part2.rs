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
    fn ways_to_beat_record(&self) -> impl Iterator<Item = u64> + '_ {
        return (1..self.time).filter_map(|hold_ms| {
            (hold_ms * (self.time - hold_ms) > self.record_distance).then_some(hold_ms)
        });
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = Race> + '_ {
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
    (times.len() == record_distances.len())
        .then(move || {
            (0..times.len()).map(move |i| Race::new(times[i], record_distances[i]))
            // .collect::<Vec<Race>>()
        })
        .expect("input should have matching times and distances")
}

fn process(input: &str) -> usize {
    parse_input(input)
        .map(|race| race.ways_to_beat_record().count())
        .product::<usize>()
}

fn part2(input: &str) -> usize {
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
