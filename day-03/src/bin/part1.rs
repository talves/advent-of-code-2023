use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Location {
    line: usize,
    start: usize,
    len: usize,
    value: i32,
}
impl Location {
    fn new(line: usize, start: usize, len: usize, value: i32) -> Location {
        Location {
            line,
            start,
            len,
            value,
        }
    }
}

fn process(input: &str) -> i32 {
    let mut parts: Vec<Location> = vec![];
    let mut symbols: HashMap<(usize, usize), char> = HashMap::default();
    for (line_idx, line) in input.lines().enumerate() {
        let mut maybe_val: Option<String> = None;
        for (i, b) in line.chars().enumerate() {
            let mut try_store_location = false;
            if b.is_ascii_digit() {
                maybe_val = match maybe_val {
                    Some(t) => Some(format!("{t}{b}")),
                    None => Some(format!("{b}")),
                };
                if i == line.len() - 1 {
                    // At end of the line and we have a part
                    try_store_location = true;
                }
            } else {
                // store symbol location as hashmap for later lookup, ignore '.'
                if b != '.' {
                    symbols.insert((line_idx, i), b);
                }
                // Flag to check for storing the location if we have a value
                try_store_location = true;
            };
            if try_store_location {
                if let Some(ref value) = maybe_val {
                    parts.push(Location::new(
                        line_idx,
                        i - value.len(),
                        value.len(),
                        value.parse::<i32>().unwrap_or(0),
                    ));
                    maybe_val = None;
                }
            }
        }
    }
    // Iterate the parts and make sure they have an ajacent symbol, if so collect and sum
    // dbg!(&parts);
    // dbg!(&symbols);
    return parts
        .iter()
        .map(|part| {
            let mut value = 0;
            for x in part.start.checked_sub(1).unwrap_or(0)..=(part.start + part.len) {
                // dbg!(&x);
                if part.line > 0 {
                    if let Some(_c) = symbols.get(&(part.line - 1, x)) {
                        value = part.value;
                        break;
                    }
                }
                if let Some(_c) = symbols.get(&(part.line, x)) {
                    value = part.value;
                    break;
                }
                if let Some(_c) = symbols.get(&(part.line + 1, x)) {
                    value = part.value;
                    break;
                }
            }
            value
        })
        .sum();
}

fn part1(input: &str) -> i32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "467..114..
...*......
..35...633
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
