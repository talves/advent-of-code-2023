use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>) {
    let mut lines = input.lines().into_iter();
    let directions: Vec<Direction> = lines
        .next()
        .unwrap()
        .chars()
        .map(|d| match d {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Should not be the case, bad directions line"),
        })
        .collect::<Vec<Direction>>();
    let mut mapping: HashMap<&str, (&str, &str)> = HashMap::new();
    // let mut start: Option<&str> = None;
    // let mut end: Option<&str> = None;
    for line in lines {
        if !line.is_empty() {
            // example: FKX = (LSH, TSV)
            let key = &line[0..3];
            // match start {
            //     Some(_) => {
            //         end = Some(&key);
            //     }
            //     None => {
            //         start = Some(&key);
            //         end = Some(&key);
            //     }
            // }
            let left = &line[7..10];
            let right = &line[12..15];
            mapping.insert(key, (left, right));
        }
    }
    (directions, mapping)
}

fn process(input: &str) -> u32 {
    let (directions, paths) = parse_input(input);
    let start = "AAA";
    let end = "ZZZ";
    // dbg!(&paths);
    let mut current_key = start;
    let mut count = 0;
    while current_key != end {
        for direction in &directions {
            match paths.get(current_key) {
                Some((left, right)) => match direction {
                    Direction::Left => current_key = left,
                    Direction::Right => current_key = right,
                },
                None => panic!("Paths found unmapped key {}", current_key),
            }
            count += 1;
            if current_key == end {
                break;
            }
        }
        dbg!(&current_key);
    }
    count
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
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
",
        );
        assert_eq!(result, 2);
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
",
        );
        assert_eq!(result, 6);
    }
}
