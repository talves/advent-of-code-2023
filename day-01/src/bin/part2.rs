fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

static DIGITS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
static INDEXES: [&'static char; 9] = [&'1', &'2', &'3', &'4', &'5', &'6', &'7', &'8', &'9'];

fn parse_to_int(input: &str) -> i32 {
    let value: i32 = input
        .lines()
        .map(|line| {
            let mut partial: String = "".to_string();
            let mut maybe_val_tuple: Option<(char, char)> = None;
            for b in line.chars() {
                if b.is_ascii_digit() {
                    maybe_val_tuple = match maybe_val_tuple {
                        Some(t) => Some((t.0, b)),
                        None => Some((b, b)),
                    };
                    partial = "".to_string();
                } else {
                    partial = format!("{}{}", partial, b.to_string());
                    for (i, s) in DIGITS.iter().enumerate() {
                        match partial.find(s) {
                            Some(_) => {
                                maybe_val_tuple = match maybe_val_tuple {
                                    Some(t) => Some((t.0, *INDEXES[i])),
                                    None => Some((*INDEXES[i], *INDEXES[i])),
                                };
                                // Covers the case when we have a last letter being used as the first letter of the next digit
                                partial = b.to_string();
                                break;
                            }
                            None => {}
                        };
                    }
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

fn part2(input: &str) -> i32 {
    parse_to_int(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2_works() {
        let result = part2(
            "two1nine
eightwo2three
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
