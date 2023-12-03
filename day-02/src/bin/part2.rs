fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    format!("Hello, {}", input)
}

fn part2(input: &str) -> String {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Advent of Code!");
        assert_eq!(result, "Hello, Advent of Code!");
    }
}
