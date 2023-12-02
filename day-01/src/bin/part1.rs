fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn parse_to_int(input: &str) -> i32 {
    let value: i32 = input
        .lines()
        .map(|line| {
            let mut maybe_val_tuple: Option<(char, char)> = None;
            for b in line.chars() {
                if b.is_ascii_digit() {
                    maybe_val_tuple = match maybe_val_tuple {
                        Some(t) => Some((t.0, b)),
                        None => Some((b, b)),
                    };
                };
            }
            return match maybe_val_tuple {
                // Some(t) => t.0.to_digit(10).unwrap() * 10 + t.1.to_digit(10).unwrap(),
                Some(t) => format!("{}{}", t.0, t.1).parse::<i32>().unwrap_or(0),
                None => 0,
            };
        })
        .sum();
    dbg!(&value);

    value
}

fn part1(input: &str) -> i32 {
    parse_to_int(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "4nineeightseven2
xtwone3four",
        );
        assert_eq!(result, 75);
    }
}
