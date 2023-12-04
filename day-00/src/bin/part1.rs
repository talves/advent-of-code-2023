fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    format!("Hello, {}", input)
}

fn part1(input: &str) -> String {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1("Advent of Code!");
        assert_eq!(result, "Hello, Advent of Code!");
    }
}
