use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> (Vec<Direction>, HashMap<&str, (&str, &str)>, Vec<&str>) {
    let mut lines = input.lines().into_iter();
    let mut start_keys = vec![];
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
    for line in lines {
        if !line.is_empty() {
            // example: FKX = (LSH, TSV)
            let key = &line[0..3];
            if &key[2..3] == "A" {
                start_keys.push(key);
            }
            let left = &line[7..10];
            let right = &line[12..15];
            mapping.insert(key, (left, right));
        }
    }
    (directions, mapping, start_keys)
}

fn process(input: &str) -> u32 {
    let (directions, paths, start_keys) = parse_input(input);
    // dbg!(&start_keys);
    // dbg!(&paths);
    let mut found_z_end = false;
    let mut current_keys: Vec<&str> = vec![""; start_keys.len()];
    start_keys
        .iter()
        .enumerate()
        .for_each(|(i, x)| current_keys[i] = *x);
    let mut count = 0;
    while !found_z_end {
        for direction in &directions {
            let mut new_keys: Vec<&str> = vec![""; start_keys.len()];
            current_keys
                .iter()
                .enumerate()
                .for_each(|(i, current_key)| match paths.get(*current_key) {
                    Some((left, right)) => match direction {
                        Direction::Left => new_keys[i] = *left,
                        Direction::Right => new_keys[i] = *right,
                    },
                    None => panic!("Paths found unmapped key {}", current_key),
                });
            count += 1;
            found_z_end = true;
            for (i, current_key) in new_keys.iter().enumerate() {
                current_keys[i] = *current_key;
                if &current_key[2..3] != "Z" {
                    found_z_end = false;
                }
            }
            if found_z_end {
                break;
            }
            // dbg!(&current_keys);
        }
    }
    count
}

fn part2(input: &str) -> u32 {
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
}
