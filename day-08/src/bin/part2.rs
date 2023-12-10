use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

fn parse_input(
    input: &str,
) -> (
    Vec<Direction>,
    HashMap<usize, &str>,
    BTreeMap<usize, usize>,
    BTreeMap<usize, usize>,
    (Vec<usize>, Vec<usize>),
) {
    let mut start_keys = vec![];
    let mut end_keys = vec![];
    let directions: Vec<Direction> = input
        .lines()
        .into_iter()
        .next()
        .unwrap()
        .chars()
        .map(|d| match d {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("Should not be the case, bad directions line"),
        })
        .collect::<Vec<Direction>>();
    let mut left: BTreeMap<usize, usize> = BTreeMap::new();
    let mut right: BTreeMap<usize, usize> = BTreeMap::new();
    let mut mapping: HashMap<usize, &str> = HashMap::new();
    let mut key_mapping: HashMap<&str, usize> = HashMap::new();
    let mut idx: usize = 0;
    for line in input.lines() {
        if !line.is_empty() && line.split('=').count() == 2 {
            let key = &line[0..3];
            match key_mapping.get(key) {
                Some(_) => {}
                None => {
                    idx += 1;
                    key_mapping.insert(key, idx);
                    mapping.insert(idx, key);
                }
            }
        }
    }
    for line in input.lines() {
        if !line.is_empty() && line.split('=').count() == 2 {
            // example: FKX = (LSH, TSV)
            let key = &line[0..3];
            let key_left = &line[7..10];
            let key_right = &line[12..15];
            match key_mapping.get(key) {
                Some(idx) => {
                    if &line[2..3] == "A" {
                        start_keys.push(*idx);
                    } else if &line[2..3] == "Z" {
                        end_keys.push(*idx);
                    }
                    match key_mapping.get(key_left) {
                        Some(i) => {
                            left.insert(*idx, *i);
                        }
                        None => {
                            panic!("missing left mapped index");
                        }
                    }
                    match key_mapping.get(key_right) {
                        Some(i) => {
                            right.insert(*idx, *i);
                        }
                        None => {
                            panic!("missing right mapped index");
                        }
                    }
                }
                None => panic!("missing key"),
            }
        }
    }
    (directions, mapping, left, right, (start_keys, end_keys))
}

fn process(input: &str) -> u64 {
    let (directions, key_map, left, right, (start_keys, end_keys)) = parse_input(input);
    // dbg!(&left);
    // dbg!(&right);
    // dbg!(&start_keys);
    // dbg!(&end_keys);
    // dbg!(&key_map);
    let mut current_keys: Vec<usize> = start_keys.clone();
    let mut finished: bool = false;
    let mut count: u64 = 0;
    while !finished {
        for direction in &directions {
            let mut new_keys: Vec<usize> = Vec::new();
            current_keys.iter().for_each(|key| match *direction {
                Direction::Left => new_keys.push(*left.get(key).unwrap()),
                Direction::Right => new_keys.push(*right.get(key).unwrap()),
            });
            current_keys = new_keys.clone();
            finished = current_keys
                .iter()
                .filter(|item| end_keys.contains(item))
                .count()
                == end_keys.len();
            count += 1;
            // dbg!(&current_keys);
            if finished {
                break;
            }
        }
        if count % 10000 == 0 {
            dbg!(&count);
        }
    }
    dbg!(count);
    count
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
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn parse_works() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let (directions, key_map, left, right, (start_keys, end_keys)) = parse_input(input);
        assert_eq!(directions.len(), 2);
        assert_eq!(key_map.get(&(1 as usize)).unwrap(), &"11A");
        assert_eq!(key_map.get(&(2 as usize)).unwrap(), &"11B");
        assert_eq!(key_map.get(&(3 as usize)).unwrap(), &"11Z");
        assert_eq!(left.get(&(2 as usize)), Some(&8));
        assert_eq!(right.get(&(2 as usize)), Some(&3));
        assert_eq!(right.get(&(4 as usize)), Some(&8));
    }
}
