fn main() {
    let input = include_str!("../input.txt");
    let output = part2(input);
    dbg!(output);
}

struct Card {
    winning_numbers: Vec<u32>,
}

impl Card {
    fn new(input: &str) -> Card {
        Card {
            winning_numbers: input
                .trim()
                .split(' ')
                .map(|num| num.parse::<u32>().unwrap_or(0))
                .filter(|num| num > &0)
                .collect(),
        }
    }
    fn get_points_from_str(&self, input: &str) -> u32 {
        let mut points: Option<u32> = None;

        let nums = input
            .trim()
            .split(' ')
            .map(|num| num.parse::<u32>().unwrap_or(0))
            .filter(|num| num > &0);
        for num in nums {
            for winner in self.winning_numbers.iter() {
                if num == *winner {
                    match points {
                        Some(val) => points = Some(val * 2),
                        None => points = Some(1),
                    }
                    break;
                }
            }
        }
        points.unwrap_or(0)
    }
}

fn process(input: &str) -> u32 {
    let points = input.lines().enumerate().map(|(_id, line)| {
        let parts = line.split(':').collect::<Vec<&str>>()[1]
            .split("|")
            .collect::<Vec<&str>>();
        let card_winners = parts[0];
        let card_input = parts[1];
        let card = Card::new(card_winners);
        card.get_points_from_str(card_input)
    });
    dbg!(&points.clone().collect::<Vec<u32>>());
    points.sum()
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }

    #[test]
    fn part2_new_card() {
        let card = Card::new("41 48 83 86 17");
        assert_eq!(card.winning_numbers, vec![41, 48, 83, 86, 17]);
    }

    #[test]
    fn part2_points() {
        let card = Card::new("41 48 83 86 17");
        assert_eq!(card.get_points_from_str("83 86  6 31 17  9 48 53"), 8);
    }
}
