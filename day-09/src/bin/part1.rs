use std::borrow::BorrowMut;

fn main() {
    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Clone)]
struct History {
    items: Vec<i32>,
}

impl History {
    fn new() -> History {
        History { items: Vec::new() }
    }
    fn from_str(&mut self, line: &str) {
        self.items = vec![];
        line.split(' ').for_each(|item| {
            self.items.push(item.trim().parse::<i32>().unwrap());
        });
    }
    fn extrapolate(&self) -> i32 {
        fn get_prediction(data: &Vec<i32>) -> i32 {
            let mut diffs = Vec::new();
            let last_len = data.len() - 1;
            for i in 0..last_len {
                diffs.push(data[i + 1] - data[i]);
            }
            if diffs[last_len - 1] == 0 {
                return 0;
            } else {
                diffs[last_len - 1] + get_prediction(&diffs)
            }
        }
        &self.items[&self.items.len() - 1] + get_prediction(&self.items)
    }
}

fn parse_input(input: &str) -> Vec<History> {
    input
        .lines()
        .map(|line| {
            let mut history = History::new();
            history.from_str(line);
            history
        })
        .collect::<Vec<History>>()
}

fn process(input: &str) -> i32 {
    let all_history = parse_input(input);
    all_history
        .iter()
        .map(|history| history.extrapolate())
        .sum()
}

fn part1(input: &str) -> i32 {
    process(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, 114);
    }

    #[test]
    fn history_works() {
        let mut history = History::new();
        history.from_str("0 3 6 9 12 15");
        assert_eq!(history.items, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn parse_works() {
        let result = parse_input(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].items, vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(result[1].items, vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(result[2].items, vec![10, 13, 16, 21, 30, 45]);
    }

    #[test]
    fn extrapolate_works() {
        let mut history = History::new();
        history.from_str("0 3 6 9 12 15");
        assert_eq!(history.extrapolate(), 18);
        let mut history = History::new();
        history.from_str("1 3 6 10 15 21");
        assert_eq!(history.extrapolate(), 28);
        let mut history = History::new();
        history.from_str("10 13 16 21 30 45");
        assert_eq!(history.extrapolate(), 68);
    }
}
